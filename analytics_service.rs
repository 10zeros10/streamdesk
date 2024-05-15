use anyhow::{Context, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
struct Viewer {
    id: String,
    age: u8,
    location: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    id: String,
    title: String,
    views: u32,
}

#[derive(Default, Debug)]
struct Analytics {
    viewer_demographics: HashMap<String, u32>,
    content_performance: HashMap<String, u32>,
    engagement_stats: HashMap<String, f32>,
}

impl Analytics {
    fn new() -> Self {
        Self::default()
    }

    fn update_with_stream_data(&mut self, viewer: &Viewer, content: &Content) {
        *self.viewer_demographics.entry(viewer.location.clone()).or_insert(0) += 1;
        *self.content_performance.entry(content.title.clone()).or_insert(0) += content.views;
    }

    fn calculate_engagement(&mut self) {
        for (content, views) in &self.content_performance {
            let engagement = (*views as f32 / 1000.0) * 100.0;
            self.engagement_stats.insert(content.clone(), engagement);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().context("Failed to load .env file")?;
    env_logger::init().context("Failed to initialize logger")?;

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        let stream_data = vec![
            (
                "{\"id\": \"1\", \"age\": 25, \"location\": \"USA\"}",
                "{\"id\": \"1\", \"title\": \"Rust Basics\", \"views\": 150}",
            ),
        ];

        for data in stream_data {
            match serde_json::from_str::<Viewer>(data.0) {
                Ok(viewer) => match serde_json::from_str::<Content>(data.1) {
                    Ok(content) => {
                        if let Err(e) = tx.send((viewer, content)).await {
                            error!("Error sending data through channel: {}", e);
                        }
                    }
                    Err(e) => error!("Error deserializing content: {}", e),
                },
                Err(e) => error!("Error deserializing viewer: {}", e),
            }
        }
    });

    let mut analytics = Analytics::new();

    while let Some((viewer, content)) = rx.recv().await {
        analytics.update_with_stream_data(&viewer, &content);
    }

    analytics.calculate_engagement();

    info!("{:?}", analytics);

    Ok(())
}
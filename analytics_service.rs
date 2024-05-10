use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use serde_json::Value;

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
        *self.viewer_demographics.entry(viewer.location.clone()).or_insert(0) += 1; // Clone is necessary due to ownership rules
        *self.content_performance.entry(content.title.clone()).or_insert(0) += content.views; // Same here
    }

    fn calculate_engagement(&mut self) {
        for (content, views) in &self.content_performance {
            let engagement = (*views as f32 / 1000.0) * 100.0; 
            self.engagement_stats.insert(content.clone(), engagement); // Clone is necessary here due to how borrowing works in Rust
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); 
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        let stream_data = vec![
            ("{\"id\": \"1\", \"age\": 25, \"location\": \"USA\"}", "{\"id\": \"1\", \"title\": \"Rust Basics\", \"views\": 150}"),
        ];

        // Iterating directly on the data, no pre-collection involved.
        for data in stream_data {
            let viewer: Viewer = serde_json::from_str(data.0).unwrap();
            let content: Content = serde_json::from_str(data.1).unwrap();
            tx.send((viewer, content)).await.unwrap();
        }
    });

    let mut analytics = Analytics::new();

    // Processing data as it arrives, without collecting it.
    while let Some((viewer, content)) = rx.recv().await {
        analytics.update_with_stream_data(&viewer, &content);
    }

    analytics.calculate_engagement();

    println!("{:?}", analytics);
}
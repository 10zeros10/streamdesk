use std::collections::HashMap;
use std::env;

struct StreamData {
    data: String,
}

struct StreamFilter {
    keyword: String,
}

struct LiveStreamManager {
    input_streams: Vec<StreamData>,
    filters: Vec<StreamFilter>,
    outputs: HashMap<String, Vec<StreamData>>,
}

impl LiveStreamManager {
    fn new() -> Self {
        let manager = LiveStreamManager {
            input_streams: Vec::new(),
            filters: Vec::new(),
            outputs: HashMap::new(),
        };
        manager.log("LiveStreamManager created");
        manager
    }

    fn add_stream_data(&mut self, data: StreamData) {
        self.log(&format!("Adding stream data: {}", data.data));
        self.input_streams.push(data);
    }

    fn add_filter(&mut self, filter: StreamFilter) {
        self.log(&format!("Adding filter: {}", filter.keyword));
        self.filters.push(filter);
    }

    fn apply_filters(&mut self) {
        for filter in &self.filters {
            self.input_streams.retain(|data| data.data.contains(&filter.keyword));
            self.log(&format!("Applied filter: {}", filter.keyword));
        }
    }

    fn distribute_to_outputs(&mut self) {
        let platforms_str = env::var("OUTPUT_PLATFORMS").unwrap_or_else(|_| {
            self.log("OUTPUT_PLATFORMS not set, using default.");
            "default".into()
        });

        let platforms = platforms_str.split(',');
        for platform in platforms {
            let platform_data = self.input_streams.clone();
            self.outputs.insert(platform.to_string(), platform_data);
            self.log(&format!("Distributed streams to output: {}", platform));
        }
    }

    pub fn process_stream(&mut self) {
        self.log("Starting stream processing");
        self.apply_filters();
        self.distribute_to_outputs();
        self.log("Stream processing completed");
    }

    fn log(&self, message: &str) {
        println!("[LiveStreamManager LOG]: {}", message);
    }
}

fn main() {
    match env::var("OUTPUT_PLATFORMS") {
        Ok(value) => println!("Using OUTPUT_PLATFORMS: {}", value),
        Err(e) => eprintln!("Couldn't read OUTPUT_PLATFORMS (using default): {}", e),
    }

    let mut manager = LiveStreamManager::new();

    manager.add_stream_data(StreamData { data: "First live stream data".to_string() });
    manager.add_stream_data(StreamData { data: "Second stream with keyword".to_string() });

    manager.add_filter(StreamFilter { keyword: "keyword".to_string() });

    manager.process_stream();

    for (platform, stream_data) in manager.outputs.iter() {
        println!("Platform: {}", platform);
        for data in stream_data {
            println!("Data: {}", data.data);
        }
    }
}
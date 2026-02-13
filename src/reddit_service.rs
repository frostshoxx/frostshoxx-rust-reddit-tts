use reqwest::Client;
use serde_json::Value;
use crate::tts_service::TtsService;

pub struct RedditService {
    client: Client,
    tts: TtsService,
}

impl RedditService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            tts: TtsService::new(),
        }
    }

    pub async fn fetch_and_speak_top_threads(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch top posts from Reddit
        let url = "https://www.reddit.com/r/popular/top.json?limit=10";
        
        let response = self.client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .send()
            .await?;

        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        // Extract top 10 thread titles
        let posts = &json["data"]["children"];
        let mut titles = Vec::new();

        for post in posts.as_array().unwrap_or(&vec![]) {
            if let Some(title) = post["data"]["title"].as_str() {
                titles.push(title.to_string());
            }
        }

        println!("Found {} thread titles. Starting text-to-speech...\n", titles.len());

        // Speak introductory message
        let intro = format!("Hello! Here are the top {} threads from Reddit.", titles.len());
        self.tts.speak_text(&intro)?;
        
        // Brief pause before start reading the threads
        self.tts.pause_between(1000);

        // Read each title via TTS
        for (index, title) in titles.iter().enumerate() {
            println!("[{}/{}] Speaking: {}", index + 1, titles.len(), title);
            let speech = format!("Thread {}: {}", index + 1, title);
            self.tts.speak_text(&speech)?;
            
            // Brief pause between titles
            self.tts.pause_between(1000);
        }

        println!("\nFinished reading all threads!");

        // Ending message
        let ending = format!("That's all for now. You have heard the top {} threads from Reddit. Goodbye!", titles.len());
        self.tts.speak_text(&ending)?;

        Ok(())
    }
}

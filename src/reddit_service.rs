use reqwest::Client;
use serde_json::Value;
use tts::Tts;
use std::thread;
use std::time::Duration;

pub struct RedditService {
    client: Client,
}

impl RedditService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
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

        println!("📖 Found {} thread titles. Starting text-to-speech...\n", titles.len());

        // Initialize TTS
        let mut tts = Tts::default()?;
        
        // Speak introductory message
        let intro = format!("Hello! Here are the top {} threads from Reddit.", titles.len());
        tts.speak(intro, true)?;
        // Wait until the engine finishes speaking 
        while tts.is_speaking()? { thread::sleep(Duration::from_millis(100));}
        // brief pause before start reading the threads
        thread::sleep(Duration::from_millis(1000));

        // Read each title via TTS
        for (index, title) in titles.iter().enumerate() {
            println!("[{}/{}] Speaking: {}", index + 1, titles.len(), title);
            let speech = format!("Thread {}: {}", index + 1, title);
            tts.speak(speech, true)?;
            // Wait until the engine finishes speaking 
            while tts.is_speaking()? { thread::sleep(Duration::from_millis(100));}
            // brief pause between titles
            thread::sleep(Duration::from_millis(1000));
        }

        println!("\n✅ Finished reading all threads!");

        // Ending message
        let ending = format!("That's all for now. You have heard the top {} threads from Reddit. 
            Goodbye!", titles.len());
        tts.speak(ending, true)?;
        // Wait until the engine finishes speaking 
        while tts.is_speaking()? { thread::sleep(Duration::from_millis(100));}

        // Clean up TTS resources
        tts.stop()?;
        Ok(())
    }
}

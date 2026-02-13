use reqwest::Client;
use serde_json::Value;
use crate::tts_service::TtsService;
use crate::dto::{RedditPost, RedditPostsDTO};

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

        // Extract top 10 thread titles into DTO
        let posts = &json["data"]["children"];
        let mut posts_dto = RedditPostsDTO::new();

        for post in posts.as_array().unwrap_or(&vec![]) {
            let data = &post["data"];
            let reddit_post = RedditPost {
                id: data["id"].as_str().unwrap_or("").to_string(),
                title: data["title"].as_str().unwrap_or("").to_string(),
                author: data["author"].as_str().unwrap_or("[deleted]").to_string(),
                score: data["score"].as_i64().unwrap_or(0) as i32,
                upvotes: data["ups"].as_i64().unwrap_or(0) as i32,
                downvotes: data["downs"].as_i64().unwrap_or(0) as i32,
                num_comments: data["num_comments"].as_i64().unwrap_or(0) as i32,
                subreddit: data["subreddit"].as_str().unwrap_or("").to_string(),
                url: data["url"].as_str().unwrap_or("").to_string(),
            };
            posts_dto.add_post(reddit_post);
        }

        println!("Found {} thread titles. Starting text-to-speech...\n", posts_dto.count());

        // Speak introductory message
        let intro = format!("Hello! Here are the top {} threads from Reddit.", posts_dto.count());
        self.tts.speak_text(&intro)?;
        
        // Brief pause before start reading the threads
        self.tts.pause_between(1000);

        // Read each title via TTS
        for (index, post) in posts_dto.posts.iter().enumerate() {
            println!("[{}/{}] Speaking: {}", index + 1, posts_dto.count(), post.title);
            let speech = format!("Thread {}: {}. Posted by {}.", 
                index + 1, post.title, post.author,);
            self.tts.speak_text(&speech)?;
            
            // Brief pause between titles
            self.tts.pause_between(1000);
        }

        println!("\nFinished reading all threads!");

        // Ending message
        let ending = format!("That's all for now. You have heard the top {} threads from Reddit. Goodbye!", posts_dto.count());
        self.tts.speak_text(&ending)?;

        Ok(())
    }
}

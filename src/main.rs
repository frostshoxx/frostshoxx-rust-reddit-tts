mod dto;
mod reddit_service;
mod tts_service;

use reddit_service::RedditService;
use tts_service::TtsService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reddit_service = RedditService::new();
    let tts_service = TtsService::new();
    
    // Fetch posts from Reddit
    let posts = reddit_service.fetch_top_threads().await?;
    println!("Found {} thread titles. Starting text-to-speech...\n", posts.count());

    // Speak introductory message
    let intro = format!("Hello! Here are the top {} threads from Reddit.", posts.count());
    tts_service.speak_text(&intro)?;
    
    // Brief pause before start reading the threads
    tts_service.pause_between(1000);

    // Read each title via TTS
    for (index, post) in posts.posts.iter().enumerate() {
        println!("[{}/{}] Speaking: {}", index + 1, posts.count(), post.title);
        let speech = format!("Thread {}: {}. Posted by {}.", 
            index + 1, post.title, post.author);
        tts_service.speak_text(&speech)?;
        
        // Brief pause between titles
        tts_service.pause_between(1000);
    }

    println!("\nFinished reading all threads!");

    // Ending message
    let ending = format!("That's all for now. You have heard the top {} threads from Reddit. Goodbye!", posts.count());
    tts_service.speak_text(&ending)?;

    Ok(())
}

mod reddit_service;

use reddit_service::RedditService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let service = RedditService::new();
    service.fetch_and_speak_top_threads().await?;
    
    Ok(())
}

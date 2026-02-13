use serde::{Deserialize, Serialize};

/// Represents a single Reddit post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub id: String,
    pub title: String,
    pub author: String,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    pub num_comments: i32,
    pub subreddit: String,
    pub url: String,
}

/// DTO containing a collection of Reddit posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPostsDTO {
    pub posts: Vec<RedditPost>,
}

impl RedditPostsDTO {
    /// Create a new empty DTO
    pub fn new() -> Self {
        Self {
            posts: Vec::new(),
        }
    }

    /// Add a post to the collection
    pub fn add_post(&mut self, post: RedditPost) {
        self.posts.push(post);
    }

    /// Get the number of posts in the collection
    pub fn count(&self) -> usize {
        self.posts.len()
    }

    /// Get a post by index
    pub fn get_post(&self, index: usize) -> Option<&RedditPost> {
        self.posts.get(index)
    }

    /// Get all titles as a Vec<String>
    pub fn get_titles(&self) -> Vec<String> {
        self.posts.iter().map(|p| p.title.clone()).collect()
    }
}

impl Default for RedditPostsDTO {
    fn default() -> Self {
        Self::new()
    }
}

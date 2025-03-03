//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```rust
//! use roux::User;
//! use roux::util::FeedOption;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let user = User::new("kasuporo");
//!     // Now you are able to:
//!
//!     // Get overview
//!     let overview = user.overview(None).await;
//!
//!     // Get submitted posts.
//!     let submitted = user.submitted(None).await;
//!
//!     // Get comments.
//!     let comments = user.comments(None).await;
//! }
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::{FeedOption, RouxError};
use reqwest::Client;

pub mod responses;
use crate::subreddit::responses::{Submissions, SubredditComments};
use responses::{About, Overview};

/// User.
pub struct User {
    /// User's name.
    pub user: String,
    client: Client,
}

impl User {
    /// Create a new `User` instance.
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            client: Client::new(),
        }
    }

    /// Get user's overview.
    pub async fn overview(&self, options: Option<FeedOption>) -> Result<Overview, RouxError> {
        let url = &mut format!("https://www.reddit.com/user/{}/overview/.json", self.user);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self, options: Option<FeedOption>) -> Result<Submissions, RouxError> {
        let url = &mut format!("https://www.reddit.com/user/{}/submitted/.json", self.user);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<Submissions>()
            .await?)
    }

    /// Get user's submitted comments.
    pub async fn comments(
        &self,
        options: Option<FeedOption>,
    ) -> Result<SubredditComments, RouxError> {
        let url = &mut format!("https://www.reddit.com/user/{}/comments/.json", self.user);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<SubredditComments>()
            .await?)
    }

    /// Get user's about page
    pub async fn about(&self, options: Option<FeedOption>) -> Result<About, RouxError> {
        let url = &mut format!("https://www.reddit.com/user/{}/about/.json", self.user);
        println!("{}", self.user);
        if let Some(options) = options {
            options.build_url(url);
        }
        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<About>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::util::FeedOption;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let user = User::new("beneater");

        // Test overview
        let overview = user.overview(None).await;
        assert!(overview.is_ok());

        // Test submitted
        let submitted = user.submitted(None).await;
        assert!(submitted.is_ok());

        // Test comments
        let comments = user.comments(None).await;
        assert!(comments.is_ok());

        // Test about
        let about = user.about(None).await;
        assert!(about.is_ok());

        // Test feed options
        let after = comments.unwrap().data.after.unwrap();
        let after_options = FeedOption::new().after(&after);
        let next_comments = user.comments(Some(after_options)).await;
        assert!(next_comments.is_ok());
    }
}

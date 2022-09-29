pub mod twitter {
    use std::env;
    use egg_mode::{ auth::Token, user::UserID, tweet::Tweet };
    use crate::database::database as db;

    struct Twitter {
        token: Token,
    }

    impl Twitter {
        /// Create a new Twitter instance
        ///
        /// Twitter Token from the environment variable TWITTER_TOKEN
        pub fn new() -> Self {
            let token = env::var("TWITTER_TOKEN").unwrap();
            Self {
                token: Token::Bearer(token)
            }
        }

        /// Get the latest tweet with media from a user
        pub async fn get_tweets_with_media(&self, user_id: UserID) -> Vec<Tweet>{
            let mut tweets_with_media = Vec::new();
            let tweets = self.get_user_timeline(user_id).await;
            for tweet in tweets {
                match tweet.extended_entities {
                    Some(ref entities) => {
                        if entities.media.len() > 0 {
                            tweets_with_media.push(tweet);
                        }
                    },
                    None => continue,
                }
            }
            tweets_with_media
        }

        /// Transfer Twitter Username to ID
        pub async fn username_to_id(&self, username: String) -> u64 {
            egg_mode::user::show(username, &self.token)
                .await
                .unwrap()
                .id
        }

        /// Get the latest tweet from a user
        ///
        /// Page Size from the environment variable PAGE_SIZE
        async fn get_user_timeline(&self, user_id: UserID) -> Vec<Tweet> {
            let page_size = env::var("TWITTER_PAGE_SIZE")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let timeline = egg_mode::tweet::user_timeline(user_id, false, false, &self.token)
                .with_page_size(page_size);
            let (_timeline, feed) = timeline.start().await.unwrap();
            feed.response
        }
    }

    #[cfg(test)]
    mod tests {
        #[tokio::test]
        async fn username_to_id_test() {
            use super::*;
            let twitter = Twitter::new();
            let id = twitter.username_to_id("avogado6".to_string()).await;
            assert_eq!(id, 324391547);
        }

        #[tokio::test]
        async fn get_user_timeline_test() {
            use super::*;
            let twitter = Twitter::new();
            let user_id = UserID::ID(324391547);
            let timeline = twitter.get_user_timeline(user_id).await;
            assert_eq!(timeline.len(), 5);
        }
    }
}

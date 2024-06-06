use log::{debug, info};
use crate::collector::EcstasyCollector;
use crate::error::EcstasyError;
use crate::item::EcstasyItem;
use crate::params::EcstasyFilter;

#[derive(Clone, Debug, Default)]
pub struct RedditCollector;

impl RedditCollector {
    pub fn new() -> Self { Self::default() }

    pub fn boxed() -> Box<dyn EcstasyCollector> { Box::new(Self::new()) }
}

impl EcstasyCollector for RedditCollector {
    fn id(&self) -> &'static str { "reddit" }

    fn name(&self) -> &'static str { "Reddit" }

    fn api_base(&self) -> &'static str { "https://reddit.com" }

    fn site_base(&self) -> &'static str { "https://reddit.com" }

    fn tags_argument(&self) -> &'static str {
        todo!()
    }

    fn page_argument(&self) -> &'static str {
        todo!()
    }

    fn api_by_page(&self, _subreddit: String, _page: u64) -> String {
        todo!()
    }

    fn collect(&self, filter: EcstasyFilter) -> Result<Vec<EcstasyItem>, EcstasyError> {
        info!("Starting {} collector...", &self.name());
        let mut items = Vec::new();
        for i in filter.subreddits {
            info!("Getting subreddit {}", i);
            debug!("Grabbing page with Reqwest GET...");
            let mut resp = reqwest::get(
                &format!(
                    "{}/r/{}/hot.json",
                    &self.api_base(),
                    i
                )
            )?;
            debug!("Decoding response...");
            let body = resp.text()?;
            println!("{}", body);
        }

        Ok(items)
    }
}

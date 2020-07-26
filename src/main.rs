use dotenv::dotenv;
use std::env;

mod bot;
mod scraper;
use crate::scraper::scraper::Scraper;
use bot::bot::Bot;

#[tokio::main]
async fn main() {
  dotenv().ok();
  env_logger::init();

  let rust_weekly_scrapper = Scraper::new("https://this-week-in-rust.org/");
  // let urls = rust_weekly_scrapper.scrap_weekly_rust();
  // println!("{}", urls.await);

  let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
  let use_webhook = env::var("USE_WEBHOOK").ok();
  let rust_weekly_tgbot = Bot::new(token, rust_weekly_scrapper);
  if let Some(use_webhook) = use_webhook {
    if use_webhook == "true" {
      println!("Not Implemented");
      return;
    }
  }
  rust_weekly_tgbot.start_long_poll().await;
}

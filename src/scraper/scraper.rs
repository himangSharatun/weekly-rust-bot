use reqwest;
use scraper::{Html, Selector};

#[derive(Clone)]
pub struct RustWeeklyScraper {
  base_url: String,
}

impl RustWeeklyScraper {
  pub fn new(base_url: String) -> Self {
    Self { base_url }
  }

  pub fn scrap_weekly_rust(&self) -> String {
    let current_issue_url = self.get_weekly_rust_url();
    let news_and_blogs_url = self.get_news_and_blogs_urls(current_issue_url);
    news_and_blogs_url
  }

  fn get_weekly_rust_url(&self) -> String {
    let html_text = self.get_html_text(self.base_url.clone()).unwrap();
    let html_doc = Html::parse_document(&html_text);
    let past_issues_url = Selector::parse("div.row.post-title div a").unwrap();
    let result = html_doc.select(&past_issues_url).next().unwrap();

    let result = match result.value().attr("href") {
      None => panic!("url not found"),
      Some(url) => String::from(url),
    };
    result
  }

  fn get_news_and_blogs_urls(&self, url: String) -> String {
    let html_text = self.get_html_text(url).unwrap();
    let html_doc = Html::parse_document(&html_text);
    let news_blog_post = Selector::parse("h2#news-blog-posts + ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let ul = html_doc.select(&news_blog_post).next().unwrap();
    let mut urls = String::new();
    for element in ul.select(&li_selector) {
      urls = format!("{}{}\n", urls, element.inner_html());
    }
    urls
  }

  fn get_html_text(&self, url: String) -> Result<String, reqwest::Error> {
    let resp = reqwest::blocking::get(&url)?;
    assert!(resp.status().is_success());
    let result = resp.text();
    result
  }
}

use reqwest;
use scraper::{Html, Selector};

#[derive(Clone)]
pub struct Scraper {
  base_url: String,
}

impl Scraper {
  pub fn new(base_url: &str) -> Self {
    let url_string = String::from(base_url);
    Self {
      base_url: url_string,
    }
  }

  pub async fn scrap_weekly_rust(&self) -> String {
    let current_issue_url = self.get_weekly_rust_url().await;
    let news_and_blogs_url = self.get_news_and_blogs_urls(&current_issue_url).await;
    news_and_blogs_url
  }

  async fn get_weekly_rust_url(&self) -> String {
    let html_text = self.get_html_text(&self.base_url).await.unwrap();
    let html_doc = Html::parse_document(&html_text);
    let past_issues_url = Selector::parse("div.row.post-title div a").unwrap();
    let result = html_doc.select(&past_issues_url).next().unwrap();

    let result = match result.value().attr("href") {
      None => panic!("url not found"),
      Some(url) => String::from(url),
    };
    result
  }

  async fn get_news_and_blogs_urls(&self, url: &str) -> String {
    let html_text = self.get_html_text(url).await.unwrap();
    let html_doc = Html::parse_document(&html_text);
    let news_and_blogs_url = self.scrap_news_and_blogs_url(&html_doc);
    let title_link = self.scrap_title_link(&html_doc);
    let date = self.scrap_date(&html_doc);
    let result = format!("{}{}\n{}", title_link, date, news_and_blogs_url);
    result
  }

  async fn get_html_text(&self, url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let result = resp.text().await;
    result
  }

  fn scrap_news_and_blogs_url(&self, html_doc: &Html) -> String {
    let post_css_selector = "h2#news-blog-posts + ul";
    let news_blog_post = Selector::parse(post_css_selector).unwrap();
    match html_doc.select(&news_blog_post).next() {
      None => {
        println!("error scraping selector: {}", post_css_selector);
        return String::from("");
      }
      Some(ul) => {
        let li_selector = Selector::parse("li").unwrap();
        let mut urls = String::new();
        for element in ul.select(&li_selector) {
          urls = format!("{}&#8226; {}\n", urls, element.inner_html());
        }
        urls
      }
    }
  }

  fn scrap_title_link(&self, html_doc: &Html) -> String {
    let title_css_selector = "div.post-title div.text-right a";
    let title_selector = Selector::parse(title_css_selector).unwrap();
    if let Some(title_div) = html_doc.select(&title_selector).next() {
      let title_link = format!("<b>{}</b>\n", title_div.inner_html());
      return title_link;
    }
    println!("error scraping selector: {}", title_css_selector);
    return String::from("");
  }

  fn scrap_date(&self, html_doc: &Html) -> String {
    let date_css_selector = "div.post-title div span time";
    let date_selector = Selector::parse(date_css_selector).unwrap();
    if let Some(date_div) = html_doc.select(&date_selector).next() {
      let date = format!("<b>{}</b>\n", date_div.inner_html());
      return date;
    }
    println!("error scraping selector: {}", date_css_selector);
    return String::from("");
  }
}

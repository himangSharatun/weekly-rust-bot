use async_trait::async_trait;
use tgbot::{
  longpoll::LongPoll,
  methods::SendMessage,
  types::{Message, ParseMode, Update, UpdateKind},
  Api, Config, UpdateHandler,
};

use crate::scraper::scraper::Scraper;

#[derive(Clone)]
pub struct Bot {
  api: Api,
  scraper: Scraper,
}

impl Bot {
  pub fn new(token: String, scraper: Scraper) -> Self {
    let config = Config::new(token.clone());
    let api = Api::new(config).expect("Failed to create API");
    Self { api, scraper }
  }

  pub async fn start_long_poll(&self) {
    LongPoll::new(self.api.clone(), self.clone()).run().await;
  }

  async fn handle_update(&self, update: Update) -> Option<Message> {
    match update.kind {
      UpdateKind::Message(message) => {
        let chat_id = message.get_chat_id();
        if let Some(commands) = message.commands {
          let command = &commands[0];
          let response = match command.command.as_str() {
            "/start" => self.get_current_week_issue(chat_id).await,
            "/rust" => self.get_current_week_issue(chat_id).await,
            _ => self.handle_unknown_command(chat_id).await,
          };
          return response;
        }
      }
      _ => {}
    }
    None
  }
  async fn get_current_week_issue(&self, chat_id: i64) -> Option<Message> {
    let message_text = self.scraper.scrap_weekly_rust().await;
    let message = SendMessage::new(chat_id, message_text).parse_mode(ParseMode::Html);
    return Some(self.api.execute(message).await.unwrap());
  }
  async fn handle_unknown_command(&self, chat_id: i64) -> Option<Message> {
    let message_text = "Hey, sorry I've got no response for such command";
    let message = SendMessage::new(chat_id, message_text).parse_mode(ParseMode::Html);
    return Some(self.api.execute(message).await.unwrap());
  }
}

#[async_trait]
impl UpdateHandler for Bot {
  async fn handle(&mut self, update: Update) {
    log::info!("Got an update: {:?}", update);
    if let Some(msg) = self.handle_update(update).await {
      log::info!("Message sent: {:?}", msg);
    }
  }
}

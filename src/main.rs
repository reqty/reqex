use async_trait::async_trait;
use frankenstein::{
    AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, ReplyParameters, SendMessageParams,
    UpdateContent,
};
use lazy_static::lazy_static;
use regex::Regex;
use sedregex::find_and_replace;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = AsyncApi::new(&token);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params).await;

        match result {
            Err(error) => {
                println!("Failed to get updates: {error:?}"); //TODO: backoff
            }

            Ok(response) => {
                for update in response.result {
                    match update.content {
                        UpdateContent::Message(message) => {
                            let api = api.clone();

                            tokio::spawn(async move {
                                let handlers: &[&dyn MessageHandler] = &[&Shrug, &Reqex];

                                for handler in handlers {
                                    let handler = handler;
                                    if handler.test(&message).await {
                                        _ = handler.handle(message, api).await;
                                        return
                                    }
                                }
                            });
                        }

                        _ => {}
                    }

                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
        }
    }
}

#[async_trait]
trait MessageHandler {
    async fn test(&self, message: &Message) -> bool;
    async fn handle(&self, message: Message, api: AsyncApi) -> Option<()>;
}

// fn message_handlers() ->  {
//
// }

struct Shrug;
#[async_trait]
impl MessageHandler for Shrug {
    async fn test(&self, message: &Message) -> bool {
        message.text.as_ref().is_some_and(|t| t == "/shrug")
    }

    async fn handle(&self, message: Message, api: AsyncApi) -> Option<()> {
        let result_message = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("¯\\_(ツ)_/¯")
            .build();

        if let Err(err) = api.send_message(&result_message).await {
            println!("Failed to send shrug: {err:?}");
            return None;
        }

        Some(())
    }
}

lazy_static! {
    static ref SED_EXPR_RE: Regex = Regex::new("^s/(.*)/(.*)/.*").expect("Failed compiling regex");
}

struct Reqex;
#[async_trait]
impl MessageHandler for Reqex {
    async fn test(&self, message: &Message) -> bool {
        message
            .text
            .as_ref()
            .is_some_and(|user_pattern| SED_EXPR_RE.is_match(&user_pattern))
    }

    async fn handle(&self, message: Message, api: AsyncApi) -> Option<()> {
        let user_input = message.reply_to_message?.text?;
        let user_pattern = message.text?;

        let Ok(result) = find_and_replace(&user_input, &[user_pattern]) else {
            return None;
        };

        let result_message = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(result)
            .reply_parameters(
                ReplyParameters::builder()
                    .message_id(message.message_id)
                    .build(),
            )
            .build();

        if let Err(err) = api.send_message(&result_message).await {
            println!("Failed to send reply: {err:?}");
            return None;
        }

        Some(())
    }
}

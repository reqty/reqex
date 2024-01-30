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
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();

                        tokio::spawn(async move {
                            _ = reqex_message(message, api_clone).await;
                        });
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

lazy_static! {
    static ref SED_EXPR_RE: Regex = Regex::new("^s/(.*)/(.*)/.*").expect("Failed compiling regex");
}

async fn reqex_message(message: Message, api: AsyncApi) -> Option<()> {
    // validate input
    let user_pattern = message.text?;
    let message_replyof = *(message.reply_to_message?);
    let user_input = message_replyof.text?;

    if !SED_EXPR_RE.is_match(&user_pattern) {
        return None;
    };

    println!(
        "<{}.{:?}>: {}",
        message.chat.id, message.from?.first_name, user_pattern
    );

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

use std::env;

use futures::StreamExt;
use telegram_bot::*;
use std::error::Error;

use regex::{RegexBuilder, Regex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sed_expr_re = Regex::new("^s/(.*)/(.*)/.*").unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                if !sed_expr_re.is_match(data) {
                    continue;
                }
                let (prev, new, flags) = parse_sed_expr(data);

                let re = match RegexBuilder::new(&prev).case_insensitive(flags.case_insensitive).build() {
                    Ok(r) => r,
                    Err(_) => continue
                };

                let new_ref: &str = new.as_ref();
                let resp = match &message.reply_to_message {
                    Some(m) => match m.text() {
                        Some(t) =>
                            {
                                if flags.global {
                                    re.replace_all(&t, new_ref).to_string()
                                } else {
                                    re.replace(&t, new_ref).to_string()
                                }
                            },
                        None => continue
                    },
                    None => continue
                };

                println!("<{}.{}>: {}", &message.chat.id(), &message.from.first_name, data);
                api.send(message.text_reply(
                    resp
                )).await?;
            }
        }
    }
    Ok(())
}

struct Flags {
    global: bool,
    case_insensitive: bool,
    //multiline: bool
}

impl Flags{
    fn new(s: &str) -> Flags {
        Flags {
            global: s.contains('g'),
            case_insensitive: s.contains('i'),

        }
    }
}

fn parse_sed_expr(data: &str) -> (String, String, Flags) {
    let (prev, offs) = parse_to_sep(&data[2..], true);
    //println!("pse got {}: {}", offs, prev);
    let (new, offs) = parse_to_sep(&data[3+offs..], false);
    //println!("pse got {}: {}", offs, new);
    (prev, new, Flags::new(&data[offs..]))
}

fn parse_to_sep(chars: &str, keep_double_backslash: bool) -> (String, usize) {
    let chars: Vec<char> = chars.chars().collect();
    //println!("pts: {:?}", chars);
    let mut prev: Vec<char> = Vec::new();
    let mut l = 0; // offset in the input data
    let mut it = chars.iter().enumerate();
    while let Some((idx, char)) = it.next() {
        if *char == '\\' {
            //peek
            // if matches, skip one with next
            // if /, skip this, add that.
            if chars.len() > idx + 1 {
                match chars[idx + 1] {
                    '\\' => {
                        prev.push('\\');
                        if keep_double_backslash {
                            prev.push('\\');
                        }
                        it.next();
                        l += 2;
                        continue;
                    },
                    '/' => {
                        prev.push('/');
                        it.next();
                        l += 2;
                        continue;
                    },
                    _ => ()
                }
            }
        }
        if *char == '/' {
            // separator
            return (prev.iter().clone().collect(), l);
        }
        l += 1;
        prev.push(*char);
    }
    (prev.iter().clone().collect(), l)
}

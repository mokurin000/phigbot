use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use rand::{thread_rng, Rng};
use std::ops::Range;
mod constants;
use constants::TIPS;

enum Case {
    Random,
    List,
    Search,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let mut results = vec![];
            let case;

            match &*query.query {
                "" => {
                    case = Case::Random;
                    let text = TIPS[rand_index(0..TIPS.len()).await];
                    let content_text = InputMessageContentText::new(text);
                    let content = InputMessageContent::Text(content_text);
                    let random_result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                        "0", text, content,
                    ));

                    results.push(random_result);
                }
                "*" => {
                    case = Case::List;
                    let text = "https://github.com/poly000/phigbot/blob/main/src/constants/mod.rs";
                    let content = InputMessageContent::Text(InputMessageContentText::new(text));
                    let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                        "0",
                        "全部tips见mod.rs",
                        content,
                    ));
                    results.push(result);
                }
                _ => {
                    case = Case::Search;
                    let downcase_key = query.query.to_lowercase();
                    for (&text, i) in TIPS
                        .iter()
                        .filter(|s| s.to_lowercase().contains(&downcase_key))
                        .zip(0..)
                    {
                        let content = InputMessageContent::Text(InputMessageContentText::new(text));
                        let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                            i.to_string(),
                            text,
                            content,
                        ));
                        results.push(result);
                    }
                }
            }

            let cache_time = match case {
                // should be the link to `mod.rs` forever
                Case::List => u32::MAX,

                // cache search results so I do not need to memorize the result manually
                Case::Search => 600,

                // do not cache random results
                Case::Random => 0,
            };
            let response = bot
                .answer_inline_query(&query.id, results)
                .cache_time(cache_time)
                .send()
                .await;

            if let Err(e) = response {
                log::error!("Error in handler: {:?}", e);
            }

            respond(())
        },
    ));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}

async fn rand_index(range: Range<usize>) -> usize {
    thread_rng().gen_range(range)
}

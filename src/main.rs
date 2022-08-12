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
            let (results, case) = get_results(&query.query);

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

fn rand_index(range: Range<usize>) -> usize {
    thread_rng().gen_range(range)
}

fn get_results(query: &str) -> (Vec<InlineQueryResult>, Case) {
    match query {
        "" => {
            let text = TIPS[rand_index(0..TIPS.len())];
            let content_text = InputMessageContentText::new(text);
            let content = InputMessageContent::Text(content_text);
            let random_result =
                InlineQueryResult::Article(InlineQueryResultArticle::new("0", text, content));

            (vec![random_result], Case::Random)
        }
        "*" => {
            let text = "https://github.com/poly000/phigbot/blob/main/src/constants/mod.rs";
            let content = InputMessageContent::Text(
                InputMessageContentText::new(text).disable_web_page_preview(true),
            );
            let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                "0",
                "全部tips见mod.rs",
                content,
            ));
            (vec![result], Case::List)
        }
        _ => {
            let downcase_key = query.to_lowercase();
            let results = TIPS
                .iter()
                .enumerate()
                .filter_map(|(i, &text)| {
                    if !text.to_lowercase().contains(&downcase_key) {
                        return None;
                    }

                    let content = InputMessageContent::Text(InputMessageContentText::new(text));
                    let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                        // will get duplicated id error if elements of TIPS aren't unique
                        i.to_string(),
                        text,
                        content,
                    ));
                    Some(result)
                })
                .collect();
            (results, Case::Search)
        }
    }
}

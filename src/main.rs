use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use rand::{thread_rng, Rng};
use std::ops::Range;
mod constants;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            let mut results = vec![];

            if query.query.is_empty() {
                let text = constants::TIPS[rand_index(0..constants::TIPS.len()).await];
                let content_text = InputMessageContentText::new(text);
                let content = InputMessageContent::Text(content_text);
                let random_result =
                    InlineQueryResult::Article(InlineQueryResultArticle::new("0", text, content));

                results.push(random_result);
            } else {
                for (&text, i) in constants::TIPS
                    .iter()
                    .filter(|s| s.contains(&query.query))
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

            let response = bot
                .answer_inline_query(&query.id, results)
                .cache_time(0)
                .send()
                .await;

            if let Err(e) = response {
                log::error!("Error in handler: {:?}", e);
            }

            respond(())
        },
    ));

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

async fn rand_index(range: Range<usize>) -> usize {
    thread_rng().gen_range(range)
}

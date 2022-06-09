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
            let text = constants::TIPS[rand_index(0..constants::TIPS.len()).await];
            let content_text = InputMessageContentText::new(text);
            let content = InputMessageContent::Text(content_text);
            let result = vec![InlineQueryResult::Article(InlineQueryResultArticle::new(
                "0", text, content,
            ))];
            
            let response = bot.answer_inline_query(&query.id, result).cache_time(0).send().await;

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

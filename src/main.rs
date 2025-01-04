use sorensen::distance;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
        LinkPreviewOptions,
    },
};

use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{cell::RefCell, fmt::Display, iter::once, ops::Range};

mod constants;
use constants::TIPS;

const MAX_INLINE_QUERY_RESULT_NUM: usize = 50;

enum Case {
    Random,
    List,
    Search,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: Bot| async move {
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

fn get_results(query: &str) -> (Vec<InlineQueryResult>, Case) {
    match query {
        "" => {
            let text = TIPS[rand_index(0..TIPS.len())];
            let random_result = make_result((0, text)).unwrap();
            (vec![random_result], Case::Random)
        }
        "*" => {
            let text = "https://github.com/poly000/phigbot/blob/main/src/constants/mod.rs";
            let content = InputMessageContent::Text(
                InputMessageContentText::new(text).link_preview_options(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_large_media: false,
                    prefer_small_media: false,
                    show_above_text: false,
                }),
            );
            let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
                "0",
                "全部tips见mod.rs",
                content,
            ));
            (vec![result], Case::List)
        }
        _ => {
            let mut results = TIPS
                .into_iter()
                .map(|s| (s, distance(s.as_bytes(), query.as_bytes())))
                .collect::<Vec<_>>();
            results.sort_unstable_by_key(|(_, d)| (d * -1024.) as i32);
            results.truncate(MAX_INLINE_QUERY_RESULT_NUM);

            (
                results
                    .into_iter()
                    .map(|(text, _)| text)
                    .enumerate()
                    .filter_map(make_result)
                    .collect(),
                Case::Search,
            )
        }
    }
}

fn make_result((id, title): (impl Display, impl AsRef<str>)) -> Option<InlineQueryResult> {
    let title = title.as_ref();
    let id = id.to_string();

    once(title)
        .map(InputMessageContentText::new)
        .map(InputMessageContent::Text)
        .map(|input_message_content| {
            InlineQueryResultArticle::new(&id, title, input_message_content)
        })
        .map(InlineQueryResult::Article)
        .next()
}

fn rand_index(range: Range<usize>) -> usize {
    RNG.with_borrow_mut(|rng| rng.gen_range(range))
}

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(thread_rng());
}

use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    api::{get_comment_by_id, get_story_comments},
    ui::CommentsState,
    Comment, StoryData, StoryItem,
};

/*

kovac 1 hour ago | next [–]

Love it! Any idea how long the display can last? I've been playing around with e-paper (nothing as impressive as this!) dashboards. I use Waveshare displays that has a max of 1 million refresh cycles. The display you've used seems more capable.
My own humble e-paper projects:

https://www.asciimx.com/projects/e-reader/ https://www.asciimx.com/projects/etlas/

reply
*/

#[component]
pub fn CommentLi(comment: Comment) -> Element {
    // TODO:卡
    // let comment =
    //     use_resource(move || async move { get_comment_by_id(story_item.id).await.unwrap() });
    let text = comment.text.clone().unwrap_or_else(|| "".to_string());
    rsx! {
        article {
            class: "p-4 text-gray-500 leading-7 tracking-wider border-b border-gray-200",
            span {
                "{comment.by} {comment.time} | next[-]"
            }
            p {
                dangerous_inner_html: text
            }
        }
    }
}

pub(crate) async fn load_comments(
    story_item: StoryItem,
    mut comments_state: Signal<CommentsState>,
    mut full_story: Signal<Option<StoryData>>,
) {
    // if the story is already loaded, set the comments state to loaded

    if let Some(story) = &full_story() {
        info!("story is already loaded");
        comments_state.set(CommentsState::Loaded(story.clone()));
        return;
    }
    // load the comments
    comments_state.set(CommentsState::Loading);
    match get_story_comments(story_item).await {
        Ok(story) => {
            info!("story is loaded");
            full_story.set(Some(story.clone()));
            comments_state.set(CommentsState::Loaded(story));
        }
        Err(e) => {
            info!("story is not loaded");
            comments_state.set(CommentsState::Error(e.to_string()));
        }
    }
}

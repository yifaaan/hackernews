use dioxus::prelude::*;

use crate::{api::get_comment_by_id, StoryItem};

#[component]
pub fn CommentLi(story_item: StoryItem) -> Element {
    let comment =
        use_resource(move || async move { get_comment_by_id(story_item.id).await.unwrap() });
    let text = comment.read().clone().unwrap().text.unwrap();
    rsx! {
        article { class: "mt-8 text-gray-500 leading-7 tracking-wider",
            p { "Hi Akhil," }
            p {
                "{text}"
            }
            footer { class: "mt-12",
                p { "Thanks & Regards," }
                p { "Alexandar" }
            }
        }
    }
}

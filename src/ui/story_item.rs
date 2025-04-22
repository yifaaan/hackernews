use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    api::get_story_comments,
    ui::{comment::load_comments, CommentsState},
    StoryData,
};

use super::StoryItem;

#[component]
pub fn StoryItemLi(story_item: StoryItem) -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();
    // cache the full story
    let full_story = use_signal(|| None);
    let url = story_item.url.clone().unwrap_or_else(|| "".to_string());
    rsx! {
        li {
            class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { class: "flex justify-between items-center", href: "#",
                h3 { class: "text-lg font-semibold", "{story_item.title}" }
                p { class: "text-md text-gray-400", "{url}" }
            }
            div { class: "text-md italic text-gray-400",
                p { class: "text-md text-gray-400",
                    "{story_item.score} points by {story_item.by} {story_item.time} | hide | "
                    a {
                        href: "#",
                        onclick: move |event| {
                            event.prevent_default();
                            info!("load comments");
                            let story_item = story_item.clone();
                            spawn(async move {
                                info!("load comments in spawn");
                                load_comments(story_item, comments_state, full_story).await;
                            });
                        },
                        "{story_item.descendants} comments" }
                }
            }
        }
    }
}

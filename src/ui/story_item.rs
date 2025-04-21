use dioxus::prelude::*;

use crate::StoryItem;

#[component]
pub fn StoryItemLi(story_item: StoryItem) -> Element {
    let url = story_item.url.unwrap_or_else(|| "".to_string());
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { class: "flex justify-between items-center", href: "#",
                h3 { class: "text-lg font-semibold", "{story_item.title}" }
                p { class: "text-md text-gray-400", "{url}" }
            }
            div { class: "text-md italic text-gray-400",
                p { class: "text-md text-gray-400",
                    "{story_item.score} points by {story_item.by} {story_item.time} | hide | "
                    a { href: "#", "{story_item.descendants} comments" }
                }
            }
        }
    }
}

#![allow(unused)]
use comment::CommentLi;
use dioxus::prelude::*;
use story_item::StoryItemLi;

use crate::{api::get_top_stories, StoryData, StoryItem};

pub mod comment;
pub mod story_item;

pub static CSS: Asset = asset!("assets/tailwind.css");

#[derive(Debug, Clone)]
pub(crate) enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
    Error(String),
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[route("/app")]
    App {},

    #[route("/")]
    HackerNews {},

    #[route("/:..others")]
    NotFound { others: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        Router::<Route> {}
    }
}

#[component]
fn NotFound(others: Vec<String>) -> Element {
    let path = others.join("/");
    rsx! {
        main {
            class: "grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8",

            div {
                class: "text-center",
                p {
                    class: "text-base font-semibold text-indigo-600",
                    "404"
                }
                h1 {
                    class: "mt-4 text-5xl font-semibold tracking-tight text-balance text-gray-900 sm:text-7xl",
                    "Page {path} Not Found"
                }
                p {
                    class: "mt-6 text-lg font-medium text-pretty text-gray-500 sm:text-xl/8",
                    "Sorry, we couldn’t find the page you’re looking for."
                }
                div {
                    class: "mt-10 flex items-center justify-center gap-x-6",
                    a {
                        class: "rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                        href: "/",
                        "Go back home"
                    }
                    a {
                        class: "text-sm font-semibold text-gray-900",
                        href: "/",

                        "Contact support"
                        span {
                            aria_hidden: "true",
                            "->"
                        }
                    }
                }
            }
        }

    }
}

#[component]
fn HackerNews() -> Element {
    // let story_items = use_resource(|| async move { get_top_stories(2).await.unwrap() });
    let comments_state = use_context_provider(|| Signal::new(CommentsState::Unset));

    rsx! {
        main {
            class: "flex w-full h-full shadow-lg rounded-3xl",
            section {
                class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
                Stories {}
            }
            section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl",
                section {
                    Comment {}
                }
            }
        }
    }
}

#[component]
fn Stories() -> Element {
    // TODO:为什么这里会卡住
    // let stories = use_resource(|| async move {
    //     get_top_stories(2).await.unwrap()
    // });
    let stories = use_resource(|| get_top_stories(20));

    match &*stories.read_unchecked() {
        Some(Ok(stories)) => {
            rsx! {
                ul {
                    for si in stories {
                        StoryItemLi {story_item: si.clone()}
                    }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div {
                    class: "text-red-500",
                    p {"Error: {e}"}
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "text-gray-500",
                    p {"Loading..."}
                }
            }
        }
    }
}

#[component]
fn Comment() -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();
    match comments_state() {
        CommentsState::Unset => {
            rsx! {}
        }
        CommentsState::Loading => {
            rsx! {
                div {
                    class: "text-gray-500",
                    p {"Loading comments..."}
                }
            }
        }
        CommentsState::Loaded(story_data) => {
            rsx! {
                ul {
                    for comment in story_data.comments {
                        CommentLi {comment: comment.clone()}
                    }
                }
            }
        }
        CommentsState::Error(e) => {
            rsx! {
                div {
                    class: "text-red-500",
                    p {"Error: {e}"}
                }
            }
        }
    }
}

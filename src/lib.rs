use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub(crate) mod api;
pub(crate) mod ui;

pub use ui::App;

use dioxus::prelude::*;

pub static ICON: Asset = asset!("assets/favicon.ico");

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryItem {
    pub by: String,
    pub descendants: i64,
    pub id: i64,
    pub kids: Vec<i64>,
    pub score: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub title: String,
    pub r#type: String,
    pub url: Option<String>,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Comment {
    pub by: String,
    pub id: i64,
    pub parent: i64,
    pub text: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub created: i64,
    pub id: String,
    pub karma: i64,
    pub submitted: Vec<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoryData {
    pub item: StoryItem,
    pub comments: Vec<Comment>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

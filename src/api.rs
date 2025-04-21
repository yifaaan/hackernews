#![allow(dead_code)]

use crate::{Comment, StoryData, StoryItem};
use anyhow::Result;

const MAX_STORIES: usize = 20;

pub async fn get_top_stories(mut n: usize) -> Result<Vec<StoryItem>> {
    n = n.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let response = reqwest::get(url).await?;
    let ids = response.json::<Vec<i64>>().await?.into_iter().take(n);
    let story_futures = ids.into_iter().map(get_story_item_by_id);
    let stories = futures::future::join_all(story_futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    Ok(stories)
}

pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let response = reqwest::get(url).await?;
    let story = response.json::<StoryItem>().await?;
    Ok(story)
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let response = reqwest::get(url).await?;
    let comment = response.json::<Comment>().await?;
    Ok(comment)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = futures::future::join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    Ok(StoryData { item, comments })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_stories() {
        let stories = get_top_stories(2).await.unwrap();
        assert!(stories.len() == 2);
    }

    #[tokio::test]
    async fn test_get_story_comments() {
        let story = get_top_stories(1).await.unwrap();
        let story = &story[0];
        let item = get_story_item_by_id(story.id).await.unwrap();
        assert_eq!(item.id, story.id);
    }
}

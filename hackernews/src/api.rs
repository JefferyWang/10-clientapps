#![allow(unused)]
use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryData, StoryItem};

const MAX_STORIES: usize = 50;

pub async fn get_top_stories(mut n: usize) -> Result<Vec<StoryItem>> {
    let n = n.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids = reqwest::get(url).await?.json::<Vec<i64>>().await?;
    let story_futures = ids.into_iter().take(n).map(get_story_item_by_id);
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let response = reqwest::get(&url).await?.json::<StoryItem>().await?;
    Ok(response)
}

// only retrieve top level comments
pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|comment| comment.ok())
        .collect();
    Ok(StoryData { item, comments })
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let response = reqwest::get(&url).await?.json::<Comment>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_top_stories_should_work() {
        let stories = get_top_stories(3).await.unwrap();
        assert_eq!(stories.len(), 3);
    }

    #[tokio::test]
    async fn get_comment_by_id_should_work() -> Result<()> {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        let id = story.kids[0];
        let comment = get_comment_by_id(id).await.unwrap();
        assert_eq!(comment.id, id);
        Ok(())
    }
}

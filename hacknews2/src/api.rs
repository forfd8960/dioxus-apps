use anyhow::Result;
use futures::future::join_all;

use crate::StoryItem;

pub async fn get_top_stories() -> Result<Vec<StoryItem>> {
    let client = reqwest::Client::new();
    let ids: Vec<i64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?
        .json()
        .await?;

    let story_futures = ids.into_iter().take(10).map(get_story);

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<StoryItem>>();
    Ok(stories)
}

pub async fn get_story(id: i64) -> Result<StoryItem> {
    let client = reqwest::Client::new();
    let story: StoryItem = client
        .get(format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            id
        ))
        .send()
        .await?
        .json()
        .await?;
    Ok(story)
}

use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryData, StoryItem};

const MAX_STORY_COUNT: u32 = 10;

pub async fn get_top_stories(n: u32) -> Result<Vec<StoryItem>> {
    let ids: Vec<i64> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json()
        .await?;

    let n = n.min(MAX_STORY_COUNT);
    let story_futures = ids.into_iter().take(n as usize).map(get_story);

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<StoryItem>>();
    Ok(stories)
}

pub async fn get_story(id: i64) -> Result<StoryItem> {
    let story: StoryItem = reqwest::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        id
    ))
    .await?
    .json()
    .await?;
    Ok(story)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id as i64));

    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<Comment>>();

    Ok(StoryData { item, comments })
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let client = reqwest::Client::new();
    let comment: Comment = client
        .get(format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            id
        ))
        .send()
        .await?
        .json()
        .await?;
    Ok(comment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_stories() -> anyhow::Result<()> {
        let stories = get_top_stories(3).await?;
        println!("{:#?}", stories);
        assert_eq!(stories.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_comment_should_work() -> anyhow::Result<()> {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        println!("{:#?}", story);

        let c_id = story.kids[0];
        let comment = get_comment_by_id(c_id as i64).await.unwrap();
        println!("{:?}", comment);

        assert_eq!(comment.id, c_id);
        Ok(())
    }
}

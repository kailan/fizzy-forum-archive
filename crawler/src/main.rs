mod requests;
mod types;

use std::fs;

use chrono::Utc;
use requests::*;
use reqwest::Client;

use types::*;

use anyhow::Result;

const COOKIE: &str = "<replace me>";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    println!("Starting export...");

    let mut export = structure::ExportMetadata {
        generated_at: Utc::now(),
        generated_hostname: hostname::get().map(|h| h.to_str().unwrap().to_string()).unwrap_or_else(|_| "Unknown".to_string()),
        forums: vec![],
    };

    fs::create_dir_all("out")?;

    println!("Fetching forum index...");
    let forums = fetch_forums(&client).await?;

    for forum in forums {
        let mut thread_index: Vec<structure::ThreadMetadata> = vec![];

        println!("Fetching topic index for forum '{}' ({})", forum.name, forum.id);
        let topics = fetch_topics(&client, &forum).await?;

        fs::create_dir_all(format!("out/{}", forum.id))?;

        for topic in topics {
            println!("Fetching post index for topic {}", topic.id);
            let posts = fetch_posts(&client, &forum, &topic).await?;

            let initial_post = posts.last().expect("no posts in topic");

            let thread = structure::ThreadMetadata {
                id: topic.id,
                title: initial_post.content.title.text.clone(),
                owner: topic.owner.into(),
            };

            fs::create_dir_all(format!("out/{}/{}", forum.id, thread.id))?;

            for post in posts {
                let post = structure::Post {
                    id: post.id,
                    owner: structure::User { id: post.poster_id, name: post.poster_name, profile_image: None },
                    created_date: post.created_date.parse().expect("invalid date format"),
                    modified_date: None, // TODO
                    content: post.content.text.text,
                };

                fs::write(format!("out/{}/{}/{}.json", forum.id, thread.id, post.id), serde_json::to_string_pretty(&post)?)?;
            }

            thread_index.push(thread);
        }

        fs::write(format!("out/{}/threads.json", forum.id), serde_json::to_string_pretty(&thread_index)?)?;
        export.forums.push(structure::ForumMetadata { id: forum.id, title: forum.name, post_count: forum.posts });
    }

    fs::write("out/export.json".to_string(), serde_json::to_string_pretty(&export)?)?;

    Ok(())
}

async fn fetch_data<T>(client: &Client, request: DataRequest) -> Result<DataResponse<T>>
where
    T: serde::de::DeserializeOwned,
{
    let resp = client.post("https://api-my.fizzyliving.com/blade-app/get/data")
        .header("Cookie", COOKIE)
        .header("App-Path", "typeID:my-social, appID:my-social")
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("Accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.55 Safari/537.36")
        .body(serde_json::to_string(&request)?)
        .send().await?;

    // ghetto rate limiting
    std::thread::sleep(std::time::Duration::from_millis(250));

    Ok(resp.json().await?)
}

async fn fetch_forums(client: &Client) -> Result<Vec<Forum>> {
    let req = DataRequest::for_forum_index();
    let resp: DataResponse<ForumIndex> = fetch_data(client, req).await?;
    Ok(resp.data.expect("no data in forum index response").residents_forums)
}

async fn fetch_topics(client: &Client, forum: &Forum) -> Result<Vec<Topic>> {
    let req = DataRequest::for_forum(&forum.id);
    let resp: DataResponse<ForumResponse> = fetch_data(client, req).await?;
    Ok(resp.data.expect("no data in forum response").topics.unwrap_or_default())
}

async fn fetch_posts(client: &Client, forum: &Forum, topic: &Topic) -> Result<Vec<Post>> {
    let req = DataRequest::for_topic(&forum.id, &topic.id);
    let resp: DataResponse<ForumResponse> = fetch_data(client, req).await?;
    Ok(resp.data.expect("no data in board response").board.expect("board not found").posts)
}

mod requests;
mod types;

use std::fs;

use requests::*;
use reqwest::Client;
use types::*;

use anyhow::Result;

const COOKIE: &str = "<REPLACE ME>";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    println!("Fetching forum index...");
    let forums = fetch_forums(&client).await?;

    fs::create_dir_all("out")?;
    fs::write("out/forums.json".to_string(), serde_json::to_string_pretty(&forums)?)?;

    for forum in forums {
        println!("Fetching topic index for forum '{}' ({})", forum.name, forum.id);
        let topics = fetch_topics(&client, &forum).await?;

        for topic in topics {
            fs::create_dir_all(format!("out/{}/{}", forum.id, topic.id))?;
            fs::write(format!("out/{}/{}/topic.json", forum.id, topic.id), serde_json::to_string_pretty(&topic)?)?;

            println!("Fetching post index for topic {}", topic.id);
            let posts = fetch_posts(&client, &forum, &topic).await?;

            for post in posts {
                fs::write(format!("out/{}/{}/post-{}.json", forum.id, topic.id, post.id), serde_json::to_string_pretty(&post)?)?;
            }
        }
    }

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

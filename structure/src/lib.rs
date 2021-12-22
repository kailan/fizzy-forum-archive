use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

///
/// out /
///     export.json : Export (forum index)
///     { forum id } / threads.json : Vec<ThreadMetadata> (thread index)
///     { forum id } / { thread id } / { post id }.json : Post (post content)

#[derive(Serialize, Deserialize)]
pub struct ExportMetadata {
  pub generated_at: DateTime<Utc>,
  pub generated_hostname: String,
  pub forums: Vec<ForumMetadata>
}

#[derive(Serialize, Deserialize)]
pub struct ForumMetadata {
  pub id: String,
  pub title: String,
  pub post_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct ThreadMetadata {
  pub id: String,
  pub title: String,
  pub owner: User
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub profile_image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
  pub id: String,
  pub owner: User,
  pub created_date: DateTime<Utc>,
  pub modified_date: Option<DateTime<Utc>>,
  pub content: String,
}

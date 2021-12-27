use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: Option<T>,
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumIndex {
    pub residents_forums: Vec<Forum>,
}

#[derive(Serialize, Deserialize)]
pub struct ForumResponse {
    pub forum: Forum,
    pub topics: Option<Vec<Topic>>,
    pub board: Option<Board>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forum {
    pub id: String,
    pub is_official: bool,
    pub name: String,
    pub summary: Option<String>,
    pub board_image: Option<String>,
    pub posts: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub id: String,
    pub owner: User,
    pub created_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub full_name: String,
    #[serde(rename = "pImg")]
    pub profile_image: Option<String>,
}

impl From<User> for structure::User {
    fn from(user: User) -> Self {
        structure::User {
            id: user.id,
            name: user.name,
            profile_image: user.profile_image,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    #[serde(rename = "boardId")]
    pub id: String,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub poster_id: String,
    #[serde(rename = "name")]
    pub poster_name: String,
    pub created_date: String,
    pub modified_date: String,
    pub modified_name: Option<String>,
    #[serde(rename = "post")]
    pub content: PostContent,
    #[serde(default)]
    pub likes: Vec<PostReaction>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContent {
    pub title: LocalContent,
    pub text: LocalContent,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalContent {
    pub locale: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostReaction {
    #[serde(rename = "pid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "pimg")]
    pub profile_image: Option<String>,
    pub reaction: String
}

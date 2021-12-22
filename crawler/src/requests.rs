use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequest {
    pub area: String,
    pub callback: Option<String>,
    pub id: String,
    pub external: bool,
    pub name: String,
    pub type_id: String,
    pub data: Option<DataParams>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataParams {
    pub forum_id: String,
    pub page_size: i32,
    pub scrolling: bool,
    pub topic_id: Option<String>,
}

impl Default for DataParams {
    fn default() -> Self {
        DataParams {
            page_size: 100,
            scrolling: true,
            forum_id: "".to_string(),
            topic_id: None,
        }
    }
}

impl Default for DataRequest {
    fn default() -> Self {
        DataRequest {
            area: "my-social-clubs".to_string(),
            callback: None,
            id: "my-social-navigation".to_string(),
            external: false,
            name: "Navigation".to_string(),
            type_id: "my-social".to_string(),
            data: None,
        }
    }
}

impl DataRequest {
    pub fn for_forum_index() -> Self {
        DataRequest {
            id: "my-social-navigation".to_string(),
            name: "Navigation".to_string(),
            ..Default::default()
        }
    }

    pub fn for_forum(forum_id: &str) -> Self {
        DataRequest {
            id: "my-forum-navigation".to_string(),
            name: "Forum Navigation".to_string(),
            data: Some(DataParams {
                forum_id: forum_id.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn for_topic(forum_id: &str, topic_id: &str) -> Self {
        DataRequest {
            id: "my-forum-board".to_string(),
            name: "Forum".to_string(),
            data: Some(DataParams {
                forum_id: forum_id.to_string(),
                topic_id: Some(topic_id.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

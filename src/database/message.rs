use bson::UtcDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousEntry {
    pub content: String,
    pub time: UtcDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    // pub nonce: String, used internally
    pub channel: String,
    pub author: String,

    pub content: String,
    pub edited: Option<UtcDateTime>,

    pub previous_content: Option<Vec<PreviousEntry>>,
}

// ? TODO: write global send message
// ? pub fn send_message();
// ? handle websockets?
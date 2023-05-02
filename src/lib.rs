use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<B> {
    pub src: String,
    pub dest: String,
    pub body: B,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitBody {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        msg_id: usize,
        in_reply_to: usize,
        id: Uuid,
    },
}

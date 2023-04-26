use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Body {
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        let message: Message = serde_json::from_str(&line).unwrap();
        println!("{:?}", message.body);
    }

    Ok(())
}

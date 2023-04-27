use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

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
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
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
    let mut stdout = io::stdout().lock();

    for line in stdin.lines() {
        let line = line.unwrap();
        let request: Message = serde_json::from_str(&line).context("deserialize message")?;

        let reply = match request.body {
            Body::Init {
                msg_id,
                node_id,
                node_ids,
            } => Some(Message {
                src: request.dest,
                dest: request.src,
                body: Body::InitOk {
                    in_reply_to: msg_id,
                },
            }),
            Body::InitOk { .. } => None,
            Body::Echo { msg_id, echo } => Some(Message {
                src: request.dest,
                dest: request.src,
                body: Body::EchoOk {
                    msg_id: 1,
                    in_reply_to: msg_id,
                    echo,
                },
            }),
            Body::EchoOk { .. } => None,
        };

        if let Some(reply) = reply {
            serde_json::to_writer(&mut stdout, &reply).context("serialize reply")?;
            stdout.write_all(b"\n").context("write trailing new line")?;
        }
    }

    Ok(())
}

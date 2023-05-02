use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use uuid::Uuid;

use nautilus::{InitBody, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        msg_id: usize,
        in_reply_to: usize,
        id: Uuid,
    },
}

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin().lock().lines();
    let mut stdout = io::stdout().lock();

    let init = stdin.next().context("parsing init").unwrap().unwrap();
    let init: Message<InitBody> =
        serde_json::from_str(&init).context("deserialize init message")?;

    let body = match init.body {
        InitBody::Init {
            msg_id,
            node_id: _,
            node_ids: _,
        } => Some(InitBody::InitOk {
            in_reply_to: msg_id,
        }),
        InitBody::InitOk { .. } => None,
    };

    if let Some(body) = body {
        let reply = Message {
            src: init.dest,
            dest: init.src,
            body,
        };
        serde_json::to_writer(&mut stdout, &reply).context("serialize reply")?;
        stdout.write_all(b"\n").context("write trailing new line")?;
    }

    for line in stdin {
        let line = line.unwrap();
        let request: Message<Body> = serde_json::from_str(&line).context("deserialize message")?;

        let body = match request.body {
            Body::Generate { msg_id } => Some(Body::GenerateOk {
                msg_id: 1,
                in_reply_to: msg_id,
                id: Uuid::new_v4(),
            }),
            Body::GenerateOk { .. } => None,
        };

        if let Some(body) = body {
            let reply = Message {
                src: request.dest,
                dest: request.src,
                body,
            };
            serde_json::to_writer(&mut stdout, &reply).context("serialize reply")?;
            stdout.write_all(b"\n").context("write trailing new line")?;
        }
    }

    Ok(())
}

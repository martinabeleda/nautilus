use anyhow::Context;
use std::io::{self, BufRead, Write};
use uuid::Uuid;

use nautilus::{Body, Message};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    for line in stdin.lines() {
        let line = line.unwrap();
        let request: Message = serde_json::from_str(&line).context("deserialize message")?;

        let body = match request.body {
            Body::Init {
                msg_id,
                node_id: _,
                node_ids: _,
            } => Some(Body::InitOk {
                in_reply_to: msg_id,
            }),
            Body::InitOk { .. } => None,
            Body::Echo { .. } => None,
            Body::EchoOk { .. } => None,
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

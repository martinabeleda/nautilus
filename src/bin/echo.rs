use anyhow::Context;
use std::io::{self, BufRead, Write};

use nautilus::{Body, Message};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    for line in stdin.lines() {
        let line = line.unwrap();
        let request: Message = serde_json::from_str(&line).context("deserialize message")?;

        let reply = match request.body {
            Body::Init {
                msg_id,
                node_id: _,
                node_ids: _,
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

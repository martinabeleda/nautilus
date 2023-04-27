# Nautilus

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![main workflow](https://github.com/martinabeleda/nautilus/actions/workflows/rust.yml/badge.svg)

Rust solutions for https://fly.io/dist-sys/

[Nautilus](<https://en.wikipedia.org/wiki/Nautilus_(fictional_submarine)>) is the fictional submarine belonging to Captain Nemo featured in Jules Verne's novels Twenty Thousand Leagues Under the Sea and The Mysterious Island which [seemed to have survived](https://en.wikipedia.org/wiki/Whirlpool#In_literature_and_popular_culture) the Lofoten Maelstrom.

## Setup

See [Maelstrom](https://github.com/jepsen-io/maelstrom) docs.

```shell
wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2
tar zxf maelstrom.tar.bz2
```

## Challenges

### Echo

Run node in maelstrom:

```shell
maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
```

Fingers crossed, you'll see: `Everything looks good! ヽ(‘ー`)ノ`

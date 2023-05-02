.PHONY: build test echo guid itest

build: clean
	cargo build

clean:
	cargo clean

test:
	cargo test

echo: build
	maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10

guid: build
	maelstrom test -w unique-ids \
		--bin target/debug/guid \
		--time-limit 30 \
		--rate 1000 \
		--node-count 3 \
		--availability total \
		--nemesis partition

itest: build echo guid
	echo "done"

build-echo:
	cargo build --features echo

build-generate:
	cargo build --features generate

build-broadcast:
	cargo build --features broadcast

run-echo: build-echo
	./maelstrom/maelstrom test -w echo --bin target/debug/gossip --node-count 1 --time-limit 3

run-ids: build-generate
	./maelstrom/maelstrom test -w unique-ids --bin target/debug/gossip --time-limit 5 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr

run-broadcast: build-broadcast
	./maelstrom/maelstrom test -w broadcast --bin target/debug/gossip --time-limit 25 --rate 10 --node-count 5 --log-stderr

run-broadcast-nf: build-broadcast
	./maelstrom/maelstrom test -w broadcast --bin target/debug/gossip --node-count 5 --time-limit 20 --rate 10 --nemesis partition --log-stderr

run-broadcast-efficient: build-broadcast
	./maelstrom/maelstrom test -w broadcast --bin target/debug/gossip --node-count 25 --time-limit 25 --rate 100 --latency 100 --nemesis partition --log-stderr

run-all: run-echo run-ids run-broadcast run-broadcast-nf

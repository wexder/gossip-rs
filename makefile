build:
	cargo build

run-echo: build
	./maelstrom/maelstrom test -w echo --bin target/debug/gossip --node-count 1 --time-limit 3

run-ids: build
	./maelstrom/maelstrom test -w unique-ids --bin target/debug/gossip --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

run-broadcast: build
	./maelstrom/maelstrom test -w broadcast --bin target/debug/gossip --time-limit 5 --rate 100 --node-count 5

run-broadcast-nf: build
	./maelstrom/maelstrom test -w broadcast --bin target/debug/gossip --node-count 5 --time-limit 5 --rate 10 --nemesis partition

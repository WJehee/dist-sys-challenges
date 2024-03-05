bin_path := "target/debug"

alias b := build

_default:
    just --list

# Build
build:
    cargo build

# Maelstrom serve
serve:
    ./maelstrom/maelstrom serve

# Run echo test (1)
echo: build
    ./maelstrom/maelstrom test -w echo --bin {{bin_path}}/echo --node-count 1 --time-limit 10

# Run unique-ids test (2)
unique-ids: build
    ./maelstrom/maelstrom test -w unique-ids --bin {{bin_path}}/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

# Run broadcast test a (3a) 
broadcast-a: build
    ./maelstrom/maelstrom test -w broadcast --bin {{bin_path}}/broadcast --node-count 1 --time-limit 20 --rate 10

# Run broadcast test b (3b)
broadcast-b: build
    ./maelstrom/maelstrom test -w broadcast --bin {{bin_path}}/broadcast --node-count 5 --time-limit 20 --rate 10

# Run broadcast test c (3c)
broadcast-c: build
    ./maelstrom/maelstrom test -w broadcast --bin {{bin_path}}/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition

# Run broadcast test d and e (3d & 3e)
broadcast-efficiency: build
    ./maelstrom/maelstrom test -w broadcast --bin {{bin_path}}/broadcast --node-count 25 --time-limit 20 --rate 100 --latency 100


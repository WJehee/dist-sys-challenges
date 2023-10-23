bin_path := "target/debug"

alias b := build

# Build
build:
    cargo build

# Run echo test as per challenge description
echo:
    cargo build
    ./maelstrom/maelstrom test -w echo --bin {{bin_path}}/echo --node-count 1 --time-limit 10

# Run unique-ids test as per challenge description
unique-ids:
    cargo build
    ./maelstrom/maelstrom test -w unique-ids --bin {{bin_path}}/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition


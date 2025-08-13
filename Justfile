build:
    cargo build

expand:
    cargo expand expr

test:
    cargo nextest run

run:
    RUST_BACKTRACE=1 cargo run

ast:
    ./generate_ast.py src
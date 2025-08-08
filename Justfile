build:
    cargo build

expand:
    cargo expand expr

test:
    cargo nextest run

run:
    cargo run

ast:
    ./generate_ast.py src
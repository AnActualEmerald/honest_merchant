web:
    trunk build
    sirv -Dq dist

build-web:
    trunk build

release-web:
    trunk build --release

dev:
    cargo run
web:
    trunk build
    sirv -Dq dist

build-web:
    trunk build

release-web:
    trunk build --release

release-web-run: release-web
    sirv -Dq dist

dev:
    cargo watch -x run

release:
    cargo build --release
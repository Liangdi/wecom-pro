dev:
    cargo run

build:
    cargo build

release:
    cargo build --release


release-patch:
    cargo release patch --no-publish --execute

release-minor:
    cargo release minor --no-publish --execute

release-major:
    cargo release major --no-publish --execute

upgrade:
    cargo +nightly update --breaking -Z unstable-options


npm-publish:
    npm publish --access public --registry https://registry.npmjs.org/

cargo-publish:
    cargo publish --registry crates-io 
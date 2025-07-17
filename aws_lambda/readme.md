# Cargo Lambda

## Getting started from scratch

1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Install [Docker](https://www.docker.com/)
3. Install [just](https://github.com/casey/just)

   `cargo install just`

4. Install [sea-orm-cli](https://github.com/SeaQL/sea-orm)

   `cargo install sea-orm-cli`

5. Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)

   `brew install cargo-lambda/tap/cargo-lambda`

6. Compose docker services

    `just docker-compose`

7. Create an `.env`file

   `cp .env-example .env`

8. Run migrations

    `just migrate-fresh`

9. Start the services

   `just start`

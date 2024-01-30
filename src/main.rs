#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

mod bot;
mod db;
mod migration;
mod entity;
mod tg;
mod tz;
mod err;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    bot::run().await;
}

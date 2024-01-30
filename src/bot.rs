use std::path::PathBuf;

use async_once::AsyncOnce;
use teloxide::utils::command::BotCommands;
use teloxide::prelude::*;

use crate::db::Database;
use crate::tg::TgResponse;

lazy_static! {
    /// Singleton database with pool connection
    static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(async {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let path = PathBuf::from(database_url.to_string());
        Database::new(&path)
            .await
            .unwrap_or_else(|err| panic!("Failed to connect to database {:?}: {}", path, err))
    });
}

#[derive(BotCommands, Clone)]
#[command(description = "Commands: ", rename_rule = "lowercase")]
enum Command {
    #[command(description = "show this text")]
    Help,
    #[command(description = "off")]
    Start,
}

pub async fn run() {
    pretty_env_logger::init();
    log::info!("Starting racoes_bot!");

    DATABASE.get()
        .await
        .apply_migrations()
        .await
        .expect("Failed to apply migrations");

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command_handler),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_handler(bot: Bot, cmd: Command, msg: Message) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Help => bot
            .send_message(msg.chat.id, Command::descriptions().to_string())
            .await?,
        Command::Start => bot
            .send_message(msg.chat.id, TgResponse::Hello)
            .await?,
    };
    Ok(())
}

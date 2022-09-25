use std::error::Error;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

mod telegram;
mod twitter;
mod pixiv;
mod database;

use telegram::telegram as tg;
use database::database as db;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    AddSource,
    RemoveSource,
    Settings,
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Display help text")]
    Help,
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Cancel the current operation")]
    Cancel,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting your-waifu-is-mine-rs...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(handle_main_menu))
        .branch(case![State::AddSource].endpoint(handle_add_source))
        .branch(case![State::RemoveSource].endpoint(handle_remove_source))
        .branch(case![State::Settings].endpoint(handle_settings));

    let callback_query_handler = Update::filter_callback_query().branch(
        case![State::Start].endpoint(handle_call_back),
    );

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Welcome! Please choose the operation below.")
        .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
        .await?;
    dialogue.update(State::Start).await.unwrap();
    Ok(())
}

async fn help(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn cancel(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
    dialogue.exit().await?;
    Ok(())
}

async fn handle_main_menu(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(operation) => {
            match operation.as_str() {
                "Add source" => {
                    bot.send_message(msg.chat.id, "What source do you want to add?")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::AddSource))
                        .await?;
                    dialogue.update(State::AddSource).await?;
                }
                "Remove source" => {
                    bot.send_message(msg.chat.id, "What source do you want to remove?")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::RemoveSource))
                        .await?;
                    dialogue.update(State::RemoveSource).await?;
                }
                "Settings" => {
                    bot.send_message(msg.chat.id, "What do you want to do?")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::Settings))
                        .await?;
                    dialogue.update(State::Settings).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Unknown operation. Please try again.").await?;
                    dialogue.update(State::Start).await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Invalid Operation. Return to main menu.")
                .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                .await?;
            dialogue.update(State::Start).await?;
        }
    }

    Ok(())
}

async fn handle_add_source(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(source) => {
            match source.as_str() {
                "Twitter" => {
                    bot.send_message(msg.chat.id, "Twitter is not supported yet.").await?;
                    dialogue.update(State::Start).await?;
                }
                "Pixiv" => {
                    bot.send_message(msg.chat.id, "Pixiv is not supported yet.").await?;
                    dialogue.update(State::Start).await?;
                }
                "Back" => {
                    bot.send_message(msg.chat.id, "Return to main menu.")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                        .await?;
                    dialogue.update(State::Start).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Unknown source. Please try again.").await?;
                    dialogue.update(State::AddSource).await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Invalid source. Return to main menu.")
                .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                .await?;
            dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}

async fn handle_remove_source(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(source) => {
            match source.as_str() {
                "Twitter" => {
                    bot.send_message(msg.chat.id, "Twitter is not supported yet.").await?;
                    dialogue.update(State::Start).await?;
                }
                "Pixiv" => {
                    bot.send_message(msg.chat.id, "Pixiv is not supported yet.").await?;
                    dialogue.update(State::Start).await?;
                }
                "Back" => {
                    bot.send_message(msg.chat.id, "Return to main menu.")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                        .await?;
                    dialogue.update(State::Start).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Unknown source. Please try again.").await?;
                    dialogue.update(State::RemoveSource).await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Invalid source. Return to main menu.")
                .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                .await?;
            dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}

async fn handle_settings(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(setting) => {
            match setting.as_str() {
                "List sources" => {
                    bot.send_message(msg.chat.id, "List source is not support yet.").await?;
                    dialogue.update(State::Start).await?;
                }
                "Back" => {
                    bot.send_message(msg.chat.id, "Return to main menu.")
                        .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                        .await?;
                    dialogue.update(State::Start).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Unknown setting. Please try again.").await?;
                    dialogue.update(State::Settings).await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Invalid setting. Return to main menu.")
                .reply_markup(tg::get_keyboard(tg::Keyboards::Main))
                .await?;
            dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}

async fn handle_call_back(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    Ok(())
}



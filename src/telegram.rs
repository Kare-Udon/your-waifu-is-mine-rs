pub mod telegram {
    use std::error::Error;
    use teloxide::Bot;
    use teloxide::prelude::*;
    use teloxide::types::{
        KeyboardButton, KeyboardMarkup,
    };

    pub enum Keyboards {
        Main,
        AddSource,
        RemoveSource,
        Settings,
        ListSource,
    }

    type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

    pub async fn send_keyboard(bot: AutoSend<Bot>, msg: Message, keyboard: Keyboards) -> HandlerResult {
        let keyboard = match keyboard {
            Keyboards::Main => get_keyboard(Keyboards::Main),
            Keyboards::AddSource => get_keyboard(Keyboards::AddSource),
            Keyboards::RemoveSource => get_keyboard(Keyboards::RemoveSource),
            Keyboards::Settings => get_keyboard(Keyboards::Settings),
            Keyboards::ListSource => get_keyboard(Keyboards::ListSource),
        };

        bot.send_message(msg.chat.id, "Choose an option")
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub fn get_keyboard(keyboard: Keyboards) -> KeyboardMarkup {
        match keyboard {
            Keyboards::Main => {
                KeyboardMarkup::default()
                    .append_row(vec![
                        KeyboardButton::new("Add source"),
                        KeyboardButton::new("Remove source"),
                    ])
                    .append_row(vec![
                        KeyboardButton::new("Settings"),
                    ])
                    .resize_keyboard(true)
            }
            Keyboards::AddSource => {
                KeyboardMarkup::default()
                    .append_row(vec![
                        KeyboardButton::new("Twitter"),
                        KeyboardButton::new("Pixiv"),
                    ])
                    .append_row(vec![
                        KeyboardButton::new("Back"),
                    ])
                    .resize_keyboard(true)
            }
            Keyboards::RemoveSource => {
                KeyboardMarkup::default()
                    .append_row(vec![
                        KeyboardButton::new("Twitter"),
                        KeyboardButton::new("Pixiv"),
                    ])
                    .append_row(vec![
                        KeyboardButton::new("Back"),
                    ])
                    .resize_keyboard(true)
            }
            Keyboards::Settings => {
                KeyboardMarkup::default()
                    .append_row(vec![
                        KeyboardButton::new("List sources"),
                    ])
                    .append_row(vec![
                        KeyboardButton::new("Back"),
                    ])
                    .resize_keyboard(true)
            }
            _ => {
                KeyboardMarkup::default()
                    .append_row(vec![
                        KeyboardButton::new("Back"),
                    ])
                    .resize_keyboard(true)
            }
        }
    }
}
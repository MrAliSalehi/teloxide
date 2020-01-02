use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{InlineKeyboardMarkup, InputMedia, Message},
};

/// Use this method to edit animation, audio, document, photo, or video
/// messages. If a message is a part of a message album, then it can be edited
/// only to a photo or a video. Otherwise, message type can be changed
/// arbitrarily. When inline message is edited, new file can't be uploaded. Use
/// previously uploaded file via its file_id or specify a URL. On success, if
/// the edited message was sent by the bot, the edited Message is returned,
/// otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct EditMessageMediaInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// A JSON-serialized object for a new media content of the message
    media: InputMedia,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageMediaInline {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<Message> {
        network::request_multipart(
            bot.client(),
            bot.token(),
            "editMessageMedia",
            FormBuilder::new()
                .add("inline_message_id", &self.inline_message_id)
                .add("media", &self.media)
                .add("reply_markup", &self.reply_markup)
                .build(),
        )
        .await
    }
}

impl EditMessageMediaInline {
    pub fn new<I>(inline_message_id: I, media: InputMedia) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            media,
            reply_markup: None,
        }
    }

    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
        self
    }

    pub fn media(mut self, val: InputMedia) -> Self {
        self.media = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}

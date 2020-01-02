use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

/// Use this method to send information about a venue. On success, the sent
/// Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Latitude of the venue
    latitude: f32,
    /// Longitude of the venue
    longitude: f32,
    /// Name of the venue
    title: String,
    /// Address of the venue
    address: String,
    /// Foursquare identifier of the venue
    foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    foursquare_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for SendVenue {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<Message> {
        network::request_json(
            bot.client(),
            bot.token(),
            "sendVenue",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl SendVenue {
    pub fn new<C, T, A>(
        chat_id: C,
        latitude: f32,
        longitude: f32,
        title: T,
        address: A,
    ) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        let chat_id = chat_id.into();
        let title = title.into();
        let address = address.into();
        Self {
            chat_id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn latitude(mut self, val: f32) -> Self {
        self.latitude = val;
        self
    }

    pub fn longitude(mut self, val: f32) -> Self {
        self.longitude = val;
        self
    }

    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn address<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.address = val.into();
        self
    }

    pub fn foursquare_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    pub fn foursquare_type<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_type = Some(val.into());
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}

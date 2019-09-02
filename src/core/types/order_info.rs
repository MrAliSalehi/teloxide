use serde::Deserialize;

use crate::core::types::ShippingAddress;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct OrderInfo {
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub shipping_address: ShippingAddress,
}

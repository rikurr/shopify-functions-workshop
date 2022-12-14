#![allow(dead_code)]

pub type Boolean = bool;
pub type Decimal = f64;
pub type Int = i32;
pub type ID = String;

pub mod input {
    use super::*;
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all(deserialize = "camelCase"))]
    pub struct Input {
        pub discount_node: DiscountNode,
        pub cart: Cart,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct DiscountNode {
        pub metafield: Option<Metafield>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Metafield {
        pub value: String,
    }

    #[derive(Clone, Debug, Deserialize, Default)]
    #[serde(rename_all(deserialize = "camelCase"))]
    pub struct Configuration {
        pub quantity: i64,
        pub percentage: f64,
    }
}

use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct FunctionResult {
    pub discount_application_strategy: DiscountApplicationStrategy,
    pub discounts: Vec<Discount>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
pub enum DiscountApplicationStrategy {
    First,
    Maximum,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct Discount {
    pub value: Value,
    pub targets: Vec<Target>,
    pub message: Option<String>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub enum Value {
    #[serde(rename_all(serialize = "camelCase"))]
    FixedAmount {
        #[serde_as(as = "DisplayFromStr")]
        amount: Decimal,
        applies_to_each_item: Boolean,
    },
    Percentage {
        #[serde_as(as = "DisplayFromStr")]
        value: Decimal,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub enum Target {
    ProductVariant { id: ID, quantity: Option<Int> },
}

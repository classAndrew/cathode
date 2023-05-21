use std::{hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Tower {
    pub owner: String,
    pub territory: String,
    pub health: i32,
    pub defense: i32,
    pub damage: String,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitWarAttemptC2S {
    pub name: String,
    pub uuid: String,
    pub class: String,
    pub tower: Tower,
}
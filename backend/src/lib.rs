use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Class {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub kinds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Teacher {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub teacher: usize,
    pub subject: usize,
    pub class: usize,
    pub per_week_first: u32,
    pub per_week_second: Option<u32>,
}

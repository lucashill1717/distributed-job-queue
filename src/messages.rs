use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ready {
    pub task_count: u8
}

impl Ready {
    pub fn new(task_count: u8) -> Self {
        Ready { task_count }
    }
}

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
pub enum Action {
    LinkFrequencies,
    LinkGraph,
    KeywordExtraction,
    ArticleSummarization
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: u32,
    pub data: String,
    pub actions: Vec<Action>
}

impl Task {
    pub fn new(id: u32, data: String, actions: Vec<Action>) -> Self {
        Task { id, data, actions }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Done {
    pub results: HashMap<u32, HashMap<Action, String>>
}

impl Done {
    pub fn new(results: HashMap<u32, HashMap<Action, String>>) -> Self {
        Done { results }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Ready(Ready),
    Task(Task),
    Done(Done)
}

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

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    LinkFrequencies,
    LinkGraph,
    KeywordExtraction,
    ArticleSummarization
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Job {
    pub id: u32,
    pub data: String,
    pub actions: Vec<Action>
}

impl Job {
    pub fn new(id: u32, data: String, actions: Vec<Action>) -> Self {
        Job { id, data, actions }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Done {
    pub results: Vec<String>
}

impl Done {
    pub fn new(results: Vec<String>) -> Self {
        Done { results }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Ready(Ready),
    Done(Done)
}

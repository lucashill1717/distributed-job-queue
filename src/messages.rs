use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Start {
    pub tasks: Vec<String>,
    pub source: String
}

impl Start {
    pub fn new(tasks: Vec<String>, source: String) -> Self {
        Start { tasks, source }
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

// pub enum Message {
//     Start(Start),
//     Done(Done)
// }

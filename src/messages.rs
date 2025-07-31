pub struct Start {
    tasks: Vec<String>,
    source: String
}

pub struct Done {
    results: Vec<String>
}

pub enum Message {
    Start(Start),
    Done(Done)
}

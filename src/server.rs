use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInput {
    source: String,
    actions: Vec<String>,
    hosts: Vec<String>,
}

pub fn server(user_input: UserInput) {

}

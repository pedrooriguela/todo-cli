use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tarefa {
    pub tarefa: String,
    pub estado: String,
}
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub projects: HashMap<String, Project>,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project {
    pub path: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Task {
    pub name: String,
    pub commands: Vec<Command>,
    pub environment: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Command {
    pub cmd: String,
    pub description: String,
    pub tags: Vec<String>,
}

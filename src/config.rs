use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub projects: HashMap<String, Project>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<HashMap<String, Task>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project {
    pub path: PathBuf,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Task {
    pub commands: Vec<Command>,
    pub environment: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Command {
    pub name: String,
    pub cmd: String,
    pub run_on: Option<String>,
    pub stdout_to_var: Option<String>,
}

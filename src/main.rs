use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::{self, Output};
use std::{env, path};
use shellexpand;

mod config;
mod constants;
mod exec;

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Cli {
    Init,
    Exec(ExecArgs),
    Task(TaskArgs),
}

#[derive(clap::Args)]
struct TaskArgs {
    #[arg(required = true)]
    task_name: String,
}

#[derive(clap::Args)]
struct ExecArgs {
    #[arg(required = true)]
    tag: String,

    #[arg(required=true, num_args=1..)]
    command: Vec<String>,
}

fn read_config_file(path: &PathBuf) -> config::Config {
    let f = std::fs::File::open(path).unwrap_or_else(|_| {
        let abs_path = path::absolute(path).unwrap();
        let apd = &abs_path.display();
        eprintln!("Couldn't open config file `{apd}`");
        process::exit(1);
    });

    serde_yml::from_reader(f).unwrap_or_else(|err| {
        eprintln!("Error reading config file: {err}");
        process::exit(1);
    })
}

fn main() {
    let cli = Cli::parse();
    let config_file_path = PathBuf::from("poly.yaml");

    match cli {
        Cli::Init => {
            if !config_file_path.is_file() {
                println!("Initializing Poly repos")
            }
        }
        Cli::Exec(_run_args) => {
            todo!();
        }
        Cli::Task(task_args) => {
            let config = read_config_file(&config_file_path);

            if let Some(tasks) = config.tasks {
                if let Some(task) = tasks.get(&task_args.task_name) {
                    //config.projects
                    let mut tag_project_names_map: HashMap<&String, HashSet<&String>> =
                        HashMap::new();
                    for (name, project) in &config.projects {
                        for tag in &project.tags {
                            tag_project_names_map.entry(tag).or_default().insert(name);
                        }
                    }

                    let mut env_vars: HashMap<&String, String> =
                        task.environment.as_ref().map_or(HashMap::new(), |vars| {
                            vars.iter().map(|(k, v)| (k, shellexpand::tilde(v).to_string())).collect()
                        });

                    for command in &task.commands {
                        let mut o: Option<Output> = None;
                        if let Some(run_on) = &command.run_on {
                            for project_name in
                                tag_project_names_map.entry(run_on).or_default().iter()
                            {
                                if let Some(project) = config.projects.get(*project_name) {
                                    o = Some(exec::exec_command_in_shell(
                                        &command.cmd,
                                        &project.path,
                                        &env_vars,
                                    ));
                                }
                            }
                        } else {
                            let curr_dir =
                                env::current_dir().expect("Unable to get cureent directory");
                            o = Some(exec::exec_command_in_shell(
                                &command.cmd,
                                &curr_dir,
                                &env_vars,
                            ));
                        }

                        if let (Some(var_name), Some(output)) = (command.stdout_to_var.as_ref(), o)
                        {
                            let var_value =
                                String::from_utf8_lossy(&output.stdout).trim().to_string();
                            env_vars.insert(var_name, var_value);
                        }
                    }
                } else {
                    eprintln!("No task named `{}` in the config file", task_args.task_name);
                    process::exit(1);
                }
            } else {
                eprintln!("No tasks defined in the config file");
                process::exit(1);
            }
        }
    }
}

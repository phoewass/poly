use std::path::PathBuf;
use std::path;
use std::process;
use clap::Parser;

mod config;


#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Cli {
    Init,
    Exec(ExecArgs),
    Task(TaskArgs),
}

#[derive(clap::Args)]
struct TaskArgs {
    #[arg(required=true)]
    task_name: String,
}

#[derive(clap::Args)]
struct ExecArgs {
    #[arg(required=true)]
    tag: String,

    #[arg()]
    command: String,
}

fn read_config_file(path: &PathBuf) -> config::Config {
    let f = std::fs::File::open(&path)
    .unwrap_or_else(|_| {
        let abs_path = path::absolute(&path).unwrap();
        let apd = &abs_path.display();
        eprintln!("Couldn't open config file `{apd}`");
        process::exit(1);
    });

    let c: config::Config = serde_yml::from_reader(f).unwrap_or_else(|err| {
        eprintln!("Error reading config file: {err}");
        process::exit(1);
    });
    return c
}

fn main() {
    
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let cli = Cli::parse();
    let config_file_path = PathBuf::from("poly.yaml");

    match  cli {
        Cli::Init => {
            if !config_file_path.is_file() {
                println!("Initializing Poly repos")
            }
        }
        Cli::Exec(_run_args) => {
            
        },
        Cli::Task(task_args) => {
            let config = read_config_file(&config_file_path);

            if let Some(tasks) = config.tasks {
                if let Some(task) = tasks.iter()
                                            .find(|t| t.name == task_args.task_name) {
                    todo!("Executing task {}", task.name)
                } else {
                    eprintln!("No task named `{}` in the config file", task_args.task_name);
                    process::exit(1);
                }
            } else { 
                eprintln!("No tasks defined in the config file");
                process::exit(1);
            }
        },
    }
}


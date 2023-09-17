pub mod tasks;

use clap::{Command, Parser, Arg};
use tasks::TaskManager;
use tasks::Task;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Add a new task 
    #[arg(short, long)]
    add: String,


    /// Number of times to greet
    #[arg(short, long)]
    delete: String
}

fn main() {
    let cmd = Command::new("taskm")
        .version("0.1")
        .author("luv_v")
        .about("A simple todo list CLI")
        .subcommand(Command::new("add")
                    .about("Add a task")
                    .args([
                        Arg::new("name")
                        .short('n')
                        .help("The name of the task."),
                        Arg::new("description")
                        .short('d')
                        .help("The description of the task.")
                    ])
                    );
    
}

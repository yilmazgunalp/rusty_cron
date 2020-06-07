mod rusty_cli;
use rusty_cli::{add, append};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Cronjob utility program.")]
enum RustyCron {
    #[structopt(about = "Adds a crontab file.")]
    Add {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Append {
        job: String,
    },
    Replace {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() {
    let rcron: RustyCron = RustyCron::from_args();
    match rcron {
        RustyCron::Add { file } => add(&file),
        RustyCron::Append { job } => append(job),
        _ => println!("something else"),
    }
}

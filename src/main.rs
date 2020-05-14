use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Cronjob utility program.")]
enum RustyCron {
    Add {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Append {
        job: String,
    },
}

fn main() {
    let rcron: RustyCron = RustyCron::from_args();
    match rcron {
        RustyCron::Add { file } => add(&file),
        _ => println!("something else"),
    }
}

fn add(file: &PathBuf) {
    Command::new("crontab")
        .arg(file)
        .output()
        .expect("Failed to execute command");
}

mod subcommands;
use std::path::PathBuf;
use structopt::StructOpt;
use subcommands::{add, append, replace, RcronError};

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

fn main() -> Result<(), RcronError> {
    let rcron: RustyCron = RustyCron::from_args();
    match rcron {
        RustyCron::Add { file } => add(&file)?,
        RustyCron::Replace { file } => replace(&file)?,
        RustyCron::Append { job } => append(job)?,
    }
    Ok(())
}

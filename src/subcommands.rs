use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{Error, ErrorKind, Read, Result as ioResult, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub fn add(file: &PathBuf) -> Result<(), RcronError> {
    let status = Command::new("crontab").arg(file).status().unwrap_or_else({
        |error| {
            if error.kind() == ErrorKind::NotFound {
                panic!("crontab must be installed!");
            } else {
                panic!("Problem running crontab command: {:?}", error);
            }
        }
    });

    if status.success() {
        println!("{:?} has been added to crontab!", file.file_name().unwrap());
        return Ok(());
    } else {
        panic!("crontab command failed with error decribed above this line");
    }
}

pub fn append(mut job: String) -> Result<(), RcronError> {
    // create a temp file
    let tmp_file_path: OsString = OsString::from("rcron_tmp_file.tmp");
    let mut tmp_file: fs::File =
        fs::File::create(&tmp_file_path).expect("Failed at creating tmp cron file");

    // read crontab entries and write to temp file
    let mut crontab_contents: Vec<u8> = Vec::new();
    // need to get rid of these 3 unwraps!!!!
    let bytes_read = read_crontab()
        .unwrap()
        .stdout
        .unwrap()
        .read_to_end(&mut crontab_contents)?;

    tmp_file.write(&crontab_contents[0..bytes_read])?;

    // add \n to job string and write to temp file
    job.push('\n');
    tmp_file.write(String::as_bytes(&job)).unwrap();

    // call add() to update crontab
    add(&PathBuf::from(&tmp_file_path));

    // remove temp file
    let status = Command::new("rm").arg(tmp_file_path).status();
    if let Err(_) = status {
        println!("failed to remove temp file");
    }
    Ok(())
}

fn read_crontab() -> ioResult<Child> {
    Command::new("crontab")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()
}
#[derive(Debug)]
pub struct RcronError;

impl fmt::Display for RcronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl From<Error> for RcronError {
    fn from(error: Error) -> Self {
        Self
    }
}

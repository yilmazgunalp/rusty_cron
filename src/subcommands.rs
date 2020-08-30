use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result as ioResult, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub fn replace(file: &PathBuf) -> Result<(), RcronError> {
    let status = Command::new("crontab").arg(file).status().map_err({
        |error| {
            if error.kind() == ErrorKind::NotFound {
                return RcronError {
                    msg: String::from("crontab must be installed!"),
                };
            } else {
                return RcronError {
                    msg: String::from(format!("Problem running crontab command: {:?}", error)),
                };
            }
        }
    });

    match status {
        Err(error) => return Err(error),
        Ok(exitstatus) => {
            if exitstatus.success() {
                println!(
                    "{:?} has been added to crontab!",
                    file.file_name()
                        .unwrap_or(&OsString::from("_file_not_found"))
                );
                return Ok(());
            } else {
                Err(RcronError {
                    msg: String::from("crontab command failed with error decribed above this line"),
                })
            }
        }
    }
}

pub fn append(mut job: String) -> Result<(), RcronError> {
    let (mut tmp_file, tmp_file_path) = create_tmp_file()?;
    // add \n to job string and write to temp file
    job.push('\n');
    tmp_file.write(String::as_bytes(&job)).unwrap();

    // call replace() to update crontab
    replace(&PathBuf::from(&tmp_file_path))?;

    // remove temp file
    let status = Command::new("rm").arg(tmp_file_path).status();
    if let Err(_) = status {
        println!("failed to remove temp file");
    }
    Ok(())
}

pub fn add(file: &PathBuf) -> Result<(), RcronError> {
    let (mut tmp_file, tmp_file_path) = create_tmp_file()?;
    // read the contents of the parameter file
    let mut param_file = File::open(file)?;
    let mut param_file_contents: Vec<u8> = Vec::new();
    param_file.read_to_end(&mut param_file_contents)?;

    // add the contents to temp file
    tmp_file.write(&param_file_contents[0..])?;

    // call replace() to update crontab
    replace(&PathBuf::from(&tmp_file_path))?;

    // remove temp file
    let status = Command::new("rm").arg(tmp_file_path).status();
    if let Err(_) = status {
        println!("failed to remove temp file");
    }
    Ok(())
}

fn create_tmp_file() -> Result<(File, OsString), RcronError> {
    // create a temp file
    let tmp_file_path: OsString = OsString::from("rcron_tmp_file.tmp");
    let mut tmp_file: fs::File =
        fs::File::create(&tmp_file_path).expect("Failed at creating tmp cron file");

    // read crontab entries and write to temp file
    let mut crontab_contents: Vec<u8> = Vec::new();
    let bytes_read = read_crontab()?
        .stdout
        .ok_or(RcronError::default())?
        .read_to_end(&mut crontab_contents)?;

    tmp_file.write(&crontab_contents[0..bytes_read])?;
    Ok((tmp_file, tmp_file_path))
}

fn read_crontab() -> ioResult<Child> {
    Command::new("crontab")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()
}
#[derive(Debug, Default)]
pub struct RcronError {
    pub msg: String,
}

impl fmt::Display for RcronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<Error> for RcronError {
    fn from(error: Error) -> Self {
        RcronError {
            msg: error.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn add_crontab_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "* * * * 5 echo hello")?;
        let mut cmd = Command::cargo_bin("rusty_cron")?;
        cmd.arg("add").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("has been added"));

        Ok(())
    }

    #[test]
    fn add_crontab_file_with_with_invalid_format() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        // invalid crontab entry
        writeln!(file, "* * * *5")?;
        let mut cmd = Command::cargo_bin("rusty_cron")?;
        cmd.arg("add").arg(file.path());
        cmd.assert().failure();

        Ok(())
    }

    #[test]
    fn append_crontab() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rusty_cron")?;
        cmd.arg("append").arg("* * * 5 * echo hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("has been added"));

        Ok(())
    }

    #[test]
    fn replace_crontab() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "* * * * 5 echo hello")?;
        let mut cmd = Command::cargo_bin("rusty_cron")?;
        cmd.arg("replace").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("has been added"));

        Ok(())
    }
}

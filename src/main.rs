use std::env;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod strs;
/* connect to github, store config data and user token somewhere safe */
/* program can have one command: search*/
/* search command can have one option: reponame */
/* retrieve the details of the repo if it exists or just show error message */
// #[derive(Debug)]
// enum Commands {
//     /// Lists repos with multi select option in pages
//     List(String),
//     /// Delete a particular repo ( pass name )
//     Delete(String),
//     /// Same as List, but only displays the result, can't select
//     Search(String)
// }
// #[derive(Debug)]
// enum FileNamesFlag {
//     FileNameAsString(String),
//     FileNameAsRegex(String),
// }
// #[derive(Debug)]
// struct SearchState {
//    /// filter repos with certain number of files
//     file_count: i32,
//     /// filter with name
//     file_names: Vec<String>
//
// }

static COMMAND_HELPER_MSG: &str = "
    GIT HELPER CLI

Usage: xxx [OPTIONS] [COMMAND] [OPTIONS]

Options:
  -V, --version     Print version info and exit
      --verbose     Use verbose output
      --quite       Do not print messages
  -h, --help        Print help

Commands:
  list    [filter conditions] list repos as selectable list
  delete  <name|id>           delete the repos/repos with the id/name
  search  <nmae>              similar to list, but not select

    ";

static LIST_HELPER_MSG: &str = "
    LIST COMMAND

Usage: xxx list [OPTIONS]

Options:
  --name   name of repo
  --files  filenames array
";

static DELETE_HELPER_MSG: &str = "
    DELETE COMMAND
";

static SEARCH_HELPER_MSG: &str = "
    SEARCH COMMAND
";

fn handle_list_command<T: Iterator<Item = String>>(cmd: T) -> &'static str {
    for (_i, v) in cmd.enumerate() {
        match v.as_str() {
            "-h" | "--help" => {
                println!("{}", LIST_HELPER_MSG);
                return LIST_HELPER_MSG;
            }
            _ => {
                return "";
            }
        }
    }
    ""
}

fn handle_delete_command<T: Iterator<Item = String>>(cmd: T) -> &'static str {
    for (_i, v) in cmd.enumerate() {
        match v.as_str() {
            "-h" | "--help" => {
                println!("{}", DELETE_HELPER_MSG);
                return DELETE_HELPER_MSG;
            }
            _ => {
                return "";
            }
        }
    }
    ""
}

fn handle_search_command<T: Iterator<Item = String>>(cmd: T) -> &'static str {
    for (_i, v) in cmd.enumerate() {
        match v.as_str() {
            "-h" | "--help" => {
                println!("{}", SEARCH_HELPER_MSG);
                return SEARCH_HELPER_MSG;
            }
            _ => {
                return "";
            }
        }
    }
    ""
}

#[cfg(test)]
mod tests {
    use std::env::{self, VarError};

    use crate::{
        get_config_two, handle_delete_command, handle_list_command, handle_search_command,
        strs::INVALID_HOME_PATH, GetConfigErrorKind, DELETE_HELPER_MSG, LIST_HELPER_MSG,
        SEARCH_HELPER_MSG,
    };

    #[test]
    fn should_show_help_on_verbose() {
        let help_string_verbose = String::from("--help");

        let list = handle_list_command([help_string_verbose.clone()].into_iter());
        let search = handle_search_command([help_string_verbose.clone()].into_iter());
        let delete = handle_delete_command([help_string_verbose.clone()].into_iter());

        assert_eq!(list, LIST_HELPER_MSG);
        assert_eq!(search, SEARCH_HELPER_MSG);
        assert_eq!(delete, DELETE_HELPER_MSG);
    }

    #[test]
    fn should_show_help_on_short_help() {
        let help_string_short = String::from("-h");

        let list = handle_list_command([help_string_short.clone()].into_iter());
        let search = handle_search_command([help_string_short.clone()].into_iter());
        let delete = handle_delete_command([help_string_short.clone()].into_iter());

        assert_eq!(list, LIST_HELPER_MSG);
        assert_eq!(search, SEARCH_HELPER_MSG);
        assert_eq!(delete, DELETE_HELPER_MSG);
    }

    #[test]
    fn should_show_env_var_error() {
        let s = get_config_two("lopplslslslslsl").unwrap_err();
        assert_eq!(
            s.kind(),
            &GetConfigErrorKind::VarError(VarError::NotPresent)
        )
    }

    #[test]
    fn should_show_unexpected_home_path_error() {
        env::set_var("testxxxapphome", "/n/b/c");
        let s = get_config_two("testxxxapphome").unwrap_err();
        assert_eq!(
            s.kind(),
            &GetConfigErrorKind::WrongHomePath(INVALID_HOME_PATH)
        );
        env::remove_var("testxxxapphome")
    }
}

enum Oses {
    Windows(String),
    Linux(String),
}

#[derive(Debug, PartialEq)]
enum GetConfigErrorKind {
    VarError(env::VarError),
    CreateError(std::io::ErrorKind),
    WrongHomePath(&'static str),
    SerdeError(serde_json::error::Category),
}

#[derive(Debug)]
struct GetConfigError {
    kind: GetConfigErrorKind,
}

impl GetConfigError {
    fn kind(&self) -> &GetConfigErrorKind {
        &self.kind
    }
}

impl Display for GetConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            GetConfigErrorKind::VarError(e) => write!(
                f,
                "I tried getting HOME env variable and got the following error\nVarError:{e}"
            ),
            GetConfigErrorKind::CreateError(e) => write!(
                f,
                "I tried creating a folder in this path and got this error\nIoError: {e}"
            ),
            GetConfigErrorKind::WrongHomePath(e) => write!(f, "{e}"),
            GetConfigErrorKind::SerdeError(e) => match e {
                serde_json::error::Category::Eof => {
                    return write!(f, "config json eof error");
                }
                serde_json::error::Category::Io => {
                    return write!(f, "config json IO error");
                }
                _ => {
                    return write!(f, "bad");
                }
            },
        }
    }
}

impl From<io::Error> for GetConfigError {
    // from is useful to use ?, which calls from to convert err from io::Error to my type
    fn from(err: io::Error) -> Self {
        GetConfigError {
            kind: GetConfigErrorKind::CreateError(err.kind()),
        }
    }
}

impl From<env::VarError> for GetConfigError {
    fn from(err: env::VarError) -> Self {
        GetConfigError {
            kind: GetConfigErrorKind::VarError(err),
        }
    }
}

impl From<&'static str> for GetConfigError {
    fn from(err: &'static str) -> Self {
        GetConfigError {
            kind: GetConfigErrorKind::WrongHomePath(err),
        }
    }
}

impl From<serde_json::Error> for GetConfigError {
    fn from(err: serde_json::Error) -> Self {
        GetConfigError {
            kind: GetConfigErrorKind::SerdeError(err.classify()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    git_token: String,
}

fn get_path_to_config_file() -> Result<String, GetConfigError> {
    // let app_path_folder_path: Path;
    match env::consts::OS {
        "windows" => {
            /*
             * If user is in linux platform, store the git token in home
             * expect a return of "C:\Users\<username>\AppData\Roaming"
             */
            if let Some(app_data_path) = env::var_os(strs::WINDOWS_APP_PATH_ENV) {
                return Ok(app_data_path.into_string().unwrap());
            }
            Err(strs::COULDNT_READ_APPDATA.into())
        }
        "linux" => {
            /*
             * If user is in linux platform, store the git token in home
             * expect a return of "/home"
             */
            if let Some(_p) = env::var_os(strs::LINUX_APP_PATH_ENV) {
                return Ok(_p.into_string().unwrap());
            };
            Err(strs::COULDNT_READ_HOME.into())
        }
        _ => {
            println!("No, OS is not supported!");
            Err(strs::COULDNT_READ_APPDATA.into())
        }
    }
}

fn parse_json(arg: std::fs::File) -> Result<AppConfig, GetConfigError> {
    let buff = std::io::BufReader::new(arg);
    let json_r: Result<AppConfig, serde_json::Error> = serde_json::from_reader(buff);
    let Ok(json) = json_r else {
        return Err(json_r.err().unwrap().into());
    };
    Ok(json)
}

// TODO - learn and implement builder pattern here
fn main() {
    // skip file name
    let mut cmd = env::args().skip(1);
    let command = cmd.next().unwrap_or(String::from(""));

    // get the config folder path for os
    let config_file_path = get_path_to_config_file().map_or_else(
        |e| {
            println!("{}", e);
            return String::new();
        },
        |v| return v,
    );

    // empty string, from error, so stop program
    if config_file_path.len() == 0 {
        return;
    }

    // check if folder and file exists
    let config_folder = Path::new(&config_file_path).join(strs::APP_FOLDER_NAME);
    let config_path = config_folder.join(strs::APP_CONFIG_FILE_NAME);

    // check if folder and file exists
    let config_file_exists = config_path.try_exists().map_or_else(
        |e| {
            println!("{:?}", e);
            false
        },
        |v| v,
    );

    // if doesn't exist create new
    if !config_file_exists {
        let config_folder_created = std::fs::create_dir_all(&config_path).map_or_else(
            |e| {
                println!("{:?}", e);
                false
            },
            |_| true,
        );
        if !config_folder_created {
            println!("Error while trying to create config folder");
            return;
        }
        // to prevent serde eof error, we will fill it with some value
        // since it was okay to create, writing wont be an issue
        let config_initial_written = std::fs::write(&config_path, "{\"initial\":\"true\"}")
            .map_or_else(
                |e| {
                    println!("{:?}", e);
                    false
                },
                |v| {
                    println!("{:?}", v);
                    true
                },
            );
        if !config_initial_written {
            println!("Error trying to write to config.json");
            return;
        }
    }

    // at this point, the config.json surely exists
    let json = parse_json(std::fs::File::open(&config_path).unwrap()).unwrap();

    println!("{:?}", json);
    match command.as_str() {
        "list" => {
            handle_list_command(cmd);
        }
        "delete" => {
            handle_delete_command(cmd);
        }
        "search" => {
            handle_search_command(cmd);
        }
        "-h" | "--help" => {
            println!("{}", COMMAND_HELPER_MSG);
        }
        _ => println!("{}", COMMAND_HELPER_MSG),
    }
}

use std::path::Path;
use std::io;
use std::env;
use std::error::Error;
use std::fmt::{Formatter,Display,self};
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


fn handle_list_command<T:Iterator<Item = String >>(cmd:T)->&'static str{
    for (_i,v) in cmd.enumerate() {
        match v.as_str() {
            "-h"|"--help"=>{
                println!("{}",LIST_HELPER_MSG);
                return LIST_HELPER_MSG;
            },
            _ => {
                return "";
            },
        }
    }
    ""
}
fn handle_delete_command<T: Iterator<Item = String >>(cmd:T)->&'static str{
    for (_i,v) in cmd.enumerate() {
        match v.as_str() {
            "-h"|"--help"=>{
                println!("{}",DELETE_HELPER_MSG);
                return DELETE_HELPER_MSG;
            },
            _ => {
                return "";
            },
        }
    }
    ""
}

fn handle_search_command<T:Iterator<Item = String >>(cmd:T)->&'static str{
    for (_i,v) in cmd.enumerate() {
        match v.as_str() {
            "-h"|"--help"=>{
                println!("{}",SEARCH_HELPER_MSG);
                return SEARCH_HELPER_MSG;
            },
            _ => {
                return "";
            },
        }
    }
    ""
}

#[cfg(test)]
mod tests {
    use std::{io, env::VarError};

    use crate::{handle_list_command, LIST_HELPER_MSG, handle_search_command, handle_delete_command, SEARCH_HELPER_MSG, DELETE_HELPER_MSG, get_config, NO_HOME_ENV_VAR, GetConfigError, GetConfigErrorKind};

    #[test]
    fn should_show_help_on_verbose() {
        let help_string_verbose = String::from("--help");

        let list = handle_list_command([help_string_verbose.clone()].into_iter());
        let search = handle_search_command([help_string_verbose.clone()].into_iter());
        let delete = handle_delete_command([help_string_verbose.clone()].into_iter());

        assert_eq!(list,LIST_HELPER_MSG);
        assert_eq!(search,SEARCH_HELPER_MSG);
        assert_eq!(delete,DELETE_HELPER_MSG);
    }

    #[test]
    fn should_show_help_on_short_help() {
        let help_string_short = String::from("-h");

        let list = handle_list_command([help_string_short.clone()].into_iter());
        let search = handle_search_command([help_string_short.clone()].into_iter());
        let delete = handle_delete_command([help_string_short.clone()].into_iter());

        assert_eq!(list,LIST_HELPER_MSG);
        assert_eq!(search,SEARCH_HELPER_MSG);
        assert_eq!(delete,DELETE_HELPER_MSG);
    }


    #[test]
    fn should_show_env_var_error(){
        let s = get_config("lopplslslslslsl").unwrap_err();
        assert_eq!(s.kind(),&GetConfigErrorKind::VarError(VarError::NotPresent))
    }

}

#[derive(Debug, PartialEq)]
enum GetConfigErrorKind {
    VarError(env::VarError),
    CreateError(io::ErrorKind),
    WrongHomePath(&'static str)
}

#[derive(Debug)]
struct  GetConfigError {
    kind: GetConfigErrorKind
}

// impl Error for GetConfigError {}

impl GetConfigError {
  pub  fn kind(&self)->&GetConfigErrorKind {
        &self.kind
    }
}

static NO_HOME_ENV_VAR: &str = "HOME variable not set in env or error reading it";
impl Display for GetConfigError {
    fn fmt (&self, f: &mut Formatter)->fmt::Result{
        match &self.kind {
            GetConfigErrorKind::VarError(e) => write!(f,"I tried getting HOME env variable and got the following error\nVarError:{e}"),
            GetConfigErrorKind::CreateError(e) => write!(f,"I tried creating a folder in this path and got this error\nIoError: {e}"),
            GetConfigErrorKind::WrongHomePath(e) => write!(f,"{e}")
        }
    }
}   

impl From<io::ErrorKind> for GetConfigError {
    // from is useful to use ?, which calls from to convert err from io::Error to my type
    fn from(err:io::ErrorKind) -> Self {
        GetConfigError { kind: GetConfigErrorKind::CreateError(err) }
    }
}

impl From<env::VarError> for GetConfigError {
    fn from(err: env::VarError) -> Self {
        GetConfigError { kind: GetConfigErrorKind::VarError(err) }
    }
}

impl From<&'static str> for GetConfigError {
    fn from(err:&'static str) -> Self {
        GetConfigError { kind: GetConfigErrorKind::WrongHomePath(err) }
    }
}

#[derive(Debug)]
struct AppConfig {
    git_token:String,
}

static INVALID_HOME_PATH: &'static str = "Expected home to be /home/<something>";

fn get_config(key:&str)->Result<AppConfig,GetConfigError>{
    let home_path_from_env = env::var(key)?;
    println!("{}",home_path_from_env);
    // if let Err(x) = home_path_from_env {
        // return Err(NO_HOME_ENV_VAR);
    // }
    if !Path::new(&home_path_from_env).starts_with("/home/") {
       return Err(INVALID_HOME_PATH.into())
    } 
    // let path_buf = PathBuf::new().join(v).join(".abc").join("xxxApp");
    // fs::read(path_buf);
    // let x:Result<(), io::Error> = create_dir_all(path_buf)
    // if let Err(err) = create_dir_all(path_buf) {
    // }
  Ok(AppConfig { git_token: String::from("45") })
            
}

fn main() {
    // skip file name
    let mut cmd = env::args().skip(1);
    let command = cmd.next().unwrap_or(String::from(""));
    let x = get_config("HOME");
    if let Err(s) = x {
        println!("{}",s);
        return
    }
    match command.as_str() {
        "list"=>{
            handle_list_command(cmd);
        },
        "delete"=>{
            handle_delete_command(cmd);
        },
        "search"=>{
            handle_search_command(cmd);
        },
        "-h"|"--help"=>{
            println!("{}",COMMAND_HELPER_MSG);
        },
        _=> println!("{}",COMMAND_HELPER_MSG),
    }
}

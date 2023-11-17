
use std::env::{self, Args};
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
    GIT HELPER CLI\n
    list  [filter conditions] list repos as selectable list\n
    delete \n
    search \n

    ";
static LIST_HELPER_MSG: &str = "";
static DELETE_HELPER_MSG: &str = "";
static SEARCH_HELPER_MSG: &str = "";

fn handle_list_command(cmd:Args){
    println!("command is list");
    for (i,v) in cmd.enumerate() {
        println!("{v}")
    }
}
fn handle_delete_command(cmd:Args){
   println!("command is delete");
}
fn handle_search_command(cmd:Args){
   println!("command is search");
}
fn main() {
    // let list =Commands::List(String::from("list"));
    let mut cmd = env::args();
    let _file_name = cmd.next().unwrap_or(String::from(""));
    let command = cmd.next().unwrap_or(String::from(""));
    match command.as_str() {
        "list"=>{
            handle_list_command(cmd)
        },
        "delete"=>{
            handle_delete_command(cmd)
        },
        "search"=>{
            handle_search_command(cmd)
        },
        "-h"|"--help"=>{
            println!("{}",COMMAND_HELPER_MSG);
        },
        _=> panic!("command {command} is not available")
    }
}

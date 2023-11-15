
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
fn handle_list_command(cmd:Args){
    for (i,v) in cmd.enumerate() {
        println!("{v}")
    }
}
fn handle_delete_command(cmd:Args){
}
fn handle_search_command(cmd:Args){
}
fn main() {
    // let list =Commands::List(String::from("list"));
    let mut cmd = env::args();
    let _file_name = cmd.next().unwrap_or(String::from(""));
    let command = cmd.next().unwrap_or(String::from(""));
    match command.as_str() {
        "list"=>{
            println!("command is list");
            handle_list_command(cmd)
        },
        "delete"=>{
            println!("command is delete");
        },
        "search"=>{
            println!("command is search");
        },
        _=> panic!("command {command} is not available")
    }
}

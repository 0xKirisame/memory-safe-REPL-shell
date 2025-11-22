#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn locate_exec(path_env: &str, argument: &str) -> Option<String> {

    let iterator = std::env::split_paths(&path_env);
    for mut file in iterator {

        file.push(&argument);
        if let Ok(meta) = std::fs::metadata(&file) {
            let mode = meta.permissions().mode();
            if (mode & 0o111) != 0 {
                return Some(file.to_string_lossy().into_owned());
            }
            else {
                continue;
            }
        }
    }
    None
}

fn main() {
    loop{

    //initializes the path
    let path = std::env::var("PATH").unwrap_or_default();

    //print $ from the buffer
     print!("$ ");
     io::stdout().flush().unwrap();
    
    //takes command and arguments and reject invalid commands
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("error: failed to read line");
    
    let clean_command = command.trim();
    if clean_command.is_empty() {
    continue;
    }
    let arguments: Vec<&str> = clean_command.split_whitespace().collect(); 

    //shell bultin commands
    let builtins = ["exit", "echo", "type", "pwd"];

    //match shell commands if found
    match arguments[0] { 
        "exit" => std::process::exit(arguments.get(1).map_or(0, |v| v.parse::<i32>().unwrap_or(0))),
        "echo" => println!("{}", arguments.get(1..).map_or(String::new(), |v| v.join(" "))),
        "pwd" => println!("{}", std::env::current_dir().unwrap().to_string_lossy()),
        "type" => if let Some(target) = arguments.get(1) {

                     if builtins.contains(target){
                        println!("{} is a shell builtin", target);
                    }
                     else if let Some(path) = locate_exec(&path, target) {
                         println!("{} is {}", target, path);
                     }

                     else {
                        println!("{}: not found", target);
                    }
                }

                else {
                    println!("not found");
                }
        _ => {
            let cmd = std::process::Command::new(arguments[0]).args(&arguments[1..]).spawn();

            if let Ok(mut child) = cmd { 
                child.wait().unwrap(); 
            }

             else {
             print!("{}: command not found\n", clean_command);
         }
        }
    }
    }
}       

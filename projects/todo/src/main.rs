use std::io;

fn main() {
    let mut usr_input = String::new();
    println!("Welcome to todo.rs, Enter a command: ");
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read line");
    let usr_cmd_vec: Vec<&str> = usr_input.split_whitespace().collect();
    if usr_cmd_vec[1] == "-new" {
        println!("Todo Info:\n  Name: {:?}", usr_cmd_vec[2]);
    }
    // do error handling with try-catch
    // desired features:
    // sort todos
    // create todos with deadline
    // add additional info to todo
    // simple syntax:
    // ex: todo -new TODO_1 -deadline 2023/1/1 -add_info https://google.com, https://youtube.com
    // use json file format
    // list todos
    // remind what todos have to be done that has deadline
    // ex: 2022/12/31 do something
    // auto remove todos after deadline date
}

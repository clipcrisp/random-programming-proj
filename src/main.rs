/*
* Two lists: one with subjects, one with the languages I'm
* interested in.
*/

use std::fs;
use rand::Rng;

fn main() {
    let languages = vec![
        "Rust",
        "Go",
        "Python",
        "C",
        "Zig",
        "C++",
        "Haskell",
        "Java",
        "Javascript",
        "Clojure",
        "Common Lisp",
        "Ruby",
        "Lua",
        "C#"
    ];

    let projects = vec![
        "Text Editor",
        "ToDo List",
        "Drawing",
        "Calculator",
        "Stop Watch",
        "Pong",
        "Snake",
        "Checkers"
    ];

    let mut rng = rand::thread_rng();
    let lang_numb = rng.gen_range(0..languages.len()-1);
    let proj_numb = rng.gen_range(0..projects.len()-1);
    let result = format!("{} ~ {}", languages[lang_numb], projects[proj_numb]);
    
    fs::write("SingleResult", result).expect("Unable to write file");
}   

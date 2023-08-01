//use std::{env, fs};

pub fn menu()  { 

    println!("Command Options:");
    println!("pwd    --     PRINT WORKING DIRECTORY");
    println!("ls     --     LISTS FILES [ flags: a for hidden files, r for recursive sub-directory search ]");
    println!("cat    --     WRITES FILE TO STDOUT");
    println!("grep   --     FINDS PHRASES IN FILES [ required: [pattern] [file] ]");
        
}

pub fn get_command() -> (String, Vec<String>) { 

    let mut flags : Vec<String> = Vec::new();
    let mut read_flags : String = String::new();
    let mut command : String = String::new();

    let _ = std::io::stdin().read_line(&mut command).unwrap();

    match command.as_str() { 

        "pwd" => println!("PRINTING WORKING DIRECTORY..."),
        "ls" => { 

            println!("LISTING FILES... input flags to be used: ");
            let _ = std::io::stdin().read_line(&mut read_flags).unwrap();
            flags = read_flags.split_whitespace().map(|s| s.to_string()).collect();

        },
        "cat" => println!("WRITING FILE TO STDOUT..."),
        "grep" => { 

            println!("FINDING PATTERN... provide input in the form 'pattern', filename: ");
            let _ = std::io::stdin().read_line(&mut read_flags).unwrap();
            flags = read_flags.split(",").map(|s| s.to_string()).collect();
        }
        _ => panic!("No option chosen!"),

    };

    let returned_flags = flags.clone();

    return (command, returned_flags);

}
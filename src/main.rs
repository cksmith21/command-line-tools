mod utils;

mod processor;
use crate::processor::start;

/* 
    Commands I want to build: 

    1. pwd (print working directory)
    2. ls (list files in a system) [ with flags -a for hidden files, -R for subdirectories, and -lh for file sizes ]
    3. cat (writes write output to stdout)
    4. grep [ pattern ] [ file ]

    MAYBE: 

    7. ping (checking whether a network or server is reachable)
    8. wget (download files from the internet)


*/ 

fn main() {
    println!("Hello, world!");
}

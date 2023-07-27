use clap::Parser; 

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]

struct Args { 

    #[clap(short, long, action)]
    pub grep : bool,

    #[clap(short, long, action)]
    pub cat : bool,

    #[clap(short, long, action)]
    pub ls : bool 

}

pub fn getargs() -> Args { 

    return Args::parse();

}
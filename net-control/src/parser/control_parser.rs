use std::{sync::{Mutex, Once}, mem::MaybeUninit};
use clap::{Command, ArgMatches};


trait Parser {
    fn parse_string_vector(data: Vec<String>) -> Option<clap::error::Result<ArgMatches>> { None }
    fn parse_string (data: String) -> Option<clap::error::Result<ArgMatches>> { None }
}

struct CLIParser{
}

impl CLIParser {
    pub fn get_cli_parser() -> &'static Mutex<Self> {
        static mut SINGLETON: MaybeUninit<Mutex<CLIParser>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        unsafe {
            ONCE.call_once(|| {
                SINGLETON.as_mut_ptr().write(Mutex::new(CLIParser::new()));
            });
        
            &*SINGLETON.as_ptr()
        }
    }

    fn new() -> Self {
        CLIParser{}
    }
}

impl Parser for CLIParser {

}
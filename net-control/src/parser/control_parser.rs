use std::{sync::{Mutex, Once}, mem::MaybeUninit, collections::HashMap};
use clap::{Command, ArgMatches};


trait Parser {
    fn parse_string_vector(data: Vec<String>) -> Option<clap::error::Result<ArgMatches>> { None }
    fn parse_string (data: String) -> Option<clap::error::Result<ArgMatches>> { None }
}

struct CLIParser{
    config: ParserConfig
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
        CLIParser{
            config: ParserConfig::new()
        }
    }
}

impl Parser for CLIParser {

}

struct ParserConfig {
    commands: HashMap<String, Command>
}

impl ParserConfig {
    fn new() -> Self {
        ParserConfig { commands: HashMap::new() }
    }

    fn reconfigure() {
        todo!("Currently unable to configure the parser")
    }
}
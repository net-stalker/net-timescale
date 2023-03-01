use std::{sync::{Mutex, Once}, mem::MaybeUninit, collections::HashMap, error::Error, fmt::Debug};
use clap::{Command, ArgMatches, command, error::ErrorKind};


trait Parser {
    fn parse_string_vector(&self, data: &Vec<&str>) -> Option<clap::error::Result<ArgMatches>> { None }
    fn parse_string (&self, data: &str) -> Option<clap::error::Result<ArgMatches>> { None }
}

#[derive(Debug)]
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
    fn parse_string_vector(&self, data: &Vec<&str>) -> Option<clap::error::Result<ArgMatches>> {
        let service_name = data[0];
        let mut service_command: Command;

        match self.config.commands.get(&service_name) {
            Some(command) => service_command = command.clone(),
            None => return Some(Err(clap::error::Error::new(ErrorKind::InvalidValue)))
        }

        Some(service_command.try_get_matches_from_mut(data))
    }
}

#[derive(Debug)]
struct ParserConfig {
    commands: HashMap<&'static str, Command>
}

impl ParserConfig {
    fn new() -> Self {
        ParserConfig { commands: HashMap::new() }
    }

    fn reconfigure(&self, ) {
        todo!("Currently unable to configure the parser")
    }

    fn add_command (&mut self, service_name: &'static str, service_parser: Command ) {
        self.commands.insert(service_name, service_parser);
    }
}

//TODO: Insert command for the CLI
impl Default for ParserConfig {
    fn default() -> Self {
        let config = ParserConfig::new();
        config
    }
}


#[cfg(test)]
mod tests {
    use clap::Command;

    use super::CLIParser;
    use super::ParserConfig;

    #[test]
    fn expect_parser_to_be_singleton()
    {
        let first_parser = CLIParser::get_cli_parser();
        let second_parser = CLIParser::get_cli_parser();

        assert!(std::ptr::eq(first_parser, second_parser));
    }

    #[test]
    fn parser_config_command_addition()
    {
        let mut config = ParserConfig::new();
        assert!(config.commands.is_empty());

        config.add_command("SomeService", Command::new("SomeService"));
        assert_eq!(config.commands.len(), 1);
    }
}
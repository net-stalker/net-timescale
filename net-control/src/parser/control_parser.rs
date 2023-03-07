use std::{sync::{Mutex, Once}, mem::MaybeUninit, collections::HashMap, error::Error, fmt::Debug};
use clap::{Command, ArgMatches, command, error::ErrorKind};

#[allow(unused)]
//TODO: 
// Think on changing the HashMap<&str,...> to some abstract naming 
// Add Error type
trait CommandHolder{
    fn get_commands (&self) -> Option<Result<&HashMap<&str, Command>, &str>> { None }
}
#[allow(unused)]
trait Parser{
    fn parse_data_vector <T, D> (t: &T, d: &Vec<D>) -> Option<clap::error::Result<ArgMatches>>
    where 
        T : CommandHolder, 
        D : ToString
    { None }
    
    fn parse_data <T, D> (d: &D, ) -> Option<clap::error::Result<ArgMatches>> 
    where 
        T: CommandHolder,
        D : ToString
    { None }
}

struct CLIParser{}

impl Parser for CLIParser {
    fn parse_data_vector <T, D> (command_holder: &T, data: &Vec<D>) -> Option<clap::error::Result<ArgMatches>>
    where 
        T : CommandHolder, 
        D : ToString 
    {
        let data_vector: Vec<String> = data.iter().map(|x| x.to_string()).collect();
        let service_command: Command;

        let service_name: &str = data_vector[0].as_str();
        let service_commands = command_holder.get_commands().unwrap().unwrap();

        match service_commands.get(&service_name) {
            Some(command) => service_command = command.clone(),
            None => return Some(Err(clap::error::Error::new(ErrorKind::InvalidValue)))
        }

        Some(service_command.try_get_matches_from(data_vector))
    }
}

#[derive(Debug)]
struct ParserConfig {
    commands: HashMap<&'static str, Command>
}

impl CommandHolder for ParserConfig {
    fn get_commands (&self) -> Option<Result<&HashMap<&str, Command>, &str>> {
        Some(Ok(&self.commands))
    }
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
    use super::Parser;
    use super::ParserConfig;

    #[test]
    fn parser_config_command_addition()
    {
        let mut config = ParserConfig::new();
        assert!(config.commands.is_empty());

        config.add_command("SomeService", Command::new("SomeService"));
        assert_eq!(config.commands.len(), 1);
    }

    #[test]
    fn should_return_error_on_parsing_with_empty_config()
    {
        let config = ParserConfig::new();

        let data_to_parse: Vec<&str> = vec!["some", "args", "to", "parse"];

        assert!(CLIParser::parse_data_vector(&config, &data_to_parse).unwrap().is_err())
    } 

    #[test]
    fn shoud_parse_clap_example()
    {
        let mut config = ParserConfig::new();

        let service_name = "foo";
        let service_command = Command::new(service_name)
        .arg(
            clap::Arg::new("bar").short('b').action(clap::ArgAction::SetTrue)
        );

        config.add_command(service_name, service_command);

        let data_to_parse: Vec<&str> = vec!["foo", "-b"];

        assert!(CLIParser::parse_data_vector(&config, &data_to_parse).unwrap().is_ok())
    }
}
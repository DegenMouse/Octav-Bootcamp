use std::env::Args;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::dns::{get_ips_for_dns_list, get_ips_for_handshake};
use crate::consts::{DNS_LIST_MAINNET, DNS_LIST_TESTNET};
use crate::handshake::run_handshake;

//enum comenzi posibil 
#[derive(Debug, Clone)]
pub enum CliCommands {
    HelpCommand,
    ListDNSCommand,
    ListIPSCommand,
    HandshakeCommand,
}

//enum argumente posibile
#[derive(Debug, Clone)]
pub enum CliArguments {
    None,
    DNSList((u32, Vec<String>)),
    IPList((bool, (bool, String), Option<u32>, Vec<SocketAddr>)),
}

#[derive(Debug, Clone)]
pub enum CliNetwork {
    Mainnet,
    Testnet,
}

//enum cu posibile erori
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Command not specified or invalid! Run 'cargo run -- -h' for more help with commands.")]
    InvalidCommand,
    #[error("Invalid DNS list passed as argument! Run 'cargo run -- -h' for more help with commands.")]
    InvalidDNSList,
    #[error("Invalid IP list passed as argument! Run 'cargo run -- -h' for more help with commands.")]
    InvalidIPList,
    #[error("Invalid or no number passed as argument! Run 'cargo run -- -h' for more help with commands.")]
    InvalidNumber,
    #[error("Couldn't fetch IPs for any of the parsed DNS seeds.")]
    ErrorFetchingIPs,
    #[error("The flag you provided is invalid.")]
    InvalidFlag,
}


#[derive(Debug, Clone)]
pub struct Config {
    pub command: CliCommands,
    pub args: CliArguments,
    pub network: CliNetwork,
}

impl Config {
    pub fn from(mut cli_arguments: Args) -> Result<Config, ConfigError> {
        cli_arguments.next();

        let command = match cli_arguments.next() {
            Some(arg) => match_command(&arg)?,
            None => return Err(ConfigError::InvalidCommand),
        };

        let (args, network) = match_arguments(&command, cli_arguments)?;

        let config = Config {
            command,
            args,
            network,
        };

        Ok(config)
    }
}

fn match_command(command: &str) -> Result<CliCommands, ConfigError> {
    match command.to_lowercase().trim() {
        "-h" => Ok(CliCommands::HelpCommand),
        "-dns" => Ok(CliCommands::ListDNSCommand),
        "-ips" => Ok(CliCommands::ListIPSCommand),
        "-handshake" => Ok(CliCommands::HandshakeCommand),
        _ => Err(ConfigError::InvalidCommand),
    }
}

fn match_arguments(command: &CliCommands, mut cli_arguments: Args) -> Result<(CliArguments, CliNetwork), ConfigError> {
    let mut arguments = (CliArguments::None, CliNetwork::Mainnet);

    match command {
        CliCommands::HelpCommand => {},
        CliCommands::ListDNSCommand => {
            if let Some(flag) = cli_arguments.next() {
                arguments.1 = match flag.as_str() {
                    "-t" => CliNetwork::Testnet,
                    _ => return Err(ConfigError::InvalidFlag),
                };
            }
        },
        CliCommands::ListIPSCommand => {
            let mut dns_args = vec![];
            let mut testnet = CliNetwork::Mainnet;
            let mut number = 0;

            while let Some(arg) = cli_arguments.next() {
                match arg.as_str() {
                    "-t" => testnet = CliNetwork::Testnet,
                    _ if number == 0 => {
                        if let Ok(nr) = arg.parse::<u32>() {
                            number = nr;
                        } else {
                            return Err(ConfigError::InvalidNumber);
                        }
                    }
                    _ if DNS_LIST_MAINNET.contains(&arg.as_str()) || DNS_LIST_TESTNET.contains(&arg.as_str()) => dns_args.push(arg),
                    _ => println!("Invalid DNS seed: {}", arg),
                }
            }

            if dns_args.is_empty() {
                return Err(ConfigError::InvalidDNSList);
            } else {
                arguments = (CliArguments::DNSList((number, dns_args)), testnet);
            }
        }
        CliCommands::HandshakeCommand => {
            let mut ip_args = vec![];
            let mut number = None;
            let mut write_in_file = false;
            let mut by_given_dns: (bool, String) = (false, String::new());
            let mut network = CliNetwork::Mainnet;

            while let Some(arg) = cli_arguments.next() {
                match arg.as_str() {
                    "-f" => write_in_file = true,
                    "-t" => network = CliNetwork::Testnet,
                    _ => {
                        if DNS_LIST_MAINNET.contains(&arg.as_str()) || DNS_LIST_TESTNET.contains(&arg.as_str()){
                            by_given_dns = (true, arg.to_string());
                        }
                        else if let Ok(nr) = arg.parse::<u32>() {
                            number = Some(nr);
                        } else if let Ok(address) = arg.parse::<SocketAddr>() {
                            ip_args.push(address);
                        } else if let Err(e) = arg.parse::<SocketAddr>() {
                            println!("Invalid IP address ({}): {}", arg, e);
                        }
                    }
                }
            }

            if ip_args.is_empty() && number.is_none() {
                return Err(ConfigError::InvalidIPList);
            } else {
                arguments = (CliArguments::IPList((write_in_file, by_given_dns, number, ip_args)), network);
            }
        }
    }

    Ok(arguments)
}

pub async fn run(config: &Config) -> Result<(), ConfigError> {
    match &config.command {
        CliCommands::HelpCommand => {
            print_help_menu();
        }
        CliCommands::ListDNSCommand => {
            print_dns_seeds(&config.network);
        }
        CliCommands::ListIPSCommand => {
            let ips = get_ips_for_dns_list(&config.args, &config.network).await;

            if ips.is_empty() {
                return Err(ConfigError::ErrorFetchingIPs);
            }
            else {
                print_ip_list(ips);
            }
        }
        CliCommands::HandshakeCommand => {
            if let CliArguments::IPList((print_in_file, by_given_dns, ips_number, ip_list)) = &config.args {
                let mut ip_list = ip_list.clone();
                let by_given_dns = by_given_dns.clone();
                if let Some(number_of_ips) = ips_number {
                    ip_list = get_ips_for_handshake(*number_of_ips, by_given_dns, &config.network).await;
                }

                run_handshake(ip_list, *print_in_file, Arc::new(config.network.clone())).await;
            }
        }
    }
    Ok(())
}

fn print_help_menu() {
    println!("\nUsage:");
    println!("    cargo run -- -h");
    println!("    cargo run -- -dns [-t]");
    println!("    cargo run -- -ips [-t] <number> <DNS> <DNS> ... <DNS>");
    println!("    cargo run -- -handshake [-t] <IP> <IP> ... <IP>");
    println!("    cargo run -- -handshake [-t] [-f] <number>\n");
    println!("    cargo run -- -handshake [-t] [-f] <DNS> <number>\n");

    println!("Description:");
    println!("    -h");
    println!("        Display this help menu.\n");

    println!("    -dns [-t]");
    println!("        Display a list of DNS seeds available.");
    println!("        Use the -t flag to run on testnet.\n");

    println!("    -ips [-t] <number> <DNS> <DNS> ... <DNS>");
    println!("        Display a list of a given number of IPs from each of the given DNS in the list.");
    println!("        Use the -t flag to run on testnet.\n");

    println!("    -handshake [-t] <IP> <IP> ... <IP>");
    println!("        Run the handshake for the given list of IPs.");
    println!("        Use the -t flag to run on testnet.\n");

    println!("    -handshake [-t] [-f] <number>");
    println!("        Run the handshake for a given number of IPs from all DNS seeds.");
    println!("        Use the -t flag to run on testnet.");
    println!("        Use the -f flag to save the output to a file (overwrites its contents).\n");

    println!("    -handshake [-t] [-f] <DNS> <number>");
    println!("        Run the handshake for a given number of ips from the specified DNS.");
    println!("        Use the -t flag to run on testnet.");
    println!("        Use the -f flag to save the output to a file (overwrites its contents).\n");
    


}

fn print_dns_seeds(network: &CliNetwork) {
    let available_dns_seeds = match network {
        CliNetwork::Mainnet => DNS_LIST_MAINNET,
        CliNetwork::Testnet => DNS_LIST_TESTNET,
    };

    println!("\nAvailable DNS seeds:");
    for dns in available_dns_seeds {
        println!("    {}", dns);
    }

    println!("\n");
}

fn print_ip_list(ips: Vec<(String, Vec<SocketAddr>)>) {
    println!("IP addresses for each DNS:");
    for dns_and_ip in ips {
        if !dns_and_ip.1.is_empty() {
            println!("    {}", dns_and_ip.0);
        }
        for ip in dns_and_ip.1 {
            println!("        {:?}", ip);
        }
    }
}
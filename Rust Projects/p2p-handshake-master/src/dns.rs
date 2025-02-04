use std::net::SocketAddr;
use crate::config::{CliArguments, CliNetwork};
use tokio::net::lookup_host;
use crate::consts::{PORT_MAINNET, PORT_TESTNET, DNS_LIST_MAINNET, DNS_LIST_TESTNET};


pub async fn get_ips_for_dns_list(args: &CliArguments, network: &CliNetwork) -> Vec<(String, Vec<SocketAddr>)>  {
    let mut ips_for_dns = vec![];

    if let CliArguments::DNSList((number_of_ips_per_dns, dns_list)) = args {
        for dns in dns_list {

            let ips = get_ips_for_given_dns(*number_of_ips_per_dns, dns, network).await;
            ips_for_dns.push((dns.clone(), ips));
        }
    }

    ips_for_dns
}

pub async fn get_ips_for_handshake(number_of_ips: u32, by_given_dns: (bool, String), network: &CliNetwork) -> Vec<SocketAddr> {
    let available_dns_seeds = match network {
        CliNetwork::Mainnet => DNS_LIST_MAINNET,
        CliNetwork::Testnet => DNS_LIST_TESTNET
    };

    let mut ips_from_all_dns_seeds = vec![];
    let ip_number_from_each_dns = distribute_ip_number_for_each_dns(available_dns_seeds.len(), number_of_ips);

    if by_given_dns.0 {
    for (position, &dns) in available_dns_seeds.iter().enumerate() {
        if ip_number_from_each_dns[position] != 0 {
            let ips = get_ips_for_given_dns(ip_number_from_each_dns[position], &dns.to_string(), network).await;
            ips_from_all_dns_seeds.extend_from_slice(ips.as_slice());
        }
        }
    }
    else {
        let ips = get_ips_for_given_dns(ip_number_from_each_dns[0], &by_given_dns.1, network).await;
        ips_from_all_dns_seeds.extend_from_slice(ips.as_slice());
    }



    ips_from_all_dns_seeds
}

fn distribute_ip_number_for_each_dns(number_of_dns: usize, total_ips: u32) -> Vec<u32> {
    let ips_per_dns = total_ips / number_of_dns as u32;
    let mut ips_per_dns_vec = vec![ips_per_dns; number_of_dns];

    let remaining_ips = total_ips % number_of_dns as u32;
    for i in 0..remaining_ips {
        ips_per_dns_vec[i as usize] += 1;
    }

    ips_per_dns_vec
}

async fn get_ips_for_given_dns(number_of_ips_per_dns: u32, dns: &String, network: &CliNetwork) -> Vec<SocketAddr> {
    let mut ip_list = vec![];
    let mut ip_counter = 0;

    let port = match network {
        CliNetwork::Mainnet => PORT_MAINNET,
        CliNetwork::Testnet => PORT_TESTNET,
    };

    match lookup_host((dns.clone(), port)).await {
        Ok(seeds) => {
            for ip in seeds {
                ip_list.push(ip);
                ip_counter += 1;
                if ip_counter == number_of_ips_per_dns {
                    break;
                }
            }
        }
        Err(e) => {
            println!("Error retrieving IPs for {}: {:?}\n", dns, e);
        }
    }

    ip_list
}

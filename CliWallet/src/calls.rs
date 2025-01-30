use reqwest::Error;
use reqwest::Client;
use crate::consts::AccountBalance;

pub async fn get_account_info(principal: &str, net: &str) -> Result<String, Error> {
    // Testnet API URL
    let url = format!(
        "https://stacks-node-api.{}.stacks.co/v2/accounts/{}",
        net, principal
    );

    // Add query parameters
    let query_params = [("proof", "0"), ("tip", "latest")];

    // Send the GET request to the Testnet node
    let response = reqwest::Client::new()
        .get(&url)
        .query(&query_params)
        .send()
        .await?;

    // Parse the response JSON and return the balance
    if response.status().is_success() {
        let account_info: AccountBalance = response.json().await?;
        let hex_balance = &account_info.balance[2..]; // Remove "0x" prefix
        let decimal_balance = u64::from_str_radix(hex_balance, 16).unwrap_or(0) as f64;
        let stx_balance = (decimal_balance / 1_000_000.0).to_string();
        Ok(stx_balance)
    } else {
        println!(
            "Failed to fetch Testnet account info. Status: {}",
            response.status()
        );
        Ok("0".to_string())
    }
}

pub async fn request_stx_tokens(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Base URL for requesting STX tokens
    let base_url = "https://api.testnet.hiro.so/extended/v1/faucets/stx";
    
    // Construct URL with query parameters
    let url = format!("{}?address={}&stacking=true", base_url, address);
    println!("{}",url);
    // Create an HTTP client
    let client = Client::new();

    // Send the POST request with query parameters
    let response = client.post(url)
        .send()
        .await?;

    match response.status().is_success() {
            true => {
                println!("STX tokens requested successfully");
            Ok(())
        },
        false => {
            let error_text = response.text().await?;
            Err(error_text.into())
        }
    }
}

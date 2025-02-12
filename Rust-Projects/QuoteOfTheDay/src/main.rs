mod args;
mod quotes;
mod file_io;

use args::QuoteArgs;
use clap::Parser;
use quotes::Quote;
use file_io::{read_quotes, write_quotes};
use colored::*;
use rand::Rng;

fn print_quote_details(quote: &Quote, author_flag: bool, time_flag: bool) {
    println!("\nQuote: ");
    println!("  -Description: {}", quote.quote);
    if author_flag {
        println!("  -Author: {}", quote.author);
    }
    if time_flag {
        println!("  -Time Stamp: {}", quote.time_stamp);
    }
}

#[tokio::main]
async fn main() {
    let args = QuoteArgs::parse();
    let (author_flag, time_flag, file_flag) = args.process_flags();

    if file_flag {
        match read_quotes("quotes.json") {
            Ok(quotes) => {
                if args.number as usize > quotes.len() {
                    eprintln!("{}", format!("Not enough quotes in file (number of quotes in file is {})", quotes.len()).red());
                    return;
                }
                for _ in (0..quotes.len()).take(args.number as usize) {
                    let i = rand::thread_rng().gen_range(0..quotes.len());                    
                    print_quote_details(&quotes[i], author_flag, time_flag);
                }
            },
            Err(e) => eprintln!("{}", e)
        }
    } else {
        for _ in 0..args.number {
            generate_quote(author_flag, time_flag).await;
        }
    }
}

async fn generate_quote(author_flag: bool, time_flag: bool) {
    let new_quote = match Quote::fetch().await {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    print_quote_details(&new_quote, author_flag, time_flag);

    match read_quotes("quotes.json") {
        Ok(mut quotes) => {
            quotes.push(new_quote);
            match write_quotes("quotes.json", &quotes) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}

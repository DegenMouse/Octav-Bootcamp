use clap::{Args, Subcommand, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WeatherArgs {
    #[clap(subcommand)]
    pub command: WeatherCommand,
    #[clap(short, long, value_parser, global = true)]
    pub unit: Option<String>,
    #[clap(short, long, value_parser, global = true)]
    pub export: Option<bool>,
}

#[derive(Debug, Args, Clone)]
pub struct CompareCommand {
    pub location: String,
}

#[derive(Debug, Subcommand)]
pub enum WeatherCommand {
    /// Get the current weather
    Now(NowCommand),
    /// Get the forecast weather
    Forecast(ForecastCommand),
    /// Get the latest synced weather
    Offline(OfflineCommand),
}

#[derive(Debug, Args)]
pub struct NowCommand {
    #[clap(subcommand)]
    pub command: LocationSubcommand,
    #[clap(short, long, value_parser, global = true)]
    pub compare: Option<String>,
}


#[derive(Debug, Args)]
pub struct ForecastCommand {
    #[clap(subcommand)]
    pub command: LocationSubcommand,
    #[clap(short, long, value_parser, global = true)]
    pub compare: Option<String>,
}

#[derive(Debug, Args)]
pub struct OfflineCommand {
    #[clap(subcommand)]
    pub command: OfflineSubcommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum LocationSubcommand {
    /// Get the current weather by city name
    City {
        #[clap(value_parser)]
        city: String,
    },
    /// Get the current weather by ip address, (US Zipcode, UK Postcode, Canada Postalcode)
    Ip {
        #[clap(value_parser)]
        ip: String,
    },
    /// Get the current weather by latitude and longitude
    LatLong {
       #[clap(value_parser)]
       lat: String,
       #[clap(value_parser)]
       long: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum OfflineSubcommand {
    #[clap(subcommand)]
    Latest(LocationSubcommand),
    #[clap(subcommand)]
    Forecast(ForecastOfflineCommand),
}

#[derive(Debug, Subcommand)]
pub enum ForecastOfflineCommand {
    /// Get the forecast weather by location
    #[clap(subcommand)]
    Location(LocationSubcommand),
}
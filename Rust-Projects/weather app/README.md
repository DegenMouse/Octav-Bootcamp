# Weather Application

A modern weather application built in Rust that provides real-time weather information and forecasts. This project demonstrates Rust's capabilities in handling asynchronous operations, API integration, and user interface development.

## Features

- 🌡️ **Real-time Weather Data**: Get current weather conditions
- 📍 **Location-based Weather**: Weather information for any location
- 📊 **Detailed Forecasts**: 5-day weather forecast
- 🌤️ **Multiple Weather Metrics**: Temperature, humidity, wind speed, and more
- 🔄 **Auto-refresh**: Automatic weather updates
- 🎨 **Clean Interface**: User-friendly command-line interface
- 🔍 **Location Search**: Search for locations by name or coordinates

## Prerequisites

- Rust 1.56.0 or higher
- OpenWeatherMap API key (or similar weather API)
- Internet connection

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd weather
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary (optional):
```bash
cargo install --path .
```

## Configuration

Create a `.env` file in your project directory:

```env
WEATHER_API_KEY=your_api_key_here
DEFAULT_LOCATION=London
UNITS=metric
```

## Usage

Basic usage:
```bash
weather-app --location "New York"
```

Get weather for current location:
```bash
weather-app --current
```

Get forecast:
```bash
weather-app --forecast "London"
```

## Project Structure

```
weather/
├── src/
│   ├── main.rs
│   ├── api.rs
│   ├── models.rs
│   ├── cli.rs
│   └── utils.rs
├── tests/
├── Cargo.toml
└── README.md
```

## Dependencies

- `reqwest`: For making HTTP requests
- `tokio`: For async runtime
- `serde`: For JSON serialization/deserialization
- `clap`: For command-line argument parsing
- `dotenv`: For environment variable management

## Development

To run tests:
```bash
cargo test
```

To run with debug logging:
```bash
RUST_LOG=debug cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
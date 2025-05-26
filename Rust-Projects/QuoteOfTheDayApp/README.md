# Quote of the Day

A simple web application that displays random quotes with their authors and categories. Built with Rust (backend) and HTML/JavaScript (frontend).

## Features

- Random quote generation
- Author information
- Category display with emojis
- Clean, modern UI with Tailwind CSS
- RESTful API endpoints

## Prerequisites

- Rust and Cargo installed
- A modern web browser

## Setup

1. Clone the repository
2. Install dependencies:
```bash
cargo build
```

## Running the Application

1. Start the backend server:
```bash
cargo run
```
The server will start at `http://localhost:8000`

2. Open `public/index.html` in your web browser

## API Endpoints

- `GET /quotes/<number>` - Get specified number of random quotes
  - Example: `http://localhost:8000/quotes/1`

## Frontend Features

- Click "New Quote" button to fetch a new random quote
- Each quote displays:
  - The quote text
  - Author name
  - Category with relevant emoji

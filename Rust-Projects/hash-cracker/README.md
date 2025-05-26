# Hash Cracker

A high-performance password hash cracking tool written in Rust, demonstrating the language's capabilities in cryptographic operations and parallel processing.

## Features

- 🔐 **Multiple Hash Support**: MD5, SHA-1, SHA-256, and more
- 📚 **Dictionary-based Attacks**: Support for wordlist-based cracking
- ⚡ **Brute Force Capabilities**: Configurable character sets and length ranges
- 🔄 **Parallel Processing**: Multi-threaded cracking for maximum performance
- 📊 **Progress Monitoring**: Real-time progress tracking
- 💾 **Session Management**: Save and resume cracking sessions
- 🔍 **Pattern Matching**: Support for common password patterns

## Prerequisites

- Rust 1.56.0 or higher
- Sufficient system resources (CPU/RAM) for intensive operations
- Wordlist files (optional, for dictionary attacks)

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd hash-cracker
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary (optional):
```bash
cargo install --path .
```

## Usage

Dictionary attack:
```bash
hash-cracker --hash <hash> --wordlist <path-to-wordlist> --algorithm <hash-type>
```

Brute force attack:
```bash
hash-cracker --hash <hash> --brute-force --min-length 4 --max-length 8
```

Resume session:
```bash
hash-cracker --resume <session-file>
```

## Project Structure

```
hash-cracker/
├── src/
│   ├── main.rs
│   ├── cracker.rs
│   ├── algorithms.rs
│   ├── dictionary.rs
│   ├── brute_force.rs
│   └── utils.rs
├── tests/
├── wordlists/
├── Cargo.toml
└── README.md
```

## Supported Hash Algorithms

- MD5
- SHA-1
- SHA-256
- SHA-512
- bcrypt
- Argon2
- PBKDF2

## Performance Considerations

- The tool uses multiple threads for parallel processing
- Memory usage can be high with large wordlists
- Consider system resources when running intensive operations

## Development

To run tests:
```bash
cargo test
```

To run with debug logging:
```bash
RUST_LOG=debug cargo run
```

## Security Notice

This tool is intended for:
- Educational purposes
- Security research
- Password recovery (on systems you own)
- Security testing (with proper authorization)

Please use responsibly and ethically.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

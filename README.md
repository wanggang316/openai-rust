# openai-rust

A Rust client library for the OpenAI API. This library provides a native Rust interface for interacting with OpenAI's services, including support for OpenAI-compatible providers like DeepSeek and OpenRouter.

## Features

- Chain-style API for easy interaction
- Builder pattern for client configuration
- Streaming support using async streams
- Support for completions, models, and responses API
- Compatible with OpenAI, DeepSeek, OpenRouter, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai-rust = { git = "https://github.com/wanggang316/openai-rust", branch = "master" }
```

## Quick Start

```rust
use openai_rust::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .api_key("your-api-key")
        .build()?;

    let response = client.completions()
        .create(&request)
        .await?;

    println!("{:?}", response);
    Ok(())
}
```

## Development

### Prerequisites

- Rust 1.85 or later
- Cargo (comes with Rust)

### Building

```bash
cargo build --release
```

### Running Examples

```bash
cargo run --example chat-completions
cargo run --example streaming
cargo run --example models
```

### Testing

```bash
cargo test
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines and [AGENTS.md](AGENTS.md) for project-specific rules (for both humans and agents).

## License

This project is licensed under the MIT License.

## Acknowledgments

Built based on the official [OpenAI API specification](https://platform.openai.com/docs/api-reference).

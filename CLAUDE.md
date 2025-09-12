# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is an OpenAI API client library written in Rust, designed with a modular architecture that separates concerns between request managers and type definitions. The library implements a chain-style API pattern similar to modern cloud SDKs.

### Core Structure

```
openai-rust/src/
├── client.rs              # Main Client with builder pattern
├── completion.rs          # Chat completions request manager (singular)
├── model.rs              # Models API request manager (singular) 
├── response.rs           # Responses API request manager (singular)
├── types/                # Type definitions (plural naming)
│   ├── completions.rs    # Chat completion types
│   ├── models.rs         # Models API types
│   └── responses.rs      # Responses API types
```

**Key Design Patterns:**
- **Business logic files** use singular naming (`completion.rs`, `model.rs`, `response.rs`)
- **Type definition files** use plural naming (`completions.rs`, `models.rs`, `responses.rs`) 
- **Chain-style API**: `client.completions().create(&request).await`
- **Builder pattern**: `Client::builder().api_key(key).base_url(url).build()?`

### API Managers

Each API endpoint has its own manager struct:
- `Completion<'a>` - handles chat completions (both regular and streaming)
- `Model<'a>` - handles model listing
- `Response<'a>` - handles the new OpenAI Responses API

Managers access client fields through `pub(crate)` getter methods: `api_key()`, `base_url()`, `http_client()`.

### Type System

- All types are re-exported from `types::*` in `lib.rs`
- Reasoning field support: Both `reasoning` and `reasoning_content` fields are supported via serde aliases
- Stream types use `async-stream` for Server-Sent Events parsing

## Development Commands

### Building and Testing
```bash
# Build the library
cargo build

# Check compilation (faster)
cargo check

# Run specific example
cargo run --bin chat
cargo run --bin chat-stream
cargo run --bin models-list-example
cargo run --bin responses-example

# Run example from its directory
cd examples/chat && cargo run
```

### Workspace Structure
This is a Cargo workspace with:
- Main library: `openai-rust/`
- Examples: `examples/*/` (each is its own crate)

### Adding New API Endpoints

1. Create type definitions in `types/[endpoint]s.rs` (plural)
2. Create request manager in `[endpoint].rs` (singular)
3. Add manager struct with `new()` and request methods
4. Add manager accessor method to `Client`
5. Update `lib.rs` module declarations
6. Create example in `examples/[endpoint]/`

### Reasoning Fields

The library automatically handles both `reasoning` and `reasoning_content` field names from different OpenAI-compatible providers using serde field aliases. Users only need to access the unified `reasoning` field.

### Environment Variables

Examples expect:
- `OPENAI_API_KEY` - API key
- `OPENAI_BASE_URL` - Base URL (optional, defaults vary by example)

Load via `.env` file using `dotenvy::dotenv()`.

## Development Rules

**Core Reference Materials:**
- `openapi.documented.yml` - Official OpenAI API specification (primary reference)
- OpenAI official documentation at platform.openai.com
- DeepSeek and OpenRouter documentation for compatibility verification

**Development Workflow:**
1. **Compilation**: Always compile after code changes with `cargo check` or `cargo build`
2. **Testing**: Write unit tests for new functionality, run with `cargo test`
3. **Examples**: Create examples for each API endpoint in `examples/[name]/`
4. **Documentation**: Keep docs concise and current - ask before modifying documentation

**API Coverage Status:**
Currently implemented: Chat Completions, Models List, Responses API
Remaining APIs from openapi.yml: Assistants, Audio, Embeddings, Files, Images, Fine-tuning, Batch, Moderations, etc.

**Compatibility Notes:**
Ensure compatibility with OpenAI-compatible providers (DeepSeek, OpenRouter) by supporting variant field names and response formats.
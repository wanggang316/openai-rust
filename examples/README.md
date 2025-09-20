# OpenAI Rust Examples

This directory contains examples demonstrating the usage of the OpenAI Rust library.

## Available Examples

### chat-completions/
Examples for the Chat Completions API:
- `completions-chat` - Basic chat completion request
- `completions-streaming` - Streaming chat completion with real-time output

### models/
Examples for the Models API:
- `models-example` - List all available models

### responses/
Examples for the new Responses API:
- `responses-chat` - Basic response generation
- `responses-streaming` - Streaming responses (placeholder, coming soon)

## Running Examples

```bash
# From the root directory
cargo run --bin completions-chat
cargo run --bin completions-streaming
cargo run --bin models-example
cargo run --bin responses-chat

# Or from a specific example directory
cd examples/chat-completions
cargo run --bin completions-chat
```

## Environment Variables

All examples require:
- `OPENAI_API_KEY` - Your OpenAI API key
- `OPENAI_BASE_URL` (optional) - Custom API endpoint (defaults vary by example)

Create a `.env` file in the root directory with your credentials:
```
OPENAI_API_KEY=your-key-here
OPENAI_BASE_URL=https://api.openai.com/v1
```
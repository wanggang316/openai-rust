# MCP (Model Context Protocol) Examples

This directory contains examples demonstrating how to use function calling (MCP-style tool support) with the OpenAI Rust client.

## Examples

### 1. mcp-functions.rs
Demonstrates synchronous function calling with the OpenAI API:
- Defines multiple functions (weather and calculator)
- Shows the complete request-response cycle with tool execution
- Handles multiple tool calls in parallel
- Demonstrates conversation flow with tool results

### 2. mcp-streaming.rs
Demonstrates streaming function calls:
- Shows how to handle streaming responses with tool calls
- Demonstrates partial tool call assembly from streaming chunks
- Real-time tool execution during streaming

## Key Features Implemented

### Types Added to the Library

1. **Tool Types**:
   - `Tool`: Represents a function tool with type and function definition
   - `Function`: Defines a function with name, description, and JSON schema parameters
   - `ToolChoice`: Controls when and which tools to use ("auto", "none", "required", or specific function)

2. **Function Call Types**:
   - `ToolCall`: Complete tool call with ID, type, and function details
   - `FunctionCall`: Function name and arguments as string
   - `DeltaToolCall`: Streaming version of tool call (partial updates)
   - `DeltaFunction`: Streaming version of function call

3. **Message Enhancements**:
   - Added `tool_calls` field to `RequestMessage` and `ResponseMessage`
   - Added `tool_call_id` field for tool response messages
   - Added `Tool` role for tool response messages
   - `RequestMessage` and `ResponseMessage` provide clear request/response semantics
   - `ChatMessage` remains as a backward-compatible alias for `RequestMessage`

### Request Features

- `tools`: Array of available function tools
- `tool_choice`: Control tool usage behavior
- `parallel_tool_calls`: Enable/disable parallel tool execution

### Streaming Support

The streaming implementation correctly handles:
- Partial tool call data across multiple chunks
- Progressive assembly of function arguments
- Detection of complete tool calls
- Proper stream termination on tool calls

## Running the Examples

### Prerequisites

Set environment variables:
```bash
export OPENAI_API_KEY="your-api-key-here"
export OPENAI_BASE_URL="https://api.openai.com/v1"  # or your preferred endpoint
```

Or create a `.env` file:
```
OPENAI_API_KEY=your-api-key-here
OPENAI_BASE_URL=https://api.openai.com/v1
```

### Run Examples

```bash
# Run function calling example
cargo run --bin mcp-functions

# Run streaming function calling example
cargo run --bin mcp-streaming
```

## Implementation Notes

### Function Definitions

Functions are defined using JSON Schema for parameters:

```rust
let function = Function {
    name: "get_weather".to_string(),
    description: "Get current weather".to_string(),
    parameters: json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City and state"
            }
        },
        "required": ["location"]
    }),
};
```

### Tool Choice Options

- `"auto"`: Let the model decide when to use tools
- `"none"`: Never use tools
- `"required"`: Force the model to use at least one tool
- Specific function: Force use of a particular function

### Conversation Flow

1. Send message with available tools
2. Model responds with tool calls (if appropriate)
3. Execute tool calls and collect results
4. Send tool results back as tool messages
5. Model provides final response

### Error Handling

The examples include error handling for:
- API request failures
- JSON parsing errors in tool arguments
- Tool execution failures
- Streaming connection issues

## Compatibility

This implementation is compatible with:
- OpenAI API
- OpenAI-compatible APIs (like OpenRouter, DeepSeek, etc.)
- Any provider that supports the OpenAI function calling format

The library handles various field name variations (like `reasoning` vs `reasoning_content`) to ensure broad compatibility.
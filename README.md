# Rust Chatbot

A command-line interface chatbot built with Rust that leverages Azure OpenAI's GPT models for interactive conversations.

## Features

- Interactive CLI-based chat interface
- Integration with Azure OpenAI API
- Streaming responses for real-time interaction
- Conversation history tracking
- Configurable model parameters (temperature, max tokens)
- Colored output for better readability

## Prerequisites

- Rust and Cargo installed
- Azure OpenAI API access (API key and endpoint)

## Configuration

The application uses a TOML configuration file located at `config/default.toml`:

```toml
[azure]
openai_api_key = "your_api_key_here"
openai_endpoint = "your_azure_endpoint_here"
model = "gpt-4o-mini"  # or other available models
max_tokens = 1000
temperature = 0.7
```

You can also configure the application using environment variables with the prefix `APP_`.

## Installation

Clone the repository and build the project:

```
git clone <repository-url>
cd rust-chatbot
cargo build --release
```

## Usage

Run the application:

```
cargo run
```

In the chat interface:
- Type your messages and press Enter to send them to the AI
- Type `exit` to quit the application

## Dependencies

- `tokio`: Asynchronous runtime
- `async-openai`: Client for OpenAI API
- `serde` and `serde_json`: Serialization/deserialization
- `config`: Configuration management
- `dotenv`: Environment variable loading
- `futures`: Asynchronous programming utilities
- `colored`: Terminal text coloring

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
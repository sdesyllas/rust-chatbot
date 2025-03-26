use async_openai::{
    config::AzureConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequest, Role,
    },
    Client,
};
use colored::*;
use config::{Config, ConfigError};
use futures::StreamExt;
use serde::Deserialize;
use std::io::{self, Write};
use ferris_says::say;

#[derive(Debug, Deserialize)]
struct Settings {
    azure: AzureSettings,
}

#[derive(Debug, Deserialize)]
struct AzureSettings {
    openai_api_key: String,
    openai_endpoint: String,
    model: String,
    max_tokens: u16,
    temperature: f32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        
        config.try_deserialize()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display welcome message with Ferris
    let welcome_message = "Welcome to Rust Chat Bot with Azure OpenAI, ask me anything!";
    let mut writer = io::BufWriter::new(io::stdout());
    say(welcome_message, 40, &mut writer)?;
    writer.flush()?;

    // Load configuration
    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            return Err(Box::new(e));
        }
    };

    // Check if API key and endpoint are provided
    if settings.azure.openai_api_key.is_empty() || settings.azure.openai_endpoint.is_empty() {
        eprintln!("Error: Azure OpenAI API key and endpoint must be set in config/default.toml");
        return Err("Missing API key or endpoint".into());
    }

    println!("{}", "Type your messages and press Enter. Type 'exit' to quit.".cyan());

    // Set up Azure OpenAI client
    let azure_config = AzureConfig::new()
        .with_api_key(&settings.azure.openai_api_key)
        .with_api_base(&settings.azure.openai_endpoint)
        .with_deployment_id(&settings.azure.model)
        .with_api_version("2023-05-15");

    let client = Client::with_config(azure_config);
    
    // Create a vector to store conversation history
    let mut conversation_history: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessage {
                content: "You are a helpful assistant.".to_string(),
                name: None,
                role: Role::System,
            }
        )
    ];

    loop {
        // Print prompt and get user input
        print!("{} ", "You:".blue().bold());
        io::stdout().flush()?;
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        
        let user_input = user_input.trim();
        
        if user_input.to_lowercase() == "exit" {
            println!("{}", "Goodbye!".green().bold());
            break;
        }
        
        // Add user message to conversation history
        conversation_history.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: user_input.into(),
                name: None,
                role: Role::User,
            }
        ));
        
        // Create chat completion request
        let request = CreateChatCompletionRequest::new(
            settings.azure.model.clone(),
            conversation_history.clone(),
        )
        .max_tokens(settings.azure.max_tokens)
        .temperature(settings.azure.temperature)
        .stream(true);
        
        // Stream the response
        print!("{} ", "Assistant:".green().bold());
        io::stdout().flush()?;
        
        let mut stream = client.chat().create_stream(request).await?;
        // Print the assistant's response as it arrives
        // Initialize a string to store the assistant's response
        // This will be used to print the final response after streaming
        // Initialize a string to store the assistant's response
        // This will be used to print the final response after streaming
        let mut assistant_response = String::new();
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    for choice in response.choices {
                        if let Some(content) = choice.delta.content {
                            print!("{}", content);
                            io::stdout().flush()?;
                            assistant_response.push_str(&content);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("\nError: {}", err);
                    break;
                }
            }
        }
        
        println!("\n");
        
        // Add assistant's response to conversation history
        conversation_history.push(ChatCompletionRequestMessage::Assistant(
            async_openai::types::ChatCompletionRequestAssistantMessage {
                content: assistant_response,
                name: None,
                role: Role::Assistant,
                function_call: None,
                tool_calls: None,
            }
        ));
    }

    Ok(())
}

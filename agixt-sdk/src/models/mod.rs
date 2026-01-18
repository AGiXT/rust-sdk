//! Model types for the AGiXT SDK.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request for OpenAI-compatible API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletions {
    /// The model/agent name to use
    #[serde(default = "default_model")]
    pub model: String,
    /// The messages for the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    /// Sampling temperature (0.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Available tools for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// How to choose which tools to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_choice: Option<String>,
    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Sequences where the model should stop generating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Penalty for repeated tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Penalty for token frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Token biases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    /// The conversation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender (user, assistant, system)
    pub role: String,
    /// The content of the message
    pub content: MessageContent,
    /// Optional message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Content of a message, can be text or structured.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Text content
    Text(String),
    /// Structured content with multiple parts
    Structured(Vec<ContentPart>),
}

/// Part of structured message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPart {
    /// Optional text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
    /// Optional file URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<FileUrl>,
}

/// Image URL reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

/// File URL reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUrl {
    pub url: String,
}

/// Tool definition for function calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// The type of tool (usually "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: ToolFunction,
}

/// Function definition within a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    /// The name of the function
    pub name: String,
    /// Description of the function
    pub description: String,
    /// JSON Schema for the function's parameters
    pub parameters: serde_json::Value,
}

fn default_model() -> String {
    "gpt4free".to_string()
}

impl Default for ChatCompletions {
    fn default() -> Self {
        Self {
            model: default_model(),
            messages: None,
            temperature: Some(0.9),
            top_p: Some(1.0),
            tools: None,
            tools_choice: Some("auto".to_string()),
            n: Some(1),
            stream: Some(false),
            stop: None,
            max_tokens: Some(4096),
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            logit_bias: None,
            user: Some("Chat".to_string()),
        }
    }
}

/// Response from chat completion API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// Choice in chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

/// Token usage in chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// Agent configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub commands: HashMap<String, serde_json::Value>,
}

/// Conversation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

/// Chain information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<ChainStep>>,
}

/// Step in a chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStep {
    pub step_number: i32,
    pub agent_id: String,
    pub prompt_type: String,
    pub prompt: serde_json::Value,
}

/// Prompt information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// Provider information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub supports_embeddings: bool,
}

/// Company information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<Agent>>,
}

/// User information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Extension information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub commands: Vec<ExtensionCommand>,
}

/// Command within an extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

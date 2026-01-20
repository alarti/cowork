use serde::{Deserialize, Serialize};
use crate::skills::{get_available_skills, get_skills_directory_path};

/// Defines the schema and metadata for a tool available to the agent.
/// This matches the format expected by the Claude/Anthropic API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// The unique name of the tool.
    pub name: String,
    /// A description of what the tool does, used by the LLM to understand when to use it.
    pub description: String,
    /// The JSON Schema defining the expected input parameters.
    pub input_schema: serde_json::Value,
}

/// Represents a request from the agent to use a specific tool.
/// Parsed from the LLM's response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    /// Unique identifier for this tool use call.
    pub id: String,
    /// The name of the tool to execute.
    pub name: String,
    /// The input parameters provided by the agent.
    pub input: serde_json::Value,
    /// Thought signature from Google Gemini 3 (required for function calling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

/// Represents the output of a tool execution to be sent back to the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// The type of the result block (usually "tool_result").
    #[serde(rename = "type")]
    pub result_type: String,
    /// The ID of the tool use request this result corresponds to.
    pub tool_use_id: String,
    /// The output content (stdout) or error message.
    pub content: String,
    /// Whether the execution resulted in an error (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// Thought signature from Google Gemini 3 (required for function response)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

impl ToolResult {
    /// Creates a successful tool result.
    pub fn success(tool_use_id: String, content: String) -> Self {
        Self {
            result_type: "tool_result".to_string(),
            tool_use_id,
            content,
            is_error: None,
            thought_signature: None,
        }
    }

    /// Creates an error tool result.
    pub fn error(tool_use_id: String, error: String) -> Self {
        Self {
            result_type: "tool_result".to_string(),
            tool_use_id,
            content: error,
            is_error: Some(true),
            thought_signature: None,
        }
    }
}

/// Content block in Claude response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
        /// Thought signature from Google Gemini 3 (required for function calling)
        #[serde(skip_serializing_if = "Option::is_none")]
        thought_signature: Option<String>,
    },
}

/// Message in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: String,
    pub content: AgentContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
    ToolResults(Vec<ToolResult>),
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub system_prompt: String,
    pub max_turns: u32,
    pub project_path: Option<String>,
    pub allowed_tools: Vec<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: build_system_prompt(),
            max_turns: 20,
            project_path: None,
            allowed_tools: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "edit_file".to_string(),
                "bash".to_string(),
                "glob".to_string(),
                "grep".to_string(),
                "list_dir".to_string(),
                "docker_run".to_string(),
                "docker_list".to_string(),
                "docker_images".to_string(),
            ],
        }
    }
}

pub const DEFAULT_SYSTEM_PROMPT: &str = r#"You are Kuse Cowork, an AI agent that helps users with software development tasks.

You have access to tools that allow you to read and write files, execute commands, and search through codebases.

## IMPORTANT: Always Create a Plan First

Before starting ANY task, you MUST output a plan in this exact format:

<plan>
1. [First step description]
2. [Second step description]
3. [Third step description]
...
</plan>

After outputting the plan, immediately begin executing each step. As you work through each step, indicate progress with:
- `[STEP 1 START]` when beginning a step
- `[STEP 1 DONE]` when completing a step

## Guidelines
- Always read files before modifying them to understand the context
- Use edit_file for small changes, write_file for new files or complete rewrites
- Be careful with bash commands - prefer read-only operations when possible
- Search with glob and grep before making assumptions about file locations
- Explain what you're doing briefly

## Available Tools
- `read_file` - Read file contents
- `write_file` - Create or overwrite a file
- `edit_file` - Make targeted edits to a file
- `bash` - Execute shell commands
- `glob` - Find files by pattern
- `grep` - Search file contents
- `list_dir` - List directory contents
- `docker_run` - Run commands in Docker containers
- `docker_list` - List running containers
- `docker_images` - List available images

## Docker Integration
The project_path (if provided) is automatically mounted to /workspace in containers.
Skills directory (~/.kuse-cowork/skills) is automatically mounted to /skills (read-only).
Default image: python:3.11-alpine. Also available: ubuntu:latest, node:20, rust:alpine

## Workflow
1. Output your plan in <plan> tags
2. Execute step by step, marking progress
3. Verify your changes work
4. Summarize what was accomplished
"#;

/// Build system prompt with dynamic skills information
pub fn build_system_prompt() -> String {
    let mut prompt = DEFAULT_SYSTEM_PROMPT.to_string();

    // Get available skills
    let skills = get_available_skills();

    if !skills.is_empty() {
        let skills_path = get_skills_directory_path();

        prompt.push_str("\n\n## Available Skills\n");
        prompt.push_str(&format!("Skills are located in {} (auto-mounted at /skills in Docker):\n\n", skills_path));

        for skill in skills {
            prompt.push_str(&format!("- **{}**: {}\n", skill.name, skill.description));
        }

        prompt.push_str("\n### Using Skills\n");
        prompt.push_str("When a user's request matches a skill:\n");
        prompt.push_str(&format!("1. Read the skill's SKILL.md file using read_file tool: `{}/{{skill_name}}/SKILL.md`\n", skills_path));
        prompt.push_str("2. Follow the instructions in SKILL.md\n");
        prompt.push_str("3. Load additional referenced files progressively as needed:\n");
        prompt.push_str(&format!("   - `{}/{{skill_name}}/forms.md`\n", skills_path));
        prompt.push_str(&format!("   - `{}/{{skill_name}}/reference.md`\n", skills_path));
        prompt.push_str("4. Execute scripts using docker_run tool - skills are auto-mounted at /skills\n");
        prompt.push_str("5. Example: `python /skills/pdf/scripts/extract_text.py /workspace/document.pdf`\n");
        prompt.push_str("\nNote: The ~ symbol is supported in read_file paths and will expand to the user's home directory.\n");
    }

    prompt
}

/// Event emitted during agent execution
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "plan")]
    Plan { steps: Vec<PlanStepInfo> },
    #[serde(rename = "step_start")]
    StepStart { step: i32 },
    #[serde(rename = "step_done")]
    StepDone { step: i32 },
    #[serde(rename = "tool_start")]
    ToolStart { tool: String, input: serde_json::Value },
    #[serde(rename = "tool_end")]
    ToolEnd { tool: String, result: String, success: bool },
    #[serde(rename = "turn_complete")]
    TurnComplete { turn: u32 },
    #[serde(rename = "done")]
    Done { total_turns: u32 },
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanStepInfo {
    pub step: i32,
    pub description: String,
}

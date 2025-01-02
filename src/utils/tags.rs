use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct MessageContent {
    role: String,
    content: String,
}

pub async fn get_tags(api_key: &str, title: &str) -> Result<Vec<String>> {
    let request = ChatGPTRequest {
        model: "gpt-4".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a tool that extracts structured tags from todo items. For each todo item, identify tags in a JSON array. The tags should include activities (e.g., coding, code review, meeting, documentation, coordination, learning), projects, modules, and additional contexts where possible. Also, categorize activities as either Management or Technical. Do not include any text outside of the JSON array.

Examples:
Input: 'Meeting with legal team to discuss the Omensys MSA issues'
Output: ['Meeting', 'Omensys', 'Management']

Input: 'Coordinate to close the open items of Interop in AROM'
Output: ['Coordination', 'AROM', 'Management']

Input: 'Code review of MPM module in AROM'
Output: ['Code Review', 'MPM', 'AROM', 'Technical']

Input: 'Learn Photosynthesis'
Output: ['Learning', 'Science', 'Biology', 'Technical']".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Identify tags in todo '{}'", title),
            },
        ],
    };

    // Send the request
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    // Parse the response
    if response.status().is_success() {
        let response_json: ChatGPTResponse = response.json().await?;
        if let Some(choice) = response_json.choices.first() {
            let json_compatible_content = choice.message.content.replace('\'', "\"");
            let tags: Vec<String> = serde_json::from_str(&json_compatible_content)
                .map_err(|e| anyhow!("Failed to parse tags as JSON: {}", e))?;
            return Ok(tags);
        }
        Err(anyhow!("No choices in response"))
    } else {
        Err(anyhow!(
            "Failed to get tags: {}",
            response.text().await.unwrap_or_default()
        ))
    }
}

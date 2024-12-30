
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ContentParts {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    role: String,
    parts: Vec<ContentParts>,
}

#[derive(Serialize, Debug)]
struct GenerationConfig {
    temperature: f32,
}

#[derive(Serialize, Debug)]
struct GeminiRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Content,
    // Add other fields like finishReason, index, safetyRatings if needed
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}


pub async fn get_tags(api_key: &String, title: &String) -> Vec<String> {
    let mut tags = vec![];

    // Construct the request URL
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);

    // Create a client
    let client = reqwest::Client::new();

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    // Prepare the request body
    let request_body = GeminiRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![ContentParts {
                text: format!("Deduce tags in the following Todo. Tags should identify the kind of activity, project and module. Todo: '{}'. Output Format: {{'tags': ['<tag1>', '<tag2>', '<tag3>', ...]}}. eg {{'tags': ['Meeting', 'AWS Migration', 'AMX']}}", title),
            }],
        }],
        generation_config: GenerationConfig { temperature: 0.0 }
    };


    // Send the request and get the response
    let response = client.post(&url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Check for HTTP errors
    if !response.status().is_success() {
        println!("Error: {}", response.status());
        let error_text = response.text().await.unwrap();
        println!("Response body: {}", error_text);
        return tags;
    }

    // Parse the response body as JSON
    let gemini_response: GeminiResponse = response.json().await.unwrap();


    // Print the generated text
    if let Some(candidate) = gemini_response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            let part_text = &part.text;
            // 1. Correct the JSON string: Replace single quotes around 'tags' with double quotes
            let corrected_json_string = part_text
                .replace("'", "\"");

            if let Ok(parsed_json) = serde_json::from_str::<Value>(&corrected_json_string) {
                {}
                // 2. Parse the corrected JSON string into a serde_json::Value
                //let parsed_json: Value = serde_json::from_str(&corrected_json_string).unwrap();
                // 3. Extract the tags (if they exist and are an array)
                if let Some(tags_array) = parsed_json["tags"].as_array() {
                    // 4. Convert the tags to a Vec<String>
                    tags = tags_array
                        .iter()
                        .filter_map(|tag| tag.as_str().map(|s| s.to_string()))
                        .collect();
                } else {
                    println!("'tags' key not found or not an array in the JSON string.");
                }
            }
        }

    }
    tags
}


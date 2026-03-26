use crate::chat::get_messages;
use crate::models::GeminiStruct;
use crate::models::Page;
use serde_json::{json, Value};

pub fn build_gemini_request(db: &sled::Db, app: &str, text: &str) -> Result<Value, String> {
    let history_message = get_messages(db, &format!("message:{}", app), &Page::default())?;

    let mut contents: Vec<Value> = history_message
        .iter()
        .map(|m| {
            json!({
                "role": m.role,
                "parts": [{ "text": m.content }]
            })
        })
        .collect();
    contents.push(json!({
        "role": "user",
        "parts": [{ "text": text }]
    }));
    let payload: Value = json!({
      "contents": contents,
      "system_instruction": {
        "parts": { "text": format!("当前时间：{}", chrono::Local::now()) }
      },
      "tools": [{ "google_search": {} }]
    });

    Ok(payload)
}

pub fn parse_gemini_response(response: &Value) -> Result<GeminiStruct, String> {
    let candidates = &response["candidates"][0];
    let text = candidates["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .replace("\\n", "\n")
        .to_string();
    let finish_reason = candidates["finishReason"].as_str().map(String::from);

    Ok(GeminiStruct {
        id: "".to_string(),
        role: "model".to_string(),
        content: text,
        content_type: "text".to_string(),
        model: response["modelVersion"].as_str().map(String::from),
        input_tokens: response["usageMetadata"]["promptTokenCount"]
            .as_i64()
            .map(|v| v as i32),
        output_tokens: response["usageMetadata"]["candidatesTokenCount"]
            .as_i64()
            .map(|v| v as i32),
        finish_reason,
        ..Default::default()
    })
}

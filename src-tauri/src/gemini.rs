use crate::models::GeminiStruct;
use serde_json::Value;

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

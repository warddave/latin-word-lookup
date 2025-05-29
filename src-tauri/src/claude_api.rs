use serde::{Deserialize, Serialize};

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const MAX_TOKENS: u32 = 800;
const MODEL: &str = "claude-3-5-sonnet-20241022";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LatinWordResult {
    pub word_with_macrons: String,
    pub part_of_speech: String,
    pub definition: String,
    pub principal_parts: Option<String>,
    pub stems: Option<String>,
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

pub struct ClaudeClient {
    api_key: String,
    http_client: reqwest::Client,
}

impl ClaudeClient {
    pub fn new_with_key(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn lookup_latin_word(
        &self,
        word: &str,
        custom_prompt: Option<&str>,
    ) -> Result<LatinWordResult, String> {
        let mut prompt = format!(
            "You are a Latin dictionary expert. For the Latin word '{}', provide its dictionary entry with EXTREME ACCURACY.

CRITICAL MACRON INSTRUCTIONS:
1. If the input word has NO macrons, you MUST check if this specific form requires macrons
2. If the input word HAS macrons, preserve them exactly
3. ALWAYS verify the correct macronization for the specific form given
4. Dictionary headwords often need macrons even if the user didn't type them

Return ONLY a JSON object with these exact fields:
{{
  \"word_with_macrons\": \"the word with CORRECT macrons for this specific form\",
  \"part_of_speech\": \"detailed grammatical information based on word type\",
  \"definition\": \"the primary English meaning(s) in a concise format\",
  \"principal_parts\": \"for verbs only: the four principal parts with macrons (null for non-verbs)\",
  \"stems\": \"for verbs only: the present, perfect, and participial stems (null for non-verbs)\"
}}

CRITICAL: For VERBS, you MUST analyze the EXACT form given:
- 'venit' (no macron) = present tense \"he/she/it comes\"
- 'vēnit' (with macron) = perfect tense \"he/she/it came\"
These are DIFFERENT verbs forms - do NOT confuse them!

VERB ANALYSIS REQUIREMENTS:
1. CONJUGATION: 1st (-āre), 2nd (-ēre), 3rd (-ere), 3rd-io (-ere with -iō), 4th (-īre)
2. TENSE: present, imperfect, future, perfect, pluperfect, future perfect
3. MOOD: indicative, subjunctive, imperative
4. VOICE: active, passive
5. PERSON & NUMBER: 1st/2nd/3rd person, singular/plural

Examples of CORRECT analysis:
VERBS (analyze the EXACT form given):
- 'venit': {{\"word_with_macrons\": \"venit\", \"part_of_speech\": \"verb - 4th conjugation, present indicative active, 3rd person singular\", \"definition\": \"he/she/it comes\", \"principal_parts\": \"veniō, venīre, vēnī, ventum\", \"stems\": \"Present: veni-, Perfect: vēn-, Participial: vent-\"}}
- 'vēnit': {{\"word_with_macrons\": \"vēnit\", \"part_of_speech\": \"verb - 4th conjugation, perfect indicative active, 3rd person singular\", \"definition\": \"he/she/it came/has come\", \"principal_parts\": \"veniō, venīre, vēnī, ventum\", \"stems\": \"Present: veni-, Perfect: vēn-, Participial: vent-\"}}
- 'amat': {{\"word_with_macrons\": \"amat\", \"part_of_speech\": \"verb - 1st conjugation, present indicative active, 3rd person singular\", \"definition\": \"he/she/it loves\", \"principal_parts\": \"amō, amāre, amāvī, amātum\", \"stems\": \"Present: amā-, Perfect: amāv-, Participial: amāt-\"}}

NOUNS (add macrons where needed):
- 'amicus': {{\"word_with_macrons\": \"amīcus\", \"part_of_speech\": \"noun - masculine, 2nd declension, nominative singular\", \"definition\": \"friend\", \"principal_parts\": null, \"stems\": null}}
- 'puella': {{\"word_with_macrons\": \"puella\", \"part_of_speech\": \"noun - feminine, 1st declension, nominative singular\", \"definition\": \"girl\", \"principal_parts\": null, \"stems\": null}}
- 'rex': {{\"word_with_macrons\": \"rēx\", \"part_of_speech\": \"noun - masculine, 3rd declension, nominative singular\", \"definition\": \"king\", \"principal_parts\": null, \"stems\": null}}

For all parts of speech:
- NOUNS: Include gender, declension, case, and CORRECT macrons
- ADJECTIVES: Include declension and agreement details with macrons
- PARTICLES/CONJUNCTIONS: Many have NO macrons (et, -que, -ve, sed, aut, etc.)
- PREPOSITIONS: Check each carefully (ā/ab, ē/ex, dē, etc.)
- OTHER: Be specific about function and macrons

SPECIAL CASES:
- 've' (enclitic): NO macrons - means \"or\"
- '-que' (enclitic): NO macrons - means \"and\"
- 'et': NO macrons - means \"and\"
- 'sed': NO macrons - means \"but\"

ESSENTIAL REQUIREMENTS:
- If the input has NO macrons, ADD them where linguistically required (e.g., 'amicus' → 'amīcus')
- If the input HAS macrons, verify they are correct and preserve them
- For verbs, be EXTREMELY careful: 'venit' and 'vēnit' are DIFFERENT forms
- Be absolutely precise about tense, mood, voice for verbs
- Provide principal parts and stems for ALL verbs
- Use null for non-verbs in principal_parts and stems fields
- Common words requiring macrons: amīcus, rēx, dōnum, fēmina, pāx, vīta, etc.
- Output ONLY the JSON, no additional text",
            word
        );

        // Add custom prompt if provided
        if let Some(custom) = custom_prompt {
            if !custom.trim().is_empty() {
                prompt.push_str("\n\nADDITIONAL INSTRUCTIONS:\n");
                prompt.push_str(custom);
            }
        }

        let request = ClaudeRequest {
            model: MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .http_client
            .post(CLAUDE_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!(
                "API request failed with status {}: {}",
                status, error_text
            ));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = claude_response
            .content
            .first()
            .ok_or("No content in response")?
            .text
            .trim();

        // Parse the JSON response
        serde_json::from_str::<LatinWordResult>(content).map_err(|e| {
            format!(
                "Failed to parse Latin word result: {}. Response was: {}",
                e, content
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin_word_result_serialization() {
        let result = LatinWordResult {
            word_with_macrons: "amīcus".to_string(),
            part_of_speech: "noun - masculine, 2nd declension, nominative singular".to_string(),
            definition: "friend".to_string(),
            principal_parts: None,
            stems: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("amīcus"));
        assert!(json.contains("noun - masculine, 2nd declension"));
        assert!(json.contains("friend"));

        let deserialized: LatinWordResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.word_with_macrons, "amīcus");
        assert_eq!(
            deserialized.part_of_speech,
            "noun - masculine, 2nd declension, nominative singular"
        );
        assert_eq!(deserialized.definition, "friend");
        assert_eq!(deserialized.principal_parts, None);
        assert_eq!(deserialized.stems, None);
    }

    #[tokio::test]
    async fn test_claude_client_creation() {
        // Test that client creation works with API key
        let client = ClaudeClient::new_with_key("test-key".to_string());
        assert_eq!(client.api_key, "test-key");
    }
}

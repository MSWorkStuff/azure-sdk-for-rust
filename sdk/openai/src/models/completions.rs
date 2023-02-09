pub(super) mod internal {
    struct CompletionsRequest {
        prompt: Option<Vec<String>>,
        temperature: Option<f32>,
        top_p: Option<f32>,
        logit_bias: Option<i32>,
        user: Option<String>,
        n: Option<n>,
        stream: Option<bool>,
        logprobs: Option<i32>,
        model: Option<String>,
        echo: Option<bool>,
        stop: Option<Vec<String>>,
        completion_config: Option<String>,
        cache_level: Option<i32>,
        presence_penalty: Option<f32>,
        frequency_penalty: Option<f32>,
        best_of: Option<i32>,
    }

    struct CompletionResponse {
        // id: String -> this should be a header under the name "apim-request-id"
        object: String,
        created: Option<i32>,
        model: Option<String>,
        choices: Option<Choice>,
        usage: CompletionUsage,
    }

    struct Choice {
        text: Option<String>,
        index: Option<i32>,
        logprobs: Option<ProbabilityLog>,
        finish_reason: Option<String>
    }

    struct ProbabilityLog {
        tokens: Option<Vec<String>>,
        token_logprobs: Option<Vec<Option<f32>>>,
        top_logprobs: Option<f32>,
        text_offset: Option<Vec<i32>>,
    }

    struct CompletionUsage {
        completion_tokens: i32,
        prompt_tokens: i32,
        total_tokens: i32,
    }
}

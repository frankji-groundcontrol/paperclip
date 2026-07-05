use paperclip_backend::llm::{is_retryable_llm_error, parse_responses_sse};

#[test]
fn parses_openai_responses_sse_fixture() {
    let s = include_str!("fixtures/openai_responses.sse");
    let answer = parse_responses_sse(s).unwrap();

    assert_eq!(answer.text, "7");
    assert_eq!(answer.usage.total_tokens, 18);
    assert!(answer.model.starts_with("gpt-5.4-mini"));
}

#[test]
fn rate_limited_response_failed_is_detected_and_retryable() {
    // A real response.failed carrying a provider concurrency/rate-limit error.
    let sse = concat!(
        "data: {\"type\":\"response.failed\",\"response\":{\"error\":",
        "{\"code\":\"rate_limit_exceeded\",\"message\":\"Concurrency limit exceeded for user\"}}}\n",
    );
    let err = parse_responses_sse(sse).expect_err("rate-limited stream must be an error");
    let msg = err.to_string();
    assert!(msg.contains("rate_limit_exceeded"), "error should carry the code: {msg}");
    assert!(is_retryable_llm_error(&msg), "rate-limit error must be retryable");
    // A non-rate-limit failure is NOT retryable.
    assert!(!is_retryable_llm_error("response failed: model_not_found"));
}


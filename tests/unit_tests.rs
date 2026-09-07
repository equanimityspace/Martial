use martial::types::ManifestSummary;
use martial::verify_file;

// empty bytes should not panic
#[tokio::test]
async fn test_verify_empty_file() {
    let empty_bytes = vec![];
    let summaries = verify_file(empty_bytes, "image/jpeg").await;

    assert!(!summaries.is_empty());
}

// init no_credentials
#[test]
fn test_no_credentials_summary() {
    let summary = ManifestSummary::no_credentials();
    assert_eq!(summary.issuer, "None");
    assert_eq!(summary.ai_present, false);
    assert!(!summary.ai_description.is_empty());
    assert!(summary.error.is_empty());
}

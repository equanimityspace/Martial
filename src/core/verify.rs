use crate::ManifestSummary;
use c2pa::{Context, DigitalSourceType, Manifest, Reader, assertions::Actions};
use std::io::Cursor;

// When examining media JUMBF data there are three possible outcomes:
// 1. Media has no JUMBF data (no content credentials signature)
// 2. Media is signed by content credentials, no generative AI use found
// 3. Media is signed by content credentials, generative AI use found

pub async fn verify_file(file: Vec<u8>, mime_type: &str) -> Vec<ManifestSummary> {
    // create stream and c2pa reader
    let stream = Cursor::new(file);

    let context = match Context::new().with_settings(include_str!("../../config.toml")) {
        Ok(r) => r,
        Err(e) => {
            return vec![ManifestSummary {
                issuer: "Error".to_string(),
                ai_present: false,
                ai_description: Vec::new(),
                error: format!("Failed to create C2PA Context:\n{}", e),
            }];
        }
    };

    let reader = match Reader::from_context(context).with_stream(mime_type, stream) {
        Ok(r) => r,
        Err(_) => return vec![ManifestSummary::no_credentials()],
    };

    let mut summaries = Vec::new();

    for manifest in reader.manifests().values() {
        let issuer = manifest.issuer().unwrap_or_else(|| "Unknown".to_string());
        let ai_info = check_ai_use(manifest);
        let ai_present = !ai_info.is_empty();

        summaries.push(ManifestSummary {
            issuer,
            ai_present,
            ai_description: ai_info,
            error: String::new(),
        });
    }
    if summaries.is_empty() {
        vec![ManifestSummary::no_credentials()]
    } else {
        summaries
    }
}

// Takes a single manifest, returns actions taken by generative AI
fn check_ai_use(manifest: &Manifest) -> Vec<String> {
    let mut ai_info: Vec<String> = Vec::new();

    if let Ok(actions_assertion) = manifest.find_assertion::<Actions>(Actions::LABEL) {
        for action in &actions_assertion.actions {
            let name = action.action();

            // only want to check actions which generative AI use
            if name == "c2pa.created" || name == "c2pa.placed" || name == "c2pa.edited" {
                // check actions source types for generative AI
                if let Some(source_type) = action.source_type() {
                    let matched_str = match source_type {
                        DigitalSourceType::CompositeSynthetic => {
                            Some("Composite Synthetic: mixed AI and human elements")
                        }
                        DigitalSourceType::CompositeWithTrainedAlgorithmicMedia => {
                            Some("Composite with Trained Algorithmic Media: AI modified/edited")
                        }
                        DigitalSourceType::VirtualRecording => {
                            Some("Virtual Recording: fully synthesized using trained/captured data")
                        }
                        DigitalSourceType::TrainedAlgorithmicMedia => {
                            Some("Trained Algorithmic Data: Purely generative AI")
                        }
                        _ => None,
                    };
                    if let Some(description) = matched_str {
                        ai_info.push(description.to_string());
                    }
                }
            }
        }
    }
    return ai_info;
}

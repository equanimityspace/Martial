use crate::ManifestSummary;
use c2pa::{Context, DigitalSourceType, Manifest, Reader, assertions::Actions};
use std::io::Cursor;

// When examining media JUMBF data there are three possible outcomes:
// 1. Media has no JUMBF data (no content credentials signature)
// 2. Media is signed by content credentials, no generative AI use found
// 3. Media is signed by content credentials, generative AI use found

pub async fn verify_file(file: Vec<u8>, mime_type: &str) -> ManifestSummary {
    // create stream and c2pa reader
    let stream = Cursor::new(file);

    let context = match Context::new().with_settings(include_str!("../../config.toml")) {
        Ok(r) => r,
        Err(e) => {
            return ManifestSummary {
                issuer: "Error".to_string(),
                ai_present: false,
                ai_description: None,
                error: format!("Failed to create C2PA Context:\n{}", e),
            };
        }
    };

    let reader = match Reader::from_context(context).with_stream(mime_type, stream) {
        Ok(r) => r,
        Err(_) => return ManifestSummary::no_credentials(),
    };

    let mut summary = ManifestSummary::default();

    for manifest in reader.manifests().values() {
        if summary.issuer == "None" {
            summary.issuer = manifest.issuer().unwrap_or_else(|| "Unknown".to_string());
        }

        if let Some(ai_info) = check_ai_use(manifest) {
            summary.ai_present = true;
            summary.ai_description = Some(ai_info);
            summary.error = "".to_string();
            break;
        }
    }

    summary
}

fn check_ai_use(manifest: &Manifest) -> Option<String> {
    let mut ai_info: Option<String> = None;

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
                    // FIX: get all ai-related history rather than first-found
                    // TODO: consider grabbing all manifest history and displaying it (new feature)
                    if let Some(description) = matched_str {
                        ai_info = Some(description.to_string());
                        break;
                    }
                }
            }
        }
    }
    return ai_info;
}

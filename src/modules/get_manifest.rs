use c2pa::{Context, DigitalSourceType, Reader, assertions::Actions};
use std::io::Cursor;

use crate::Error;

// information to be output for attachment
#[derive(Debug)]
pub struct ManifestSummary {
    pub issuer: String,
    pub ai_present: bool,
    pub ai_description: Option<String>,
}

// Uses file bytes to find c2pa manifest if it exists
// NOTE: should return manifest.json or nothing_found.json
// which will be unwrapped during bot response

pub async fn get_manifest(
    file: &poise::serenity_prelude::Attachment,
) -> Result<ManifestSummary, Error> {
    // result if no JUMBF data
    let mut result = ManifestSummary {
        issuer: "None".to_string(),
        ai_present: false,
        ai_description: Some("No Content Credentials Found".to_string()),
    };

    // download attachment into memory
    println!("Downloading {}", file.filename);
    let file_data = file.download().await?;
    println!("Finished downloading {}", file.filename);

    // establish mime type
    let content_type = file.content_type.as_deref().unwrap_or("");
    println!("Mime type: {}", content_type);

    // create stream, reader
    let stream = Cursor::new(file_data);

    let context = Context::new().with_settings(include_str!("../../config.toml"))?;
    let reader = match Reader::from_context(context).with_stream(content_type, stream) {
        Ok(r) => r,
        Err(e) => {
            // if no JUMBF data to check
            return Ok(ManifestSummary {
                issuer: e.to_string(),
                ai_present: false,
                ai_description: Some("No Content Credentials found".to_string()),
            });
        }
    };

    // extract resources to return to discord
    // one attachment may have multiple manifests
    // so as soon as AI is detected, return result
    let mut ai_present = false;

    for manifest in reader.manifests().values() {
        let issuer = manifest
            .issuer()
            .unwrap_or_else(|| "Unknown Origin".to_string());

        if let Ok(actions_assertion) = manifest.find_assertion::<Actions>(Actions::LABEL) {
            // should? skip all manifests after AI has been found to prevent multiple embeds for
            // one attachment
            if ai_present {
                break;
            };

            for action in &actions_assertion.actions {
                let name = action.action();
                println!("sources: {:?}", action.source_type());

                // only want to check actions which alter content for generative ai
                if name == "c2pa.created" || name == "c2pa.placed" || name == "c2pa.edited" {
                    // check action source types for generative ai
                    if let Some(source_type) = action.source_type() {
                        let matched_str = match source_type {
                            DigitalSourceType::CompositeSynthetic => {
                                Some("Composite Synthetic: mixed AI and human elements")
                            }
                            DigitalSourceType::CompositeWithTrainedAlgorithmicMedia => {
                                Some("Composite with Trained Algorithmic Media: AI modified/edited")
                            }
                            DigitalSourceType::VirtualRecording => Some(
                                "Virtual Recording: fully synthesized using trained/captured data",
                            ),
                            DigitalSourceType::TrainedAlgorithmicMedia => {
                                Some("Trained Algorithmic Data: Purely generative AI")
                            }
                            _ => None,
                        };

                        // FIX: Get all AI-related history rather than first found
                        if let Some(description) = matched_str {
                            ai_present = true;
                            let ai_description = Some(description.to_string());

                            result = ManifestSummary {
                                issuer: issuer,
                                ai_present: ai_present,
                                ai_description: ai_description,
                            };

                            break;
                        }
                    }
                }
            }
        }
    }

    // If JUMBF data and no AI
    if ai_present == false {
        (result = ManifestSummary {
            issuer: "".to_string(),
            ai_present: false,
            ai_description: None,
        });
    }
    Ok(result)
}

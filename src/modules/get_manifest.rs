use c2pa::{Context, DigitalSourceType, Reader, assertions::Actions};
use futures::future::join_all;
use std::io::Cursor;

use crate::Error;

// structure for discord modal.rs whenever I do that
#[derive(Debug)]
pub struct ManifestSummary {
    pub issuer: String,
    pub ai_present: bool,
    pub ai_description: Option<String>,
}

// Uses file bytes to find c2pa manifest if it exists
// NOTE: should return manifest.json or nothing_found.json
// which will be unwrapped during bot response

pub async fn GetManifest(
    files: Vec<poise::serenity_prelude::Attachment>,
) -> Result<Vec<ManifestSummary>, Error> {
    // c2pa requires attachment byte info
    // loaded into memory so make sure it doesnt become too large

    let processing_tasks = files.into_iter().map(|file| {
        async move {
            println!("Downloading {}", file.filename);
            let file_data = file.download().await?;
            println!("Finished downloading {}", file.filename);

            let content_type = file.content_type.as_deref().unwrap_or("");
            println!("Mime type: {}", content_type);

            let stream = Cursor::new(file_data);

            // create reader
            let context = Context::new().with_settings(include_str!("../../config.toml"))?;
            let reader = Reader::from_context(context).with_stream(content_type, stream)?;

            // extract resources to return to discord
            let manifest = match reader.active_manifest() {
                Some(m) => m,
                None => {
                    return Ok(ManifestSummary {
                        issuer: "No Content Credentials Found".to_string(),
                        ai_present: false,
                        ai_description: None,
                    });
                }
            };

            let issuer = manifest
                .issuer()
                .unwrap_or_else(|| "Unknown Origin".to_string());

            // get digital source types for generative actions if not None
            let mut ai_present = false;
            let mut ai_description = None;

            if let Ok(actions_assertion) = manifest.find_assertion::<Actions>(Actions::LABEL) {
                for action in &actions_assertion.actions {
                    let name = action.action();

                    // only want to check actions which alter content for generative ai
                    if name == "c2pa.created" || name == "c2pa.placed" {
                        // check action source types for generative ai
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
                                DigitalSourceType::TrainedAlgorithmicData => {
                                    Some("Trained Algorithmic Data: Purely generative AI")
                                }
                                _ => None,
                            };

                            if let Some(description) = matched_str {
                                ai_present = true;
                                ai_description = Some(description.to_string());
                                break;
                            }
                        }
                    }
                }
            }

            Ok(ManifestSummary {
                issuer,
                ai_present,
                ai_description,

            }) as Result<ManifestSummary, Error>
        }
    });

    let results: Vec<Result<ManifestSummary, Error>> = join_all(processing_tasks).await;

    // transform Vec<Result<T, E>> to Result<Vec<T>, E> to use ?
    let summaries: Vec<ManifestSummary> = results.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(summaries)
}

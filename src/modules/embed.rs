use chrono::Utc;
use poise::serenity_prelude as serenity;

use crate::modules::get_manifest::ManifestSummary;

// verification results
pub fn build_verification_embed(
    summary: ManifestSummary,
    attachment: serenity::Attachment,
) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();

    embed = embed.thumbnail(&attachment.url);

    embed = embed
        .title("Content Credentials")
        .field("Issuer/Company", summary.issuer, true);

    if summary.ai_present {
        embed = embed
            .field("AI Involvement", "⚠️ Generative AI Detected", true)
            .color(0xffcc00);
    } else {
        embed = embed
            .field("AI Involvement", "✅ No generative AI use detected", true)
            .color(0x00ff00);
    }

    if let Some(description) = summary.ai_description {
        embed = embed.field("AI Details", description, false);
    }

    // timestamp
    let timestamp =
        serenity::Timestamp::parse(&Utc::now().to_rfc3339()).expect("Invalid Timestamp");

    embed = embed.timestamp(timestamp);

    embed
}

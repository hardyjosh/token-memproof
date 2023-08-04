use hdk::prelude::*;
use gated_dna_integrity::*;
#[hdk_extern]
pub fn get_post(original_post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_post_hash.clone(), LinkTypes::PostUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_post_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_post_hash.clone(),
    };
    get(latest_post_hash, GetOptions::default())
}
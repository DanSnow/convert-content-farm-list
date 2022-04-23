mod transform_sites;

use anyhow::Result;
use indoc::indoc;
use std::{env, time::SystemTime};
use tap::prelude::*;
use time::{macros::format_description as fd, OffsetDateTime};
use transform_sites::transform_sites;

static CONTENT_FARM: &str =
    "https://danny0838.github.io/content-farm-terminator/files/blocklist/content-farms.txt";

fn main() -> Result<()> {
    let default_update_url = option_env!("UPDATE_URL").unwrap_or("");
    let runtime_update_url = env::args().nth(1).or_else(|| env::var("UPDATE_URL").ok());
    let update_url = runtime_update_url.as_deref().unwrap_or(default_update_url);

    let content = ureq::get(CONTENT_FARM).call()?.into_string()?;
    println!("! Title: zh Content Farms List");
    if !update_url.is_empty() {
        println!("! Redirect: {update_url}",);
    }
    println!("! Version: {}", today()?);
    let header = indoc!(
        r#"
        ! Expires: 3 days

        ! Content farm"#
    );
    println!("{header}");
    for line in transform_sites(&content) {
        println!("||{line}");
    }
    Ok(())
}

fn today() -> Result<String> {
    let now = SystemTime::now().conv::<OffsetDateTime>();
    Ok(now.format(fd!("[year][month][day]"))?)
}

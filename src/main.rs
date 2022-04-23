use anyhow::Result;
use indoc::indoc;
use std::time::SystemTime;
use tap::prelude::*;
use time::{macros::format_description as fd, OffsetDateTime};

static CONTENT_FARM: &str =
    "https://danny0838.github.io/content-farm-terminator/files/blocklist/content-farms.txt";

fn main() -> Result<()> {
    let content = ureq::get(CONTENT_FARM).call()?.into_string()?;
    println!("! Title: zh Content Farms List");
    println!("! Redirect: {}", env!("UPDATE_URL"));
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

fn transform_sites(content: &str) -> Vec<&str> {
    let mut list = content
        .lines()
        .skip_while(|line| line.starts_with("/"))
        .map(|line| line.split(' ').nth(0).unwrap().trim_end_matches('/'))
        .collect::<Vec<_>>();
    list.sort();
    list
}

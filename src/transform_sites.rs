pub fn transform_sites(content: &str) -> Vec<&str> {
    let mut list = content
        .lines()
        .skip_while(|line| line.starts_with('/'))
        .filter_map(|line| {
            if line.starts_with('/') || line.starts_with('#') || line.starts_with(' ') {
                None
            } else {
                Some(line.split(' ').next().unwrap().trim_end_matches('/'))
            }
        })
        .collect::<Vec<_>>();
    list.sort_unstable();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_hosts() {
        assert_eq!(
            transform_sites("example.com\nexample.com\n"),
            vec!["example.com", "example.com"]
        );
    }

    #[test]
    fn should_ignore_regex() {
        assert!(transform_sites("/^https?://example.com/").is_empty());
    }

    #[test]
    fn should_ignore_comment() {
        assert_eq!(transform_sites("example.com // foo"), vec!["example.com"]);
    }
}

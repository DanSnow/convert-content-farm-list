fn is_regex(line: &str) -> bool {
    line.starts_with('/')
}

fn is_comment(line: &str) -> bool {
    line.starts_with('#') || line.starts_with(' ')
}

fn parse_site(line: &str) -> Option<&str> {
    if is_regex(line) || is_comment(line) {
        None
    } else {
        let site = line.split(' ').next().unwrap().trim_end_matches('/').trim();
        if site.is_empty() {
            None
        } else {
            Some(site)
        }
    }
}

pub fn transform_sites(content: &str) -> Vec<&str> {
    let mut list = content
        .lines()
        .skip_while(|line| is_regex(line))
        .filter_map(|line| parse_site(line))
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

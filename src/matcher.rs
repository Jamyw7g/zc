use std::path::MAIN_SEPARATOR;

use regex::{escape, Regex, RegexBuilder};

// match ".*`needle`.*`needle`.*"
pub fn anywhere_re(needles: &[String], ignore_case: bool) -> Regex {
    let mut pattern = String::from(".*");
    for s in needles {
        pattern.push_str(&escape(s));
        pattern.push_str(".*");
    }
    RegexBuilder::new(&pattern)
        .case_insensitive(ignore_case)
        .unicode(true)
        .build()
        .unwrap()
}

// match "`needle`[^/]/[^/]`needle`[^/]$"
pub fn consecutive_re(needles: &[String], ignore_case: bool) -> Regex {
    let mut seq = String::from("[^");
    seq.push(MAIN_SEPARATOR);
    seq.push_str("]*");

    let mut end = seq.clone();
    end.push('$');

    let mut sub_seq = seq.clone();
    sub_seq.push(MAIN_SEPARATOR);
    sub_seq.push_str(&seq);

    let escaped_str: Vec<_> = needles.iter().map(|s| escape(s)).collect();
    let mut pattern = escaped_str.join(&sub_seq);
    pattern.push_str(&end);

    RegexBuilder::new(&pattern)
        .case_insensitive(ignore_case)
        .unicode(true)
        .build()
        .unwrap()
}

#[inline]
pub fn match_dist(needle: &str, path: &str, ignore_case: bool, threshold: f32) -> bool {
    if let Some(basename) = path.split(MAIN_SEPARATOR).last() {
        letter_similarity(needle, basename, ignore_case) >= threshold
    } else {
        false
    }
}

#[inline]
fn letter_similarity(needle: &str, basename: &str, ignore_case: bool) -> f32 {
    let len = needle.len().max(basename.len());
    let needle = if ignore_case {
        needle.to_lowercase()
    } else {
        needle.to_string()
    };
    let basename = if ignore_case {
        needle.to_lowercase()
    } else {
        needle.to_string()
    };
    1.0 - edition_dist(&needle, &basename) as f32 / len as f32
}

fn edition_dist(letter_a: &str, letter_b: &str) -> usize {
    let letter_ch1 = letter_a.chars().collect::<Vec<_>>();
    let letter_ch2 = letter_b.chars().collect::<Vec<_>>();
    let mut dist: Vec<_> = (0..=letter_ch1.len()).collect();

    let (mut isjs, mut isj, mut ijs, mut res);
    for j in 1..=letter_ch2.len() {
        isjs = dist[0];
        dist[0] = j;
        for i in 1..=letter_ch1.len() {
            isj = dist[i - 1]; // lev(i-1, j)
            ijs = dist[i]; // lev(i, j-1)
            res = if letter_ch1[i - 1] == letter_ch2[j - 1] {
                isjs
            } else {
                (isj + 1).min(ijs + 1).min(isjs + 1)
            };
            isjs = dist[i]; // lev(i-1, j-1)
            dist[i] = res;
        }
    }

    dist[letter_ch1.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_dist() {
        assert_eq!(edition_dist("eeba", "abac"), 3);
        assert_eq!(edition_dist("love", "lolpe"), 2);
        assert_eq!(edition_dist("我我他你", "你他你们"), 3);
    }

    #[test]
    fn test_anywhere() {
        let re = anywhere_re(&["foo".to_string(), "bar".to_string()], true);
        assert!(re.is_match("/foo/go/bar"));
        assert!(re.is_match("/hello/footer/bar/"));
        assert!(re.is_match("/hello/footer/barrier"));
        assert!(!re.is_match("/hello/for_to/bar"));
        assert!(!re.is_match("/hello/for/badder"));
    }

    #[test]
    fn test_consecutive() {
        let re = consecutive_re(&["foo".to_string(), "bar".to_string()], true);
        assert!(re.is_match("foo/bar"));
        assert!(re.is_match("/hello/foo/bar"));
        assert!(!re.is_match("/hello/foo/bar/"));
        assert!(!re.is_match("/hello/foo/hello/bar"));
    }
}

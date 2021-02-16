use std::{cmp::max, path::MAIN_SEPARATOR};

use regex::{RegexBuilder, escape};

use crate::data::DataList;


pub fn match_anywhere(
    needles: &Vec<String>,
    haystack: &DataList,
    ignore_case: bool
) -> Vec<(String, usize)> {
    let mut re_needle = String::from(".*");
    let escaped_str: Box<[String]> = needles.into_iter()
        .map(|s| escape(&s))
        .collect();

    re_needle.push_str(&escaped_str.join(".*"));
    re_needle.push_str(".*");

    let re = RegexBuilder::new(&re_needle)
        .case_insensitive(ignore_case)
        .unicode(true)
        .build()
        .unwrap();

    haystack.sort()
        .iter()
        .filter(|(k, _)| re.is_match(k))
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

pub fn match_consecutive(
    needles: &Vec<String>,
    haystack: &DataList,
    ignore_case: bool
) -> Vec<(String, usize)> {
    let mut re_no_seq = String::from("[^");
    re_no_seq.push(MAIN_SEPARATOR);
    re_no_seq.push_str("]*");

    let mut re_no_seq_end = String::from(&re_no_seq);
    re_no_seq_end.push('$');

    let escaped_str: Box<[String]> = needles.into_iter()
        .map(|s| escape(&s))
        .collect();
    let mut re_one_seq = String::from(&re_no_seq);
    re_one_seq.push(MAIN_SEPARATOR);
    re_one_seq.push_str(&re_no_seq);

    let mut re_needle = escaped_str.join(&re_one_seq);
    re_needle.push_str(&re_no_seq_end);

    let re = RegexBuilder::new(&re_needle)
        .case_insensitive(ignore_case)
        .unicode(true)
        .build()
        .unwrap();

    haystack.sort()
        .iter()
        .filter(|(k, _)| re.is_match(k))
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

pub fn match_fuzzy(
    needles: &Vec<String>,
    haystack: &DataList,
    ignore_case: bool,
    threshold: Option<f32>
) -> Vec<(String, usize)> {
    let threshold = threshold.unwrap_or(0.6);
    let needle = if ignore_case {
        needles.last().unwrap().to_lowercase()
    } else {
        needles.last().unwrap().to_string()
    };

    haystack.sort()
        .iter()
        .filter(|(k, _)| {
            let dir_name = k.split(MAIN_SEPARATOR).last().unwrap();
            letter_similarity(&needle, &dir_name, ignore_case) >= threshold
        })
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

fn letter_similarity(word_a: &str, word_b: &str, ignore_case: bool) -> f32 {
    let (a, b): (Box<[_]>, Box<[_]>) = if ignore_case {
        (word_a.to_lowercase().chars().collect(),
         word_b.to_lowercase().chars().collect())
    } else {
        (word_a.chars().collect(),
         word_b.chars().collect())
    };

    let dist = 3;
    let len = max(a.len(), b.len());

    1.0 - dist as f32 / len as f32
}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut chars:Vec<char> = s.chars().collect();
            chars.sort();
            let key : String = chars.into_iter().collect();
            anagrams.entry(key).or_insert(Vec::new()).push(s);
        }

        anagrams.values().cloned().collect()
    }
}

pub struct Solution;

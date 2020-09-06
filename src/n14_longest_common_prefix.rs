impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let prefix = &strs[0];

        for p in prefix.chars() {
            println!("{}", p);
        }

        String::from("")
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_14() {
        let s = "Hello world!".to_string();
        Solution::longest_common_prefix(vec![s]);
    }
}

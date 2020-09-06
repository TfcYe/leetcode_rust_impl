impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let mut iterators = Vec::new();

        if strs.len() == 0 {
            return res;
        }

        for in_str in &strs {
            iterators.push(in_str.chars());
        }

        loop {
            let tmp = match iterators[0].next() {
                Some(char_in) => Some(char_in),
                None => return res,
            };

            for iter in &mut iterators[1..] {
                if iter.next() == tmp {
                    continue;
                } else {
                    return res;
                }
            }

            res.push(tmp.unwrap());
        }
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

impl Solution {
    pub fn reverse(x: i32) -> i32 {
           if x==0 {
        return 0;
    }
    let mut x: i64 = x as i64;
    let mut result:i64 = 0;
    while x != 0 {
        result = result*10 + x%10;
        x = x/10;
    }
    if result > i32::max_value() as i64 {
        return 0;
    } 

    if result < i32::min_value() as i64 {
        return 0;
    }
    return result as i32;
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}
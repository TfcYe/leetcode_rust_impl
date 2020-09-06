impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        if n == 0 && start == 0 {
            return 0;
        }
        let mut nums: Vec<i32> = Vec::new();
        for i in 0..n {
            let num = start + 2 * i;
            nums.push(num);
        }

        let mut res = nums[0];
        for i in 1..nums.len() {
            res = res ^ nums[i];
        }

        res
    }
}

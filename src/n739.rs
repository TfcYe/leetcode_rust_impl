impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let res: Vec<i32> = vec![];

        for i in 0..t.len() {
            let mut days = 0;
            for j in i..t.len() {
                if t[i] < t[j] {
                    days = (j - i) as i32;
                    break;
                }
            }

            res.push(days)
        }

        res
    }
}

// impl Solution {
//     pub fn is_power_of_two(n: i32) -> bool {
//         if n == 0 {
//             return false;
//         }
//         let mut t = n;

//         while t != 1 {
//             if t % 2 != 0 {
//                 return false;
//             }
//             t >>= 1;
//         }

//         true
//     }
// }

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}

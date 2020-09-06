impl Solution {
    pub fn swap_numbers(numbers: Vec<i32>) -> Vec<i32> {
        let mut mut_numbers = numbers;
        mut_numbers[0] = mut_numbers[0] ^ mut_numbers[1];
        mut_numbers[1] = mut_numbers[0] ^ mut_numbers[1];
        mut_numbers[0] = mut_numbers[1] ^ mut_numbers[0];

        mut_numbers
    }
}
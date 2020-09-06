impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut sort_nums1 = nums1;
        let mut sort_nums2 = nums2;

        sort_nums1.sort();
        sort_nums2.sort();

        let mut res:Vec<i32> = Vec::new();
        let (mut i, mut j) = (0,0);

        while i<sort_nums1.len() && j<sort_nums2.len() {
            if sort_nums1[i] > sort_nums2[j] {
                j+=1;
            } else if sort_nums1[i] < sort_nums2[j] {
                i+=1;
            } else {
                i+=1;
                j+=1;
                res.push(sort_nums1[i]);
            }
        }

        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        Solution::intersect(vec![2, 7, 11, 15], vec![2, 7, 11, 15]);
    }
}
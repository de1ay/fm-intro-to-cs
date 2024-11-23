pub fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let middle = (right + left) / 2;

        if nums[middle] == target {
            return middle as i32;
        } else if nums[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    return -1;
}

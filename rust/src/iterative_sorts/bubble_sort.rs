pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    for i in (1..nums.len()).rev() {
        let mut has_swap_occurred = true;
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
                has_swap_occurred = false;
            }
        }
        if has_swap_occurred {
            break;
        }
    }
    return nums;
}

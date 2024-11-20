pub fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        let number_to_insert = nums[i];
        let mut j = i;

        while j > 0 {
            if number_to_insert >= nums[j - 1] {
                break;
            }
            nums[j] = nums[j - 1];
            j -= 1;
        }

        nums[j] = number_to_insert;
    }

    return nums;
}

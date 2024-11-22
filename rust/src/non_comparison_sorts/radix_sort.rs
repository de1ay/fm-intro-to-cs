fn get_max_digit_index(nums: &Vec<i32>) -> usize {
    let mut max_num = nums[0];

    for num in nums {
        if *num > max_num {
            max_num = *num;
        }
    }

    return max_num.to_string().len();
}

fn get_digit_by_index(num: &i32, index: usize) -> usize {
    let num_str = num.to_string();

    if index >= num_str.len() {
        return 0;
    }

    let digit = num_str.as_bytes().iter().rev().nth(index).unwrap() - b'0';

    return digit as usize;
}

pub fn radix_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut buckets: [Vec<i32>; 10] = std::array::from_fn(|_| Vec::new());

    let mut arr = nums.to_vec();

    let max_digit_index = get_max_digit_index(&arr);
    for i in 0..max_digit_index {
        for num in &arr {
            let digit_index = get_digit_by_index(num, i);
            buckets[digit_index].push(*num);
        }

        let mut insert_index: usize = 0;
        for bucket in &buckets {
            for num in bucket {
                arr[insert_index] = *num;
                insert_index += 1;
            }
        }

        buckets = std::array::from_fn(|_| Vec::new());
    }

    return arr;
}

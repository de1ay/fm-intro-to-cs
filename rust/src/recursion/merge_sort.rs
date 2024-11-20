fn merge_vectors(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut left_ptr = 0;
    let mut right_ptr = 0;

    while left_ptr < left.len() && right_ptr < right.len() {
        let left_val = left[left_ptr];
        let right_val = right[right_ptr];

        if left_val <= right_val {
            res.push(left_val);
            left_ptr += 1;
        } else {
            res.push(right_val);
            right_ptr += 1;
        }
    }

    if left_ptr == left.len() {
        res.append(&mut right[right_ptr..].to_vec());
    } else {
        res.append(&mut left[left_ptr..].to_vec());
    }
    return res;
}

pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let middle = arr.len() / 2;

    let left = merge_sort(arr[..middle].to_vec());
    let right = merge_sort(arr[middle..].to_vec());

    return merge_vectors(left, right);
}

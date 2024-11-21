pub fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 { return arr; }

    let pivot = arr[arr.len()-1];

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    arr[..arr.len()-1].iter().for_each(|&num| {
        if num <= pivot {
            left.push(num);
        } else {
            right.push(num);
        }
    });
    
    left = quick_sort(left);
    right = quick_sort(right);

    left.push(pivot);
    left.append(&mut right);

    return left;
}

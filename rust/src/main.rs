mod iterative_sorts;

fn main() {
    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = iterative_sorts::bubble_sort::bubble_sort(nums);
    println!("bubble_sort: {:?}", nums);

    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = iterative_sorts::insertion_sort::insertion_sort(nums);
    println!("insetion_sort: {:?}", nums);
}

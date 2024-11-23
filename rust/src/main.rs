mod iterative_sorts;
mod recursion;
mod non_comparison_sorts;
mod binary_search;

fn main() {
    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = iterative_sorts::bubble_sort::bubble_sort(nums);
    println!("bubble_sort: {:?}", nums);

    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = iterative_sorts::insertion_sort::insertion_sort(nums);
    println!("insetion_sort: {:?}", nums);

    println!("factorial of 5: {}", recursion::factorial::factorial(5));

    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = recursion::merge_sort::merge_sort(nums);
    println!("merge_sort: {:?}", nums);

    let mut nums = vec![10, -1, 1, 0, 43, 44, 4, 6];
    nums = recursion::quick_sort::quick_sort(nums);
    println!("quick_sort: {:?}", nums);

    let mut nums = vec![10, 11, 1, 0, 43, 44, 4, 6];
    nums = non_comparison_sorts::radix_sort::radix_sort(nums);
    println!("radix_sort: {:?}", nums);

    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100];
    let index = binary_search::binary_search::binary_search(&nums, 100);
    println!("binary_search (expected 11): {:?}", index);
}

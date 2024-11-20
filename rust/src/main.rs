mod iterative_sorts;
mod recursion;

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
}

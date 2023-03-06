mod bubble_sort;
use bubble_sort::bubble_sort;

fn main() {
    let mut nums = [-9, 23, -32, 99, 12, 7];
    println!("unsorted: {:?}", nums);
    bubble_sort(&mut nums);
    println!("sorted: {:?}", nums);
}

use counting_sort::CountingSort;

fn main() {
    let test_vector = vec![4,3,2,1];
    let iterator = test_vector.iter();
    println!("{:?}", iterator.cnt_sort());
}

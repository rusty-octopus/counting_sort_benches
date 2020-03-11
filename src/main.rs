use counting_sort::CountingSort;

fn main() {
    println!("Hello, world!");
    let test_vector = vec![4,3,2,1];
    //vec.iter().counting_sort();
    //counting_sort::counting_sort(& mut vec.iter());
    //let mut iterator:Box<dyn DoubleEndedIterator<Item=u8>> = Box::new(vec.into_iter());
    //println!("{:?}", iterator.counting_sort());
    let iterator = test_vector.iter();
    println!("{:?}", iterator.cnt_sort());
}

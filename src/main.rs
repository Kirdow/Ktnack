use std::io;

use std::ops::AddAssign;

fn sum_list<T: AddAssign<T> + Default + Copy>(list: &[T]) -> T {
    let mut sum: T = T::default();
    for val in list.iter() {
        sum += *val;
    }
    sum
}

fn main() {
    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
}

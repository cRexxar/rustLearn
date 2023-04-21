// use crate::garden::vegetables::Asparagus;
// use Vec;

use std::vec;
// struct Number<T> {
//     body: Option<T>,
// }
// impl Number<T> {
//     fn 
// }

fn main() {
    let number_list: Vec<i32> = vec![1, 2];
    let largest = largest(&number_list);
    // let number = match largest {
    //     Some(n) => n,
    //     None => &0,
    // };
    println!("{:?}", largest)
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = list.get(0)?;

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return Some(largest);
}
// fn get_area(rect:&Rectangle) -> u32 {
//     return rect.height*rect.width
// }

// use crate::garden::vegetables::Asparagus;
// use Vec;
use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::from([1, 2, 3, -1]);
    println!("Vector v is {:?}", v);

    // let fifth: Option<&i32> = v.get(4);
    // match fifth {
    //     Some(fifth) => println!("the fifth element is {fifth}"),
    //     None => println!("There is no fifth element"),
    // }
    let x = v.get(4);
    let y = match x {
        Some(x) => x,
        None => &-2021,
    };
    println!("{y}");

    let mut v1 = Vec::from([1, 2, 3]);
    let first = v1[0];
    v1.push(6);
    println!("{:?} {}", v1, first);

    for i in &mut v1 {
       *i = *i + 50;
    }
    println!("{:?}", v1);

    let mut _data = "test".to_string();
    _data.push_str("abc");
    println!("{_data}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}

// fn get_area(rect:&Rectangle) -> u32 {
//     return rect.height*rect.width
// }

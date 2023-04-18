

fn main() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{}", a[0]);
    println!("{}", a[1]);

    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[5..];
    let length = len_string(&s);
    println!("{hello} {world}");
    println!("{length}");
    s.clear();
}

// fn print_string(s: &mut String) -> &mut String{
//     s.push_str("world");
//     return s;
// }

fn len_string(s: &String) -> &str {
    return s
}
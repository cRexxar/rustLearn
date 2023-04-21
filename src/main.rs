fn main() {
    let add_closure = |x| x + 1;
    let n = add_closure(5);
    println!("{n}");

    let v1 = vec![1, 2, 3];
    let res:Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", res);
}

fn main() {
    let plus_one = |x: i32| x + 1;
    println!("Hello, world!");
    let answer = do_something(|x| x + 2, 5);
    println!("{}",answer );
}

fn do_something<F>(some_closure: F, val: i32) -> i32
where F: Fn(i32) -> i32 {
    some_closure(val)
}

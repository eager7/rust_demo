#[test]
fn null_pointer() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("{}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() > y.len() { x } else { y };
}

#[test]
fn longest_example() {
    let s1 = "hello".to_string();
    let s2 = "world".to_string();

    let result = longest(&s1,&s2);
    println!("{}", result);
}
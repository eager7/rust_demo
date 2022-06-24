#[test]
fn string() {
    let s: &str = "hello string";
    println!("{:?}", s);

    let str = s.to_string();
    println!("{:?}", str);

    let s: &'static str = "static hello";
    println!("{:?}", s);

    let mut s = String::new();
    println!("new:{:?}", s);
    s = String::from("string new ");
    println!("from:{:?}", s);
    s.push('w');
    s.push_str("orld");
    println!("push:{:?}", s);

    assert_eq!(s.pop(), Some('d'));

    println!("{}", "hello".to_string() + &"world".to_string());
    println!("{}", format!("{}-{}-{}", "a", "b", "c"))
}

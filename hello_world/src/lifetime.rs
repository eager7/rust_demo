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

    let result = longest(&s1, &s2);
    println!("{}", result);
}

fn copy_and_return<'a>(vector: &mut Vec<String>, value: &'a str)->&'a str {
    vector.push(String::from(value));
    value
}

#[test]
fn life_ex() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
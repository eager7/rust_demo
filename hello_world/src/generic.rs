#[test]
fn generic() {
    let list = vec![1,2,3,4,5];
    println!("{:?}", largest(&list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T])-> &T {
    let mut largest = &list[0];

    for item in list {
        largest = if largest > item { largest } else {item};
    }
    return largest;
}
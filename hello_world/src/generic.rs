#[test]
fn generic() {
    let list = vec![1,2,3,4,5];
    println!("{:?}", largest(&list));

    println!("{:?}", largest(&vec!["hello", "world"]));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T])-> &T {
    let mut largest = &list[0];

    for item in list {
        largest = if largest > item { largest } else {item};
    }
    return largest;
}

#[test]
fn generic_struct() {
    struct Point<T> {
        x:T,
        y:T,
    }
    let integer = Point{x:5,y:10};
    let float = Point{x:5.1,y:10.1};

    impl <T>Point<T> {
        fn x(&self) -> &T {
            return &self.x;
        }
    }
    impl Point<f64> {
        fn distance_from_origin(&self)->f64 {
            return (self.x.powi(2)+self.y.powi(2)).sqrt();
        }
    }

    println!("{}", integer.x());
    println!("{}", float.distance_from_origin());
}
use std::cmp::Ordering;

#[test]
fn iter() {
    let ve = vec![1, 2, 3];
    let mut ve_iter = ve.iter();
    for val in ve_iter {
        println!("{:?}", val);
    }

    let vb = vec![1];
    let mut vb_iter = vb.iter();
    assert_eq!(vb_iter.next(), Some(&1));
    assert_eq!(vb_iter.next(), None);
    assert_eq!(0, vb_iter.sum());

    let vc = vec![1, 2, 3, 4];
    let vd: Vec<_> = vc.iter().map(|x| x + 1).collect();
    assert_eq!(vd, vec![2, 3, 4, 5]);
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    return shoes.into_iter().filter(|s| s.size == size).collect();
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: "boot".to_string(),
        },
    ];
    let v = shoes_in_size(shoes, 10);
    assert_eq!(
        v,
        vec![Shoe {
            size: 10,
            style: "sneaker".to_string()
        }]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        return Counter { count: 0 };
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        return None;
    }
}
#[test]
fn calling_next() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
}
#[test]
fn iterator_method() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

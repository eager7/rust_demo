use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...{}", num);
        thread::sleep(Duration::from_secs(2));
        return num;
    });
    if intensity < 25 {
        println!("do {:?}", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("break today");
            return;
        }
        println!("run {:?}", expensive_closure.value(intensity));
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        return Cacher {
            calculation,
            value: HashMap::new(),
        };
    }
    fn value(&mut self, arg: u32) -> u32 {
        let v = self.value.entry(arg).or_insert(arg);
        return *v;
    }
}

#[test]
fn example_closure() {
    let value = 10;
    let number = 7;
    println!("{:?}", generate_workout(value, number));
}
#[test]
fn call_diff_value() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(1, v1);
    assert_eq!(2, v2);
}

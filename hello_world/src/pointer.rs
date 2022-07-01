use crate::pointer::List::Cons;
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

#[test]
fn my_box() {
    let m = MyBox(2);
    println!("{}", *m);

    let y = MyBox("hello");
    println!("{:?}", *(y.deref()));
}

#[derive(Debug)]
struct SmartDrop {
    data: String,
}

impl Drop for SmartDrop {
    fn drop(&mut self) {
        println!("drop object:{}", self.data);
    }
}

#[test]
fn drop_example() {
    let a = SmartDrop {
        data: "a".to_string(),
    };
    println!("ex a:{:?}", a);

    {
        let b = SmartDrop {
            data: "b".to_string(),
        };
        drop(b);
    }
    println!("exit");
}

enum List {
    Cons(i32, std::rc::Rc<List>),
    Nil,
}
use std::rc::Rc;
#[test]
fn list_example() {
    let l = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(List::Nil)))));
    println!("count after creating l = {}", Rc::strong_count(&l));
    let a = Cons(1, std::rc::Rc::clone(&l));
    println!("count after creating l = {}", Rc::strong_count(&l));
    let b = Cons(2, std::rc::Rc::clone(&l));
    println!("count after creating l = {}", Rc::strong_count(&l));
    drop(a);
    println!("count after creating l = {}", Rc::strong_count(&l));
    drop(b);
    println!("count after creating l = {}", Rc::strong_count(&l));
}
fn print_type_name_of<T>(_: T) {
    println!("{:?}", unsafe { std::intrinsics::type_name::<T>() })
}

#[test]
fn ref_example() {
    let r=&1;
    let &a=r; // & 符相当于模式匹配，a匹配1
    let a=*r;

    println!("{:?},{:?},{:?},", r, &a, a);
    print_type_name_of(r); // &i32
    print_type_name_of(&a);// &i32
    print_type_name_of(a);// i32

    let ref a:i32;
    // print_type_name_of(a);// use of possibly-uninitialized `a`
    a = &1;
    print_type_name_of(a);// &i32
}
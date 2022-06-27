use std::ops::Deref;
use crate::pointer::List::Cons;

struct MyBox<T> (T);

impl <T> MyBox<T> {
    fn new(x:T)->MyBox<T> {
        return MyBox(x);
    }
}

impl <T> Deref for MyBox<T> {
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
    data:String,
}

impl Drop for SmartDrop {
    fn drop(&mut self) {
        println!("drop object:{}", self.data);
    }
}

#[test]
fn drop_example() {
    let a = SmartDrop{data:"a".to_string()};
    println!("ex a:{:?}", a);

    {
        let b = SmartDrop{data:"b".to_string()};
        drop(b);
    }
    println!("exit");
}

enum List{
    Cons(i32, std::rc::Rc<List>),
    Nil,
}
use std::rc::Rc;
#[test]
fn list_example() {
    let l = Rc::new(Cons(5,Rc::new(Cons(6,Rc::new(List::Nil)))));
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
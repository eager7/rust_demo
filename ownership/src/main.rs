fn main() {
    println!("Hello, world!");
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s); // s  not found in this scope
    {
        // 移动和克隆
        let s = String::from("hello");
        let s2 = s;
        // println!("{}", s); //  value borrowed here after move
        println!("{}", s2);
        let s3 = s2.clone();
        println!("{},{}", s2, s3);
    }
    {
        // 引用
        let s = String::from("hello");
        let s2 = &s; // 使用引用避免变量移动
        println!("{},{}", s, s2);
        // 引用数据不能修改
        // s2.push_str(" world");// `s2` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        println!("{},{}", s, s2);
    }
    {
        // 可变引用
        let mut s = String::from("hello");
        let s2 = &mut s; // 声明一个可变引用
        s2.push_str(" world");
        println!("{}", s2); // hello world
        println!("{}", s); // hello world 原变量也被改变
                           // println!("{},{}", s, s2); // immutable borrow occurs here 同一时间只能有一个对象的可变引用是可用的
    }
    {
        println!("{:?}", first_world(&String::from("hell")));
    }
    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{}, {}", hello, world);
    }
}

// 返回一个内部指针是不允许的，会导致野指针情况，rust 不允许逃逸
// fn dangle()->&String{
//     let s = String::from("hello");
//     return &s; //  ^ expected named lifetime parameter
// }

fn first_world(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

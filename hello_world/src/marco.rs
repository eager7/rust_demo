#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! log {
    () => {
        println!("\n");
    };
    ($($arg:tt)*) => {{
       println!($($arg)*);
    }};
}

#[test]
fn marco_ex() {
    let a = vec![1, 3];
    println!("{:?}", a);
    log!("hello");
}

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

#[test]
fn error () {
    f1();
}

fn f1() {
    f2();
}

fn f2 () {
    panic!("panic f2")
}

#[test]
fn result () {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file)=>file,
        Err(e)=>match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(ee)=>panic!("{}", ee),
            },
            other_error=>panic!("other"),
        },
    };
}

#[test]
fn unwrap() {
    let f = File::open("h.txt").unwrap();
}

#[test]
fn expect() {
    let f = File::open("h.txt").expect("not found");
}

#[test]
fn read() {
    let ret = read_file();
    println!("{:?}",ret);
}

fn read_file()->Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}
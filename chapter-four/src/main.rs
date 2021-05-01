use std::collections::HashMap;

fn main() {
    //println!("Hello, world!");

    let mut fruits = vec!["1", "2", "3"];
    fruits.insert(1, "4");
    println!("{:?}", fruits);

    let a = "😂😘".to_string();
    println!("{}", a);

    let chars = "😱😡😏😞😳😒😂😘❤️😍😊😁❤️💋💋💋".to_string();
    for ch in  chars.chars() {
        println!("{}", ch);
    }

    let all_nums = 0..;

    for (i, l) in  chars.chars().enumerate() {
        println!("{} {}", i, l);
    }

    let chars_2 = "😱😡😏😞😳😒😂😘❤️😍😊😁❤️💋💋💋".to_string();
    let str_chars2 = chars_2.as_str();
    
    if let Some(sym) = chars_2.chars().find(|x |x == &'💋'){
        println!("char existst {}", sym);
    }


    let mut tv_strings = HashMap::new();
    tv_strings.insert("dima", 1.8);
    let num = tv_strings["dima1"];
    let num2 = tv_strings.get("sasha");

    //println!("{}", num);
    println!("{}", num2.unwrap());


    let mut sq_vec = SquaredVector::new();
    sq_vec.push(1);
    sq_vec.push(2);
    sq_vec.push(3);
    sq_vec.push(4);
    for (i, v) in sq_vec.iter().enumerate() {
        println!("{} {}", i, v);
    }

    let aaa = sq_vec.iter();

    let fib: Vec<_> = fibonacci().take(10).collect();
    //let aaa = ibonacci().iter()
}

struct Fibonacci {
    curr: u32,
    next: u32
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1}
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let old = self.curr;
        self.curr = self.next;
        self.next += old;
        Some(old)
    }
}

use std::ops::Mul;

struct SquaredVector<T> where T:Mul + Copy {
    vec: Vec<T::Output>
}

impl<T> SquaredVector<T> where T:Mul + Copy {
    fn new() -> Self {
        SquaredVector{vec: Vec::new()}
    }
    fn push(&mut self, item: T) {
        self.vec.push(item*item);
    }
}

use std::ops::Deref;
impl<T> Deref for SquaredVector<T> where T:Mul + Copy {
    type Target = [T::Output];
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
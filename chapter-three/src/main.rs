use std::thread;
use std::time::{Instant};

macro_rules! muliply {
    //edge
    ($last:expr) => {
        $last
    };
    //recursive
    ($head:expr, $($tail:expr), +) => {
        $head * muliply!($($tail), +)
    };
}

fn main() {
 //   println!("Hello, world!");   
    let child = thread::spawn(|| {
        println!("hello form new thread!!")
    });

    child.join().expect("failed join");

    let vec_cap: usize = 500000000;
    let mut vector = Vec::with_capacity(vec_cap);

    for i in 0..vec_cap {
        vector.push(1);
    }
    
    let start_1 = Instant::now();
    let pa_sum = paralell_sum(&vector);
    let duration_1 = start_1.elapsed();

    let start_2 = Instant::now();
    let sy_sum = sum_bucket(&vector);
    let duration_2 = start_2.elapsed();

    println!("{} paralell sum in {:?}", pa_sum, duration_1);
    println!("{} one thread sum in {:?}", sy_sum, duration_2);

    let a = muliply!(2,3,4);
    println!("{}", a);
}


fn paralell_sum(range: &[i32]) -> i32 {
    const NUM_THREADS: usize = 4;
    let len = range.len();
    if len < NUM_THREADS {
        return sum_bucket(range);
    } else {
        let bucket_size = len / NUM_THREADS;
        let mut count = 0;
        let mut threads = Vec::new();
        while count + bucket_size < len {
            let bucket = range[count..count+bucket_size].to_vec();
            let thread = thread::Builder::new()
                                                        .name("calc".to_string())
                                                        .spawn(move || sum_bucket(&bucket))
                                                        .expect("failed create new thread");
            threads.push(thread);
            count += bucket_size;
        }
        let mut sum = sum_bucket(&range[count..]);
        for thread in threads {
            sum += thread.join().expect("failed to join");
        }
        sum
    }
}

fn sum_bucket(range: &[i32]) -> i32{
    let mut sum = 0;
    for num in range {
        sum += *num;
    }
    sum
}



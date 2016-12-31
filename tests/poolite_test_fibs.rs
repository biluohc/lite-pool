#![allow(unused_variables)]
extern crate poolite;

use std::time::SystemTime;
use std::time::Duration;
use std::thread;

// cargo test -- --nocapture ,不阻止终端输出。
// To observe the change of CPU/RAM occupy.
const FOR: usize = 1;
#[test]
fn main() {
    println!("Tset poolite !");
    let st = SystemTime::now();
    for _ in 0..FOR {
        fibm();
    }
    let ed = SystemTime::now();
    println!("{:?}", (ed.duration_since(st).unwrap()));

}
fn fibm() {
    let pool = poolite::Pool::new().run();

    let mut count = 0;
    loop {
        if count == 100 {
            break;
        }
        for i in 0..36 {
            // print!("main_loop0: ");
            pool.spawn(Box::new(move || test(count, i)));
        }
        count += 1;
    }
    count = 0;
    println!("loop0 finished ! and sleep 5000 ms ! ");
    thread::sleep(Duration::from_millis(5000));
    loop {
        if count == 100 {
            break;
        }
        for i in 0..32 {
            // print!("main_loop1: ");
            pool.spawn(Box::new(move || test(count, i)));
        }
        thread::sleep(Duration::from_millis(100));
        count += 1;
    }
    println!("loop1 finished ! main finished after sleep 6000 ms ! ");
    thread::sleep(Duration::from_millis(6000));
    fn test(count: i32, msg: i32) {
        // println!("count({})_fib({})={}", count, msg, fib(msg));
        let _ = fib(msg);
    }
    fn fib(msg: i32) -> i32 {
        match msg {
            0...2 => 1,
            x => fib(x - 1) + fib(x - 2),
        }
    }
}
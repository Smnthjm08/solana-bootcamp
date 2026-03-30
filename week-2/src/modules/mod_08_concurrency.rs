use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, spawn};

// mpsc
pub fn process_values(values: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let result: Vec<i32> = values.into_iter().map(|x| x * 2).collect();

        tx.send(result).unwrap();
    });
    let processed = rx.recv().unwrap();

    handle.join().unwrap();

    processed
}

// parallel_sum
pub fn parallel_sum(nums: Vec<i32>) -> i32 {
    // Split, spawn two threads, join, return total
    let mid = nums.len() / 2;
    let left = nums[..mid].to_vec();
    let right = nums[mid..].to_vec();

    let thread_01 = thread::spawn(move || left.iter().sum::<i32>());
    let thread_02 = thread::spawn(move || right.iter().sum::<i32>());

    let sum1 = thread_01.join().unwrap();
    let sum2 = thread_02.join().unwrap();

    sum1 + sum2
}

pub fn concurrent_counter(n: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));

    let mut handlers = vec![];

    for _ in 0..n {
        let increment = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = increment.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

pub fn pipeline(input: Vec<i32>) -> Vec<String> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    let _result1 = thread::spawn(move || {
        let ans: Vec<i32> = input.into_iter().filter(|x| x % 2 == 0).collect();
        tx1.send(ans).unwrap();
    });

    let _result2 = thread::spawn(move || {
        let ans: Vec<i32> = rx1.recv().unwrap().into_iter().map(|x| x * x).collect();
        tx2.send(ans).unwrap();
    });

    let _result3 = thread::spawn(move || {
        let ans: Vec<String> = rx2
            .recv()
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        tx3.send(ans).unwrap();
    });

    rx3.recv().unwrap()
}
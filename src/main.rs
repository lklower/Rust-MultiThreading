use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
    vec,
};

/**
 * main function
 **/
fn main() {
    let (sender, receiver) = mpsc::channel::<String>();
    let receiver_mutex = Arc::new(Mutex::new(receiver));
    let thread_numbers: usize = 4;
    let mut threads: Vec<JoinHandle<()>> = vec![];

    for t in 0..thread_numbers {
        let receiver = Arc::clone(&receiver_mutex);
        let th = thread::spawn(move || loop {
            let data = receiver.lock().unwrap().recv();
            if data.is_ok() {
                print!("Thread-{} received {}.\n", t, data.unwrap());
            } else {
                print!("Thread-{} reading error.\n", t);
                break;
            }
        });
        threads.push(th);
    }

    thread::spawn(move || {
        for i in 0..100 {
            println!("Sending data {} to thread.\n", i);
            sender.send(format!("[data {}]", i)).unwrap();

            if i % 5 == 0 {
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    for t in threads {
        t.join().unwrap();
    }
}

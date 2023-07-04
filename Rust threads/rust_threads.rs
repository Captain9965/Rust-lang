/* 
    A thread is the smallest executable unit of a process:
    we can do this using the std::spawn() function which creates a native os thread. It takes a closure as an argument.
    -> If the main thread completes, all other threads stop regardless of whether they have finished running or not

 */

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main(){
    //create a thread:
    let message = String::from("Running in thread");

    // create a channel for inter-thread communication:
    /*  
        The std::sync::mpsc module provides multiple-producer, single consumer channels
        recv() blocks until a message is received and returns a Result option.
    
    
     */
    let (sender, receiver) = mpsc::channel();

    /*  
    we use move to force the thread to take ownership of any variables used in  the closure
    
     */
    let handle = thread::spawn ( move ||{
        for _i in 0..10{
            // receive message from channel:

            let msg = receiver.recv().unwrap();
            println!("{} -->Received message -> {}", message, msg);
            thread::sleep(Duration::from_millis(2000));
        }
    });

    // main thread:
    for _i in 0..5{
        println!("Running main thread");
        sender.send("sent from main").unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    // calling join on the handle blocks the thread until it terminates: 
    handle.join().unwrap();

    /*
    
     */
}
// use std::sync::Arc;

// fn main() {
//     println!("Hello, world!");
//     let steps = Arc::new(5);
//     let thread = {
//         let steps = steps.clone();
//         std::thread::spawn(move || {
//             for step in 1..=*steps {
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//                 println!("Thread step {}", step);
//             }
//             "Goodbye!"
//         })
//     };
//     println!("Spawned a thread with a step count of {}!", steps);
//     std::thread::sleep(std::time::Duration::from_secs(2));
//     println!("Slept 2 seconds. Now joining thread ...");
//     let result = thread.join().unwrap();
//     println!("Thread returned with {:?}", result);
// }

// use std::sync::mpsc::channel;

// fn main() {
//     println!("Hello, world!");
//     let (sender, receiver) = channel();
//     let thread = std::thread::spawn(move || {
//         let steps = receiver.recv().unwrap();
//         for step in 1..=steps {
//             std::thread::sleep(std::time::Duration::from_secs(1));
//             println!("Thread step {}", step);
//         }
//         "Goodbye!"
//     });
//     println!("Spawned a thread!");
//     let _ = sender.send(5);
//     std::thread::sleep(std::time::Duration::from_secs(2));
//     println!("Slept 2 seconds. Now joining threed ...");
//     let result = thread.join().unwrap();
//     println!("Thread returned with {:?}", result);
// }

use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");
    let steps = Arc::new(Mutex::new(5));
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move || {
            while *steps.lock().unwrap() > 0 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}", steps.lock().unwrap());
                *steps.lock().unwrap() -= 1;
            }
            "Goodbye"
        })
    };
    println!(
        "Spawned a thread with a step count of {}!",
        steps.lock().unwrap()
    );
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Slept 2 seconds. Now joining thread ... ");
    let result = thread.join().unwrap();
    println!("Thread returned with {:?}", result);
}

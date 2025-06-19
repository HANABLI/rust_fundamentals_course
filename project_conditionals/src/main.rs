// fn main() {
//     let proceed = true;
//     if proceed {
//         println!("Proceeding");
//     } else {
//         println!("Not proceeding");
//     }

//     let height = 160;

//     if height >= 180 {
//         println!("Tall");
//     } else if height < 180 && height > 160 {
//         println!("Average");
//     } else {
//         println!("Short");
//     }
// }
// fn main() {
//     //let mut maybe_number = Some(None);
//     let maybe_number = Some(42);
//     if let Some(number) = maybe_number {
//         println!("The number is {}", number);
//     } else {
//         println!("None");
//     }
// }
// fn main() {
//     //let mut i = 0;
//     // while i < 5 {
//     //     println!("i = {}", i);
//     //     i += 1;
//     // }
//     //range and reverse range
//     // for i in (1..=10).rev() {
//     //     println!("i = {}", i);
//     // }
//     // vector
//     // let numbers = vec![1, 2, 3, 4, 5];
//     // for n in numbers {
//     //     println!("n = {}", n);
//     // }
// }

fn main() {
    for i in 1..=10 {
        if i % 2 == 0 {
            // Skip
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            // Exit loop when i is 7
            break;
        }
    }
}

fn print_sum(numbers: &[i32]) {
    let sum: i32 = numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
    println!("No panic, all is good!");
}

fn main() {
    let numbers = [1, 2, 3];
    print_sum(&numbers);
    let mut vec_from_slice = numbers.to_vec();
    vec_from_slice.push(-5);
    // let slice: &[i32] = &vec_from_slice;
    process_numbers(&vec_from_slice);
}

fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..]; // create a slice of all elements in numbers
    println!("slice = {:?}", slice);
}

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    let value = vec.get(index).unwrap();

    println!("The value at index {} is {:?}", index, value);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..]; // creates a slice of all elements in numbers
    slice[0] = 10;
    // This would fail!
    //let other_slice = &numbers[..]
    println!("slice = {:?}", slice);
}

fn main() {
    // slices and vectors are similar. But slices are immutable depending on how tey are borrowed
    ownership();
    modifiable();

    let vec = vec![1, 2, 3, 4, 5];

    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    //Retreive a value at a specific index
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    //Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }
    let usize_index = 3;
    get_item(usize_index);

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("The vector v elements: {:?}", v);

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];  
    v.extend(more_numbers);
    println!("Extended vector elements: {:?}", v);
    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("Append mutable vector elements: {:?}", v);

    v.insert(0, 0);
    println!("Vector with inserted value in index 0 : {:?}", v);
}

// Use a slice to borrow a portion of a collection rather than the whole collection.
// Use a slice to pass around a reference to a sequence of items without copying them.
// Use a slite to access a subset of a collection without copying.

// Use a vector to dynamically grow or shrink your collection.
// Use a vector to own the collection and transfer ownership to another scope.

use std::str;
fn main() {
    let sentence = "the quick brown fox jump over the lazy dog".to_string();
    // Use slicing to ge the first three characters of the sentence
    println!("{}", &sentence[0..4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over th characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel: {}!\n\r", c),
            _ => continue,
        }
    }

    // split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("Vector content: {:?}", words);
}

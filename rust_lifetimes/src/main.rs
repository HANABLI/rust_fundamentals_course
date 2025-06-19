// struct OurIterator<'a> {
//     nums: &'a Vec<i32>,
//     i: usize,
//     x: i32,
// }

// impl<'a> OurIterator<'a> {
//     fn new(nums: &'a Vec<i32>, x: i32) -> Self {
//         Self {
//             nums, i: 0, x
//         }
//     }
// }
struct OurIterator<'a> {
    nums: &'a [i32],
    i: usize,
    x: i32,
}

impl<'a> OurIterator<'a> {
    fn new(nums: &'a [i32], x: i32) -> Self {
        Self { nums, i: 0, x }
    }
}

impl Drop for OurIterator<'_> {
    fn drop(&mut self) {
        println!("Destroying our iterator for x{}", self.x);
    }
}

impl Iterator for OurIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.nums.len() {
            let value = self.nums[self.i];
            self.i += 1;
            Some(value * self.x)
        } else {
            None
        }
    }
}

#[allow(clippy::while_let_on_iterator)]
fn main() {
    let mut nums = vec![1, 2, 3];
    {
        let mut num2 = OurIterator::new(&nums, 2);
        let mut num3 = OurIterator::new(&nums, 5);

        while let Some(n) = num2.next() {
            println!("{}", n);
        }
        drop(num2);
        while let Some(n) = num3.next() {
            println!("{}", n);
        }
        drop(num3);
    }
    println!("Pushing 4 into the vector");
    nums.push(4);
}

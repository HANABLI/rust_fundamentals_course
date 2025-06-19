// fn double(n: i32) -> i32 {
//     n * 2
// }

// fn triple(n: i32) -> i32 {
//     n * 3
// }

// // struct MultiplierFunction {
// //     x: i32,
// // }

// // impl MultiplierFunction {
// //     fn call_fn(&self, n: i32) -> i32 {
// //         n * self.x
// //     }
// // }

// fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 { // call_fn
//     move |n| n * x
// }

fn make_multiplier(mut x: i32) -> impl FnMut(i32) -> i32 {
    // call_fn_mut
    move |n| {
        x += 1;
        n * x
    }
}

// struct Token;

// fn make_multiplier(mut x: i32, token: Token) -> impl FnOnce(i32) -> i32 {
//     move |n| {
//         drop(token);
//         x += 1;
//         n * x
//     }
// }

fn main() {
    let nums = [1, 2, 3];
    let nums_as_iter = nums.into_iter();
    let multiolied = nums_as_iter.map(make_multiplier(4));
    for n in multiolied {
        println!("{}", n);
    }
}

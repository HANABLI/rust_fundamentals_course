use model::Hands;
mod model {

    use std::fmt::Display;

    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }

    pub struct Hands {
        left: Option<Fruit>,
        right: Option<Fruit>,
    }

    impl Display for Fruit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Fruit::Apple => f.write_str("an Apple"),
                Fruit::Banana => f.write_str("a Banana"),
                Fruit::Kiwi => f.write_str("a Kiwi"),
            }
        }
    }

    impl Hands {
        pub fn new() -> Self {
            Hands {
                left: Some(Fruit::Apple),
                right: Some(Fruit::Banana),
            }
        }

        #[allow(clippy::manual_swap)]
        pub fn juggle(mut self) -> Self {
            println!("Let's juggle!");
            let air = self.left;
            self.left = self.right;
            self.right = air;

            println!("After juggling");
            self
        }

        pub fn report(&self) {
            report_item(&self.left, "left");
            report_item(&self.right, "right");
        }
    }

    pub fn report_item<T: Display>(item: &Option<T>, which: &str) {
        match item {
            Some(what) => {
                println!("{} hand is holding {}", which, what)
            }
            _ => println!("{} hand is not holding anything", which),
        }
    }
}

fn main() {
    let mut hands = Hands::new();

    // Hands::report(&hands );
    hands.report();
    hands = Hands::juggle(hands);

    Hands::report(&hands);
}

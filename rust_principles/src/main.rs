use model::Hands;
mod model {

    pub struct Item {
        what: String,
        present: bool,
    }

    pub struct Hands {
        left: Item,
        right: Item,
    }

    impl Hands {
        pub fn new() -> Self {
            Hands {
                left: Item {
                    what: "an apple".to_owned(),
                    present: true,
                },
                right: Item {
                    what: "a banana".to_owned(),
                    present: true,
                },
            }
        }
        #[allow(clippy::manual_swap)]
        pub fn juggle(mut hands: Self) -> Self {
            println!("Let's juggle!");
            let air = hands.left;
            hands.left = hands.right;
            hands.right = air;

            println!("After juggling");
            hands
        }

        pub fn report(&self) {
            Item::report_item(&self.left, "left");
            Item::report_item(&self.right, "right");
        }
    }

    impl Item {
        fn report_item(&self, which: &str) {
            if self.present {
                println!("{} hand is holding {}", which, self.what);
            } else {
                println!("{} hand is not holding anything", which);
            }
        }
    }
}

fn main() {
    let mut hands = Hands::new();

    // Hands::report(&hands);
    hands.report();
    hands = Hands::juggle(hands);

    Hands::report(&hands);
}

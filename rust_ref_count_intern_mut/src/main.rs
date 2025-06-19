// use std::rc::Rc;
// struct Node {
//     value: &'static str,
//     edges: Vec<Rc<Node>>,
// }

// impl Node {
//     fn display(&self) {
//         println!("Value: {}", self.value);
//         for edge in &self.edges {
//             edge.display()
//         }
//         println!("UP");
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("{} is droped.", self.value);
//     }
// }

// fn main() {
//     let e = Rc::new(Node {
//         value: "e",
//         edges: vec![],
//     });
//     let d = Rc::new(Node {
//         value: "d",
//         edges: vec![e.clone()],
//     });
//     let a = Node {
//         value: "a",
//         edges: vec![
//             Rc::new(Node {
//                 value: "b",
//                 edges: vec![d.clone(), e.clone()],
//             }),
//             Rc::new(Node {
//                 value: "c",
//                 edges: vec![d.clone(), e.clone()],
//             }),
//             d.clone(),
//             e.clone(),
//         ]
//     };
//     a.display();
//     drop(a);
//     d.display();
//     println!("That's the end of the graph");
// }
use std::cell::RefCell;
struct Cat {
    times_spoken: RefCell<usize>,
}

impl Cat {
    fn report(&self) {
        println!("I spoke {} times", self.times_spoken.borrow());
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
        *self.times_spoken.borrow_mut() += 1;
        //let n2 = self.times_spoken.borrow_mut();
        //*n += 1;
    }
}

trait Animal {
    fn speak(&self);
}

fn work_with_animal(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let cat = Cat {
        times_spoken: RefCell::new(0),
    };
    work_with_animal(&cat);
    work_with_animal(&cat);
    work_with_animal(&cat);
    cat.report();
}

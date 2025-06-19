#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

struct User {
    name: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(name: String, email: String, uri: String) -> Self {
        User {
            name,
            email,
            uri,
            active: true,
        }
    }
    fn desactivate(&mut self) {
        self.active = false;
    }
}

struct Point(i32, i32, i32);

fn main() {
    let hatem = Person {
        first_name: "Hatem".to_string(),
        last_name: "Nabli".to_string(),
        age: None,
    };
    println!(
        " The Person created is:\n\r First Name: {}\n\r Last Name: {}\n\r Age: {:?}\n\r",
        hatem.first_name, hatem.last_name, hatem.age
    );
    let mut new_user = User::new(
        String::from("Hatem"),
        String::from("hatem@exemple.com"),
        String::from("https:://hatem"),
    );
    println!(
        " The created user's name is: {} with infos:\n\r email: {}\n\r uri: {}",
        new_user.name, new_user.email, new_user.uri
    );
    println!(" Account {} status is: {}", new_user.name, new_user.active);
    new_user.desactivate();
    println!(" Account {} status is: {}", new_user.name, new_user.active);

    let new_point = Point(10, 20, 30);

    println!(
        " Point cordinates: {}, {}, {}",
        new_point.0, new_point.1, new_point.2
    );
}

// fn main() {
//     println!("{:?}", Person {
//         first_name: "Hatem".to_string(),
//         last_name: "Nabli".to_string(),
//         age: 31,
//     });
// }

use std::collections::HashMap;

struct User {
    username: String,
    email: String,
    login_count: u64,
    is_active: bool,
}

struct RedFox {
    enemy: bool,
    life: u8,
}

trait Noisy {
    fn make_noise(&self) -> &str;
}

impl Noisy for RedFox {
    // fn new() -> Self {
    //     Self {
    //         enemy : true,
    //         life : 2,
    //     }
    // }
    fn make_noise(&self) -> &str {
        "test"
    }

}

impl User {
    fn details(&self) -> String {
       self.email.clone()
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}


impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();

    let mut some_number: Option<i32> = Some(0);
    // let mut letter: Option<i8> = Some(7);
    let mut absent_number: Option<i32> = None;
    // let i =  255;

    // if let Some(x) = absent_number {

    //     println!("255 here {} {}", x, x == 255);
    // } else {
    //     println!("else here")
    // }
    // let q = match absent_number {
    //     Some(x) => x,
    //     None => 0
    // };

    let b: Option<i32> = None;

    if b.is_some() {
        for i in b {
            println!("{}", i);
        }
    } else {
        println!("is is none");
    }

    // println!("{}" , q);

    println!("{:?} - {:?}", some_number.take(), absent_number.take());
}

fn mai() {
        let mut devs: Vec<&str> = vec!["fritz", "noble"];
        let mut dev: HashMap<&str, &str> = HashMap::new();

        dev.insert("name", "noble");
        dev.insert("career", "software dev");

        // let have_name = dev.remove("name").unwrap();

        devs.push("pius");
        devs.push("fortune");

        println!("{:?}", devs);
        
        println!("{:?}", dev);

        let have_name = dev.remove("name").unwrap();

        println!("{:?}", have_name);
        println!("{:?}", dev);




        return;

    // let user1: User = User {
    //     username: String::from("noble"),
    //     email: String::from("noble@gmail.com"),
    //     login_count: 24,
    //     is_active: true,
    // };

    // let user1: User = build_user("noble@gmail.com", "noble");

    // let email = user1.email;

    // println!("{}", user1.details());

    // let red_fox = RedFox::new();

    // println!("{}", red_fox.enemy);

        // let red_fox = RedFox::new();
        // println!("{}", red_fox.enemy);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        login_count: 0,
        is_active: true,
    }
}
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
    fn make_noise() -> &str {
        {"test"}
    }

}

impl User {
    fn details(&self) -> String {
       self.email.clone()
    }
}

fn main() {
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

        let red_fox = RedFox::new();
        println!("{}", red_fox.enemy);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        login_count: 0,
        is_active: true,
    }
}
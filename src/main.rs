extern crate tistory;

use std::env;

const USAGE: &str = "USAGE:
tistory login
tistory logout

tistory category <blog_name>
tistory post <blog_name> <category_id> <file_path>";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{}", USAGE);
        return;
    }

    let command = &args[1];
    match &command[..] {
        "login" => {
            tistory::login();
        },
        "logout" => {
            tistory::logout();
        },
        "category" => {
            if args.len() < 3 {
                println!("{}", USAGE);
                return;
            }
            let blog_name = &args[2];
            tistory::category(blog_name);
        },
        "post" => {
            if args.len() < 5 {
                println!("{}", USAGE);
                return;
            }
            let blog_name = &args[2];
            let category_id = &args[3];
            let file_path = &args[4];
            tistory::post(&blog_name[..],
                          &category_id[..],
                          &file_path[..]);
        },
        _ => {
            println!("{}", USAGE);
        }
    };
}

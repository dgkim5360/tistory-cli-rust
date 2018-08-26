extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate comrak;

mod secret;
mod oauth2;
mod client;

pub fn login() {
    println!("Checking the credential file ...");
    let path = secret::get_credentials_path_from_env();
    let credentials = secret::read_credentials(&path[..]);

    println!("Checking the redirect URI ...");
    println!("Done: http://localhost:{}",
             credentials.redirect_port);

    println!("Checking the client ID ...");
    if credentials.client_id == "" {
        panic!("No client id provided.");
    }
    println!("Done.");

    println!("Checking the client secret ...");
    if credentials.client_secret == "" {
        panic!("No client secret provided.");
    }
    println!("Done.");

    println!("Checking the access token ...");
    if credentials.access_token == "" {
        println!("The access token does not exist.
Please visit the following login URL.");
        println!(
            "https://www.tistory.com/oauth/authorize?response_type=code&client_id={}&redirect_uri=http://localhost:{}",
            credentials.client_id,
            credentials.redirect_port,
        );
        oauth2::run(credentials.redirect_port);
    }
    println!("Done.");
}

pub fn logout() {
    println!("Checking the credential file ...");
    let path = secret::get_credentials_path_from_env();
    let mut credentials = secret::read_credentials(&path[..]);

    credentials.access_token = String::new();
    secret::update_credentials(&path[..], credentials);
    println!("Successfully removed the access token.");
}

pub fn category(blog_name: &str) {
    println!("Checking the credential file ...");
    let path = secret::get_credentials_path_from_env();
    let credentials = secret::read_credentials(&path[..]);

    if credentials.access_token == "" {
        panic!("The access token is not provided. Please login first.");
    }
    client::category(credentials, blog_name);
}

pub fn post(blog_name: &str, category_id: &str, md_path: &str) {
    println!("Checking the credential file ...");
    let path = secret::get_credentials_path_from_env();
    let credentials = secret::read_credentials(&path[..]);

    if credentials.access_token == "" {
        panic!("The access token is not provided. Please login first.");
    }
    client::post(credentials, blog_name, category_id, md_path);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}

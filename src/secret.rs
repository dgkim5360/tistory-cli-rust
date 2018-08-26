use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::path::PathBuf;

pub struct Credentials {
    pub redirect_port: u16,
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
}

pub fn get_credentials_path_from_env() -> String {
    let path = match env::var("TISTORY_CLI") {
        Ok(path) => {
            path
        },
        Err(_error) => {
            let home = match env::home_dir() {
                Some(path) => path,
                None => {
                    panic!("It is impossible to get your home directory.");
                },
            };
            String::from(home.join(".tistory").to_str().unwrap())
        }
    };

    let pathbuf = PathBuf::from(&path[..]);
    if !pathbuf.is_file() {
        panic!("The credential file does not exist.
Please make sure the file exists at ~/.tistory or
set the environmental variable TISTORY_CLI.");
    }
    path
}

pub fn read_credentials(path_str: &str) -> Credentials {
    let f_credentials = File::open(path_str)
        .expect("file not found");
    let mut reader = BufReader::new(f_credentials);

    let mut line = String::new();

    let _len = reader.read_line(&mut line);
    let redirect_port: u16 = line.trim().parse()
        .expect("Port should be an integer.");
    line.clear();

    let _len = reader.read_line(&mut line);
    let client_id = line.trim().to_string();
    line.clear();

    let _len = reader.read_line(&mut line);
    let client_secret = line.trim().to_string();
    line.clear();

    let _len = reader.read_line(&mut line);
    let access_token = line.trim().to_string();
    line.clear();

    Credentials {
        redirect_port,
        client_id,
        client_secret,
        access_token,
    }
}

pub fn update_credentials(path_str: &str, credentials: Credentials) {
    let mut f_credentials = File::create(path_str).unwrap();
    writeln!(f_credentials, "{}", credentials.redirect_port).unwrap();
    writeln!(f_credentials, "{}", credentials.client_id).unwrap();
    writeln!(f_credentials, "{}", credentials.client_secret).unwrap();
    writeln!(f_credentials, "{}", credentials.access_token).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_credentials_path() {
        env::set_var("TISTORY_CLI", "tests/data/.tistory-empty");
        assert_eq!(get_credentials_path_from_env(), "tests/data/.tistory-empty");
    }

    #[test]
    fn read_full_credentials() {
        let cred = read_credentials("tests/data/.tistory-with-token");
        assert_eq!(cred.redirect_port, 8888);
        assert_eq!(cred.client_id, "CLIENT_ID");
        assert_eq!(cred.client_secret, "CLIENT_SECRET");
        assert_eq!(cred.access_token, "ACCESS_TOKEN");
    }

    #[test]
    fn read_partial_credentials() {
        let cred = read_credentials("tests/data/.tistory-without-token");
        assert_eq!(cred.redirect_port, 8888);
        assert_eq!(cred.client_id, "CLIENT_ID");
        assert_eq!(cred.client_secret, "CLIENT_SECRET");
        assert_eq!(cred.access_token, "");
    }

    #[test]
    #[should_panic(expected = "Port should be an integer")]
    fn read_empty_credentials() {
        let _cred = read_credentials("tests/data/.tistory-empty");
    }
}

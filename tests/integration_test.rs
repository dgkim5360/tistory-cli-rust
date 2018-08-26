extern crate tistory;

use std::env;

#[test]
#[should_panic(expected = "The credential file does not exist.")]
fn testcase_no_credentials() {
    env::set_var("TISTORY_CLI", "does/not/exist/.tistory");
    tistory::login();
}

#[test]
#[should_panic(expected = "Port should be an integer.")]
fn testcase_empty_credentials() {
    env::set_var("TISTORY_CLI", "tests/data/.tistory-empty");
    tistory::login();
}

#[test]
#[ignore]
fn testcase_login() {
    env::set_var("TISTORY_CLI", "tests/data/.tistory-without-token");
    // Currently IDK how to test the local oauth2 server ...
    tistory::login();
}

#[test]
#[ignore]
fn testcase_real_login() {
    env::set_var("TISTORY_CLI", "tests/data/.tistory-real");
    // Also output in the stdout ...
    tistory::login();
}

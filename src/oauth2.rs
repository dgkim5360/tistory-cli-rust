use hyper::{Client, Body, Request, Response, Server, Uri};
use hyper::rt::{self, Future, Stream};
use hyper::service::service_fn_ok;
use hyper_tls::HttpsConnector;

use super::secret;

fn get_access_token(auth_code: &str) -> impl Future<Item=String, Error=()> {
    let credentials = secret::read_credentials("tests/data/.tistory-real");
    let uri: Uri =
        format!(
            "https://www.tistory.com/oauth/access_token?\
            code={}&client_id={}&client_secret={}&\
            redirect_uri=http://localhost:{}&grant_type=authorization_code",
            auth_code,
            credentials.client_id,
            credentials.client_secret,
            credentials.redirect_port,
        )
        .parse()
        .unwrap();

    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, Body>(https);

    client
        .get(uri)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .map_err(|e| {
            eprintln!("Error {}", e);
        })
        .and_then(|body| {
            let v = body.to_vec();
            let resp_str = String::from_utf8_lossy(&v).to_string();
            Ok(resp_str)
        })
}

fn get_auth_code_and_then_token(req: Request<Body>) -> Response<Body> {
    let uri = req.uri();
    let uri_queries = uri.query().unwrap();

    let mut auth_code: Option<String> = None;
    let iter_queries = uri_queries.split('&');
    for query in iter_queries {
        if query.starts_with("code") {
            let vec_code: Vec<&str> = query.split('=').collect();
            auth_code = Some(String::from(vec_code[1]));
            break;
        }
    }

    match auth_code {
        Some(code) => {
            let fut = get_access_token(&code[..])
                .map(|resp_token| {
                    let vec_token: Vec<&str> = resp_token.split('=').collect();
                    let token = String::from(vec_token[1]);
                    let path = secret::get_credentials_path_from_env();
                    let mut credentials = secret::read_credentials(&path[..]);
                    credentials.access_token = token;
                    secret::update_credentials(&path[..], credentials);
                });
            rt::spawn(fut);
        },
        None => {
            panic!("Failed to get authorization code.");
        }
    }

    Response::new(Body::from("<html>
<head><title>Tistory-cli</title></head>
<body>
<h1>Successfully Got the Access Token</h1>
<p>Just go back to your terminal and press CTRL-C to stop the server :)</p>
</body>
</html>"))
}

pub fn run(port: u16) {
    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = || {
        service_fn_ok(get_auth_code_and_then_token)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    rt::run(server);
}

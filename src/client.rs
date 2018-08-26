use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use hyper::{header, Client, Body, Method, Request, Uri};
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use serde_json;
use comrak::{markdown_to_html, ComrakOptions};

use super::secret::Credentials;

#[derive(Deserialize, Debug)]
struct CategoryMessage {
    tistory: TistoryCategoryMessage,
}

#[derive(Deserialize, Debug)]
struct TistoryCategoryMessage {
    status: String,
    item: TistoryCategorySubMessage,
}

#[derive(Deserialize, Debug)]
struct TistoryCategorySubMessage {
    url: String,
    #[serde(rename="secondaryUrl")]
    secondary_url: String,
    categories: Vec<TistoryCategory>,
}

#[derive(Deserialize, Debug)]
struct TistoryCategory {
    id: String,
    name: String,
    parent: String,
    label: String,
    entries: String,
    #[serde(rename="entriesInLogin")]
    entries_in_login: String,
}

#[derive(Deserialize, Debug)]
struct WriteMessage {
    tistory: TistoryWriteMessage,
}

#[derive(Deserialize, Debug)]
struct TistoryWriteMessage {
    status: String,
    #[serde(rename="postId")]
    post_id: String,
    url: String,
}

pub fn category(credentials: Credentials, blog_name: &str) {
    let fut = get_category(&credentials.access_token[..], blog_name)
        .map(|categories| {
            println!("ID\tCATEGORY");
            println!("--\t--------");
            for category in categories {
                println!("{}\t{}", category.id, category.name);
            }
        });
    rt::run(fut);
}

fn get_category(access_token: &str, blog_name: &str)
    -> impl Future<Item=Vec<TistoryCategory>, Error=()> {
    let uri: Uri =
        format!(
            "https://www.tistory.com/apis/category/list\
            ?access_token={}&blogName={}&output=json",
            access_token,
            blog_name,
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
            let json: CategoryMessage = serde_json::from_slice(&body)
                .unwrap();
            Ok(json.tistory.item.categories)
        })
}

#[derive(Debug)]
struct TistoryPost {
    title: String,
    slogan: String,
    content_html: String,
}

fn read_markdown(md_path: &str) -> TistoryPost {
    let f_md = File::open(md_path)
        .expect("Markdown file not found.");

    let path = Path::new(md_path);
    let slogan = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let slogan = String::from(slogan);

    let mut reader = BufReader::new(f_md);
    let mut lines = String::new();

    let _len = reader.read_line(&mut lines);
    let title = lines.trim().to_string();
    lines.clear();

    let len = reader.read_line(&mut lines).unwrap();
    if len > 1 {
        panic!("Invalid format: \
               The title and the contents should be \
               separated by a blank line.");
    }
    lines.clear();

    let _len = reader.read_to_string(&mut lines);
    let content_md = lines.trim().to_string();

    let content_html = markdown_to_html(
        &content_md[..],
        &ComrakOptions::default()
    );

    TistoryPost {
        title,
        slogan,
        content_html,
    }
}

pub fn post(
    credentials: Credentials,
    blog_name: &str,
    category_id: &str,
    md_path: &str
) {
    let post: TistoryPost = read_markdown(md_path);

    let fut =
        post_post(&credentials.access_token[..],
                  blog_name,
                  &post.title[..],
                  category_id,
                  &post.content_html[..],
                  &post.slogan[..])
        .map(|post_id| {
            println!("Successfully posted with ID: {}", post_id);
        });
    rt::run(fut);
}

fn post_post(access_token: &str,
             blog_name: &str,
             title: &str,
             category_id: &str,
             content: &str,
             slogan: &str) -> impl Future<Item=u32, Error=()> {
    let uri: Uri =
        "https://www.tistory.com/apis/post/write"
        .parse()
        .unwrap();
    let body = format!(
        "access_token={}&blogName={}&title={}&category={}&\
        content={}&slogan={}&output=json",
        access_token,
        blog_name,
        title,
        category_id,
        content,
        slogan,
    );

    let mut req = Request::new(Body::from(body));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/x-www-form-urlencoded")
    );

    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, Body>(https);

    client
        .request(req)
        .and_then(|res| {
            println!("STATUS: {}", res.status());
            res.into_body().concat2()
        })
        .map_err(|e| {
            eprintln!("Error {}", e);
        })
        .and_then(|body| {
            let json: WriteMessage = serde_json::from_slice(&body).unwrap();
            let post_id: u32 = json.tistory.post_id.parse().unwrap();
            Ok(post_id)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::secret;

    #[test]
    fn test_category() {
        let credentials = secret::read_credentials("tests/data/.tistory-real");
        category(credentials, "dgkim5360");
    }

    #[test]
    fn test_markdown() {
        read_markdown("tests/data/tistory-cli-test.md");
    }

    #[test]
    fn test_post() {
        let credentials = secret::read_credentials("tests/data/.tistory-real");
        post(credentials,
             "dgkim5360",
             "991057",
             "tests/data/tistory-cli-test.md");
    }
}

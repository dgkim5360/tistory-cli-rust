# Tistory-CLI

A command line tool for Tistory, the Korean blogging platform. It conveys Markdown files to your Tistory blog.

This project is currently in alpha stage and supports the minimal features: To post an article.

한국 Tistory 사용자를 위한 도구이므로 앞으로 한글로 설명합니다.

본인의 Rust 공부를 위해 작성한 코드로, [Python implementation](https://github.com/dgkim5360/tistory-cli-python)과 다르게 libsecret을 사용해서 운영체제 레벨의 credential 저장을 하지 않고 plain text 파일로 저장하게 한 상태입니다. 따라서 실제 사용을 위해서는 Python 버전을 사용하는 것을 추천합니다.

## Installation

소스에서 직접 설치할 수 있습니다.
```sh
$ git clone https://github.com/dgkim5360/tistory-cli-rust.git
$ cd tistory-cli-rust
$ cargo install
```

## Features and Non-features

* 본인의 client/secret key를 사용해서 직접 로그인하므로, 외부 서비스에 비밀 정보를 남기지 않습니다.
* Plain text로 저장된 credential을 사용하므로 __매우 위험합니다__.
* Git commit message 스타일로 제목/내용 구분해서 Markdown 파일을 작성해서 Tistory에 비공개글로 업로드합니다.
* 카테고리와 slug는 설정 가능합니다.
* 공개글로 업로드 __하지 않습니다__.
* 이미지 업로드 __하지 않습니다__.
* 태그 추가 __하지 않습니다__.

## Oauth Setting

0. `~/.tistory` 파일에 차례대로 포트, client ID, client secret를 적습니다. 다른 경로로 지정하고 싶을 경우 환경 변수 `TISTORY_CLI`로 경로를 정해주면 됩니다. 아래는 credential 파일의 예시입니다.
  ```
  8888                                  # redirection uri의 포트
  abcedfghijklmnopqrstuv                # client ID
  zyxwvutsrqponmlkjihgfdecba1234567890  # client secret
  ```
1. Tistory --> 가이드 --> 오픈API 가이드 --> 클라이언트 등록
2. Callback 경로를 `http://localhost:8888`로 설정해서 클라이언트 등록을 완료합니다.
3. `tistory login` 실행하면, 로그인할 페이지를 콘솔에 띄워줍니다.
4. 브라우저로 해당 주소로 들어가서 Tistory 로그인을 해서 API 토큰을 받아옵니다.
5. 콘솔에 다음과 같이 나오면 로그인이 끝납니다.

## Usage

```sh
$ tistory
USAGE:
    tistory login
    tistory logout

    tistory category <blog_name>
    tistory post <blog_name> <category_id> <file_path>
```

* `tistory login`: 모든 비밀 정보가 있는 지 확인하고 없으면 로그인을 시킵니다.
* `tistory logout`: Access token만을 지웁니다.
* `tistory category`: 해당 블로그의 카테고리 아이디 정보를 받아옵니다.
* `tistory post`: 파일을 읽어서 해당 블로그 및 카테고리에 비공개 글로 업로드합니다.

## Step-by-step Tutorial

1. 로그인을 합니다 (Oauth Setting 참조).
2. `tistory category <blog_name>`을 통해 본인 블로그의 카테고리 아이디를 확인합니다.  
  ```bash
  $ tistory category dgkim5360
  ID      Name
  --      ----
  880607  HTML+CSS
  880608  Django
  886608  Bootstrap
  892553  Python
  892554  Javascript
  894029  GNU-Linux
  896354  뻘글
  897315  Cloud
  907471  etc
  908711  Front-end
  912687  Machine Learning
  931755  NBA
  933686  Elastic
  937655  Flask
  951477  Javascript
  959045  Travelogue
  990157  Rust
  ```
3. `<slug>.md` 파일을 Git commit message처럼 작성합니다 (첫 줄 제목, 한 줄 띄우고 내용 시작). 글 내용은 Markdown 형식에 맞추어 작성합니다. (파일명: `tistory-cli-test.md`)
  ```markdown
  [제목]Hello Tistory!

  # Tistory-cli 테스트 중입니다.

  아래는 list
  * hello
    * tistory
    * bye
  * tistory

  1. hello
    1. tistory
    1. bye
  1. tistory

  이것은 [링크](http://dgkim5360.tistory.com)

  Inline `code text`도 써봅니다.
  ```
4. `tistory post <blog_name> <category_id> <slug>.md`로 포스팅을 한 후, Tistory에서 확인합니다. 작업이 완료되면 Tistory 시스템의 ID값을 알려줍니다. (아래 예제에서는 203이 붙었습니다).
  ```bash
  $ tistory post dgkim5360 896354 path/to/tistory-cli-test.md
  Checking the credentials file ...
  STATUS: 200 OK
  Successfully posted with ID: 203
  ```
5. 추가적으로 필요한 이미지 업로드, 태그 추가, 공개 설정은 Tistory에서 해결합니다.

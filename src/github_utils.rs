use std::io::Read;
use sapper::Error as SapperError;
use serde_json;
use serde_urlencoded;
use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper::header::Headers;
use hyper::header::ContentType;

use std::thread;
use std::sync::mpsc;
use serde_derive::Deserialize;

use crate::dataservice::user::GithubUserInfo;


pub fn create_https_client() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}

pub fn get_github_token(code: &str, client_id: String, client_secret: String) -> Result<String, SapperError> {
    let _code = code.to_owned();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let client = create_https_client();

        let params = serde_urlencoded::to_string([
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", &_code[..]),
            ("accept", "json"),
        ]).unwrap();

        let ret = client
            .post("https://github.com/login/oauth/access_token")
            .header(ContentType::form_url_encoded())
            .body(&params)
            .send()
            .map_err(|e| SapperError::Custom(format!("hyper's io error: '{}'", e)))
            .and_then(|mut response| {
                let mut body = String::new();
                response
                    .read_to_string(&mut body)
                    .map_err(|e| SapperError::Custom(format!("read body error: '{}'", e)))
                    .map(|_| body)
            })
            .and_then(|ref body| {
                #[derive(Deserialize)]
                struct Inner {
                    access_token: String,
                }
                serde_urlencoded::from_str::<Inner>(body)
                    .map_err(|_| SapperError::Custom(String::from("No permission")))
                    .map(|inner| inner.access_token)
            });

        tx.send(ret).unwrap();
    });

    let received: Result<String, SapperError> = rx.recv().unwrap();
    println!("Got: {:?}", received);

    received
}

pub fn get_github_user_info(raw_token: &str) -> Result<GithubUserInfo, SapperError> {
    let token = serde_urlencoded::to_string([("access_token", raw_token)]).unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let client = create_https_client();

        let user_url = format!("https://api.github.com/user?{}", token);

        let mut header = Headers::new();
        header.append_raw("User-Agent", b"rustcc".to_vec());

        let ret = client
            .get(&user_url)
            .headers(header)
            .send()
            .map_err(|e| SapperError::Custom(format!("hyper's io error: '{}'", e)))
            .and_then(|mut response| {
                let mut body = String::new();
                response
                    .read_to_string(&mut body)
                    .map_err(|e| SapperError::Custom(format!("read body error: '{}'", e)))
                    .map(|_| body)
            })
            .and_then(|ref body| {
                serde_json::from_str::<serde_json::Value>(body)
                    .map_err(|e| SapperError::Custom(format!("read body error: '{}'", e)))
                    .and_then(|inner| {
                        let account = match inner["login"].as_str() {
                            Some(data) => data.to_string(),
                            None => {
                                return Err(SapperError::Custom(String::from("read body error")))
                            }
                        };
                        let github_address = match inner["html_url"].as_str() {
                            Some(data) => data.to_string(),
                            None => {
                                return Err(SapperError::Custom(String::from("read body error")))
                            }
                        };
                        Ok(GithubUserInfo {
                            account,
                            github_address
                        })
                    })
            });

        tx.send(ret).unwrap();
    });

    let received: Result<GithubUserInfo, SapperError> = rx.recv().unwrap();
    println!("Got: {:?}", received);

    received
}

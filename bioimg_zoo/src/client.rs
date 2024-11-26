use std::io::Read;
use std::str::FromStr;

use bioimg_spec::rdf::HttpUrl;

use crate::collection::ZooNickname;
use crate::{BadResponse, APPLICATION_JSON};
use crate::read_json_response;
use crate::auth::{Seconds, UserToken};


#[derive(serde::Serialize)]
pub enum ClientMethod{
    #[serde(rename="put_object")]
    PutObject,
    #[serde(rename="get_object")]
    GetObject
}

#[derive(serde::Serialize)]
pub struct PresignedUrlPayload<'path>{
    path: &'path camino::Utf8Path,
    client_method: ClientMethod,
    expiration: Seconds,
}

#[derive(derive_more::AsRef, derive_more::Deref, derive_more::Display)]
pub struct PresignedUrl{
    url: murl::Url,
}

pub struct Client{
    user_token: UserToken,
}

impl Client{
    pub fn new(user_token: UserToken) -> Self{
        Self{user_token}
    }

    pub fn presigned_url_request(
        &self, nickname: &ZooNickname, expiration: Seconds, client_method: ClientMethod
    ) -> http::Request<Vec<u8>>{
        let mut path = camino::Utf8PathBuf::from("models");
        path.push(nickname.to_string());
        println!("Generating presigned url with this path: {path}");

        let auth_header = self.user_token.as_header();
        http::Request::builder()
            .method(http::Method::POST)
            .uri("https://hypha.aicell.io/public/services/s3-storage/generate_presigned_url")
            .header(http::header::CONTENT_TYPE, APPLICATION_JSON)
            .header(auth_header.0, auth_header.1)
            .body(serde_json::to_vec(
                &PresignedUrlPayload{
                    path: &path,
                    client_method,
                    expiration,
                }
            ).unwrap())
            .unwrap()
    }

    pub fn parse_presigned_url_resp(&self, resp: &http::Response<Vec<u8>>) -> Result<PresignedUrl, BadResponse>{
        let raw: String = read_json_response(&resp)?;
        let url = murl::Url::from_str(&raw).map_err(|err| BadResponse::ParsingError {
            expected_type_name: std::any::type_name::<HttpUrl>() , source: Box::new(err)
        })?;
        Ok(PresignedUrl { url })
    }

    pub fn write_to_bucket_request<R: Read>(&self, url: &PresignedUrl, reader: R) -> http::Request<R>{
        //bucket url is presigned (and from a different host) and needs no auth header
        http::Request::builder()
            .method(http::Method::PUT)
            .uri(url.to_string())
            .body(reader)
            .unwrap()
    }

    pub fn stage_model_request(&self, nickname: &ZooNickname, presigned_url: &PresignedUrl) -> http::Request<[u8;0]>{
        let hypha_workspace = self.user_token.to_hypha_workspace_name();

        let url = murl::Url{
            scheme: murl::Scheme::Https,
            host: murl::Host{
                name: "hypha".parse().unwrap(),
                domains: vec![
                    "aicell".parse().unwrap(),
                    "io".parse().unwrap(),
                ],
            },
            path: camino::Utf8PathBuf::from(format!("/ws-user-github|478667/services/bioimageio-uploader-service/stage")),
            query: std::collections::BTreeMap::from([
                ("resource_path".to_owned(), nickname.to_string()),
                ("package_url".to_owned(), presigned_url.to_string()),
            ]),
            port: None,
            fragment: None,
        };

        let auth_header = self.user_token.as_header();
        http::Request::builder()
            .method(http::Method::GET)
            .uri(url.to_string())
            .header(auth_header.0, auth_header.1)
            .body([])
            .unwrap()
    }
}

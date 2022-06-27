// use crate::error::*;
use crate::error::Result;
use reqwest::redirect::Policy;
use reqwest::StatusCode;
use reqwest::multipart;
use serde::Serialize;
use std::collections::HashMap;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36 Edg/97.0.1072.69";

#[derive(Default, Clone, Debug)]
pub struct CookieJar {
    pub inner: HashMap<String, String>,
}

impl CookieJar {
    fn parse_cookie(cookie: &str) -> Option<(&str, &str)> {
        // JSESSIONID=xSiUKpqm0lmjhDXB41_hhyxiNUa69u4xMnHkFOFS61E6VZ6Osp7S!-1266297679; path=/; HttpOnly
        cookie.split_once(';').and_then(|s| s.0.split_once('='))
    }

    pub fn from_str(cookie_str: &str) -> Self {
        let mut jar = Self {
            inner: HashMap::new(),
        };
        cookie_str.split_inclusive(";").for_each(|line| jar.append(line));
        jar
    }

    pub fn append(&mut self, cookie: &str) {
        if let Some((k, v)) = Self::parse_cookie(cookie) {
            // This method will override the old one if k already exists.
            self.inner.insert(k.to_string(), v.to_string());
        }
    }

    pub fn to_string(&self) -> Option<String> {
        if self.inner.is_empty() {
            return None;
        }
        let result = self
            .inner
            .iter()
            .fold(String::new(), |s, (k, v)| s + &*format!("{}={};", k, v));
        return Some(result);
    }
}

/// 会话. 用于在 Http 连接上虚拟若干不同用户的会话.
#[derive(Clone)]
pub struct Session {
    /// 会话用的连接
    client: reqwest::Client,
    ///
    redirect: bool,
    /// Cookie 存储
    pub cookie_jar: CookieJar,
}

impl Session {
    pub fn new(redirect: bool) -> Result<Session> {
        let client_builder = reqwest::Client::builder().redirect(Policy::none());
        // if let Some(proxy) = &CONFIG.get().unwrap().http_proxy {
        //     client_builder = client_builder
        //         .proxy(reqwest::Proxy::http(proxy).unwrap())
        //         .proxy(reqwest::Proxy::https(proxy).unwrap())
        //         .danger_accept_invalid_certs(true);
        // }
        let client = client_builder.build()?;
        Ok(Session {
            redirect,
            client,
            cookie_jar: CookieJar::default(),
        })
    }

    async fn request(
        &mut self,
        mut builder: reqwest::RequestBuilder,
        headers: Option<&Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response> {
        builder = builder.header("User-Agent", DEFAULT_USER_AGENT);
        if let Some(cookie) = self.cookie_jar.to_string() {
            builder = builder.header("Cookie", cookie);
        }
        if let Some(headers) = headers {
            for header in headers.iter() {
                builder = builder.header(header.0, header.1)
            }
        }

        let response = builder.send().await?;

        response
            .headers()
            .get_all("Set-Cookie")
            .iter()
            .for_each(|cookie| self.cookie_jar.append(cookie.to_str().unwrap()));

        Ok(response)
    }
    pub async fn get(
        &mut self,
        url: &str,
        headers: Option<&Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response> {
        let mut target = url;
        let mut response: reqwest::Response;
        if self.redirect {
            loop {
                response = self.request(self.client.get(target), headers).await?;
                if response.status() == StatusCode::FOUND {
                    target = response
                        .headers()
                        .get("Location")
                        .unwrap()
                        .to_str()
                        .unwrap();
                } else {
                    break Ok(response);
                }
            }
        } else {
            self.request(self.client.get(target), headers).await
        }
    }

    pub async fn post<T: Serialize + ?Sized>(
        &mut self,
        url: &str,
        form: Option<&T>,
        headers: Option<&Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response> {
        let mut builder = self.client.post(url);
        if let Some(form) = form {
            builder = builder.form(form);
        }
        self.request(builder, headers).await
    }

    pub async fn multipart(
        &mut self,
        url: &str,
        form: multipart::Form,
        headers: Option<&Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response> {
        let builder = self.client.post(url).multipart(form);
        self.request(builder, headers).await
    }
}

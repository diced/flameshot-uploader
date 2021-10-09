use std::{
  collections::HashMap,
  fs::{self, read_to_string},
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

use chrono::Local;
use regex::Regex;
use reqwest::{
  blocking::{multipart::Form, Client, Request},
  header::{HeaderMap, HeaderName},
};
use serde::{Deserialize, Serialize};

use crate::{config::Config, Res};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SXCUBody {
  MultipartFormData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SXCURequestMethod {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SXCU {
  #[serde(rename = "RequestMethod")]
  pub request_method: SXCURequestMethod,

  #[serde(rename = "RequestURL")]
  pub request_url: String,

  #[serde(rename = "Parameters")]
  pub parameters: Option<HashMap<String, String>>,

  #[serde(rename = "Headers")]
  pub headers: Option<HashMap<String, String>>,

  #[serde(rename = "Body")]
  pub body: SXCUBody,

  #[serde(rename = "Arguments")]
  pub query: Option<HashMap<String, String>>,

  #[serde(rename = "FileFormName")]
  pub file_form_name: String,

  #[serde(rename = "URL")]
  pub url: String,

  #[serde(rename = "ThumbnailURL")]
  pub thumbnail_url: Option<String>,

  #[serde(rename = "DeletionURL")]
  pub deletion_url: Option<String>,

  #[serde(rename = "ErrorMessage")]
  pub error_message: Option<String>,
}
pub type StringKV = HashMap<String, String>;

impl SXCU {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Res<Self> {
    let content = read_to_string(path)?;
    let parsed = serde_json::from_str(&content)?;

    Ok(parsed)
  }

  pub fn save_image(bytes: Vec<u8>) -> Res<PathBuf> {
    let config = Config::read()?;

    let now = Local::now();
    let formatted = now.format(&config.date_format);

    let saved = config.save_path.join(formatted.to_string());

    fs::write(&saved, bytes)?;

    Ok(saved)
  }

  pub fn create_req(&self, bytes: Vec<u8>) -> Res<(PathBuf, Request)> {
    let image_path = Self::save_image(bytes)?;

    let form = Form::new().file(self.file_form_name.to_owned(), &image_path)?;
    let client = Client::builder().build()?;

    let mut builder = match self.request_method {
      SXCURequestMethod::GET => client.get(self.request_url.to_owned()),
      SXCURequestMethod::POST => client.post(self.request_url.to_owned()),
      SXCURequestMethod::PUT => client.put(self.request_url.to_owned()),
      SXCURequestMethod::PATCH => client.patch(self.request_url.to_owned()),
      SXCURequestMethod::DELETE => client.delete(self.request_url.to_owned()),
    };

    if let Some(headers) = &self.headers {
      let mut h = HeaderMap::new();

      for (key, val) in headers {
        h.insert::<HeaderName>(key.parse()?, val.parse()?);
      }

      builder = builder.headers(h);
    }

    if let Some(queries) = &self.query {
      let mut values: Vec<(String, String)> = Vec::new();

      for (key, val) in queries {
        values.push((key.clone(), val.clone()));
      }

      builder = builder.query(&[values]);
    }

    let request = builder.multipart(form).build()?;

    Ok((image_path, request))
  }

  pub fn upload_screenshot(&self, bytes: Vec<u8>) -> Res<(PathBuf, String)> {
    let req = self.create_req(bytes)?;
    let res = Client::new().execute(req.1)?;
    let status = res.status();
    let raw = res.text()?;

    if !status.is_success() {
      panic!("Recieved {}\n\n{}", status, raw);
    } else {
      Ok((req.0, self.parse_url(&raw, true)?))
    }
  }

  pub fn parse_url(&self, json: &str, jq: bool) -> Res<String> {
    let regex = Regex::new(r"\$json:(?P<path>.[^\$]+)\$").unwrap();
    let result = regex.captures_iter(&self.url);

    let mut new_text = self.url.clone();

    for mat in result {
      let raw = mat.get(0).map_or("", |m| m.as_str());
      let path = &mat["path"];

      if jq {
        let parsed = Self::jq_path(path, json)?;
        new_text = new_text.replace(raw, &parsed);
      }
    }

    Ok(new_text)
  }

  pub fn jq_path(path: &str, json: &str) -> Res<String> {
    let echo = Command::new("echo")
      .arg(json)
      .stdout(Stdio::piped())
      .spawn()?;

    let jq = Command::new("jq")
      .stdin(echo.stdout.unwrap())
      .stdout(Stdio::piped())
      .stderr(Stdio::null())
      .arg("-rj")
      .arg(format!(".{}", path))
      .spawn()?
      .wait_with_output()?;

    Ok(
      String::from_utf8_lossy(&jq.stdout)
        .to_string()
        .replace("\n", ""),
    )
  }
}

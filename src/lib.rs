extern crate api;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use std::io;


type Timestamp = f64;
type Value = String;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
}


#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InvalidExpression(String),
    Timeout(String),
    InvalidResponse(serde_json::Error),
    Unexpected(u16),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixItem {
    pub metric: BTreeMap<String, String>,
    pub values: Vec<Scalar>,
}
pub type Matrix = Vec<MatrixItem>;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstantVecItem {
    pub metric: BTreeMap<String, String>,
    pub value: Scalar,
}
pub type InstantVec = Vec<InstantVecItem>;

pub type Scalar = (Timestamp, Value);

pub type Str = (Timestamp, String);


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType", content = "result" )]
#[serde(rename_all = "lowercase")]
pub enum Data {
    Matrix(Matrix),
    Vector(InstantVec),
    Scalar(Scalar),
    String(Str),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PromResponse {
    pub status: Status,
    pub data: Data,
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}


pub struct Query {
    pub query: String,
    pub time: Option<Timestamp>,
    pub timeout: Option<std::time::Duration>,
}


impl api::Api for Query {
    type Reply = Data;
    type Body = io::Empty;
    type Error = Error;

    fn method(&self) -> api::Method {
        api::Method::Get
    }

    fn path(&self) -> String {
        "/api/v1/query".to_string()
    }

    fn query(&self) -> api::Query {
        let mut query = api::Query::new();

        query.push(("query".to_string(), self.query.to_string()));
        if let Some(time) = self.time {
            query.push(("time".to_string(), time.to_string()))
        }
        if let Some(timeout) = self.timeout {
            let timeout = timeout.as_secs();
            let timeout = timeout.to_string();
            query.push(("timeout".to_string(), timeout));
        }

        query
    }

    fn headers(&self) -> api::Headers {
        api::Headers::new()
    }

    fn body(&self) -> io::Empty {
        io::empty()
    }

    fn parse<H>(&self, resp: &mut H) -> Result<Data, Error>
        where H: api::HttpResponse
    {

        let resp = serde_json::from_reader(resp.body())
            .map_err(|e| Error::InvalidResponse(e))
            .and_then(|r: PromResponse| {
                let error = r.error.unwrap_or("".to_string());

                match resp.status() {
                    200 => Ok(r.data),
                    400 => Err(Error::BadRequest(error)),
                    422 => Err(Error::InvalidExpression(error)),
                    503 => Err(Error::Timeout(error)),
                    s @ _ => Err(Error::Unexpected(s)),
                }
            });

        resp
    }
}

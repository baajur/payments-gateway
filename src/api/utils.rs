use std::fmt::Debug;

use failure::Fail;
use futures::future;
use futures::prelude::*;
use hyper::Body;
use hyper::Response;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

use super::error::*;
use super::ControllerFuture;

pub fn parse_body<T>(body: Vec<u8>) -> impl Future<Item = T, Error = Error> + Send
where
    T: for<'de> Deserialize<'de> + Send,
{
    String::from_utf8(body.clone())
        .map_err(ectx!(ErrorContext::RequestUTF8, ErrorKind::BadRequest => body))
        .into_future()
        .and_then(|string| serde_json::from_str::<T>(&string).map_err(ectx!(ErrorContext::RequestJson, ErrorKind::BadRequest => string)))
}

pub fn response_with_model<M>(model: &M) -> ControllerFuture
where
    M: Debug + Serialize,
{
    Box::new(
        serde_json::to_string(&model)
            .map_err(ectx!(ErrorContext::ResponseJson, ErrorKind::Internal => model))
            .into_future()
            .map(|text| {
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(text.into())
                    .unwrap()
            }),
    )
}

pub fn response_with_redirect(url: String) -> ControllerFuture {
    Box::new(future::ok(
        Response::builder()
            .status(308)
            .header("Location", &*url)
            .body(Body::empty())
            .unwrap(),
    ))
}

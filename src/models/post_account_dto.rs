use rocket::data::{Data, FromData, Outcome, ToByteUnit};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, Request};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostAccountDTO {
    pub account_balance: f32,
}

#[derive(Debug)]
pub enum PostRequestError {
    TooLarge,
    NoBodyProvided,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for PostAccountDTO {
    type Error = PostRequestError;
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let string_data = match data.open(10.kilobytes()).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => return Error((Status::PayloadTooLarge, PostRequestError::TooLarge)),
            Err(e) => return Error((Status::InternalServerError, PostRequestError::Io(e))),
        };

        let string_body = request::local_cache!(req, string_data);
        let object = serde_json::from_str(string_body);

        match object {
            Ok(object) => Success(object),
            Err(_) => Error((Status::BadRequest, PostRequestError::NoBodyProvided)),
        }
    }
}

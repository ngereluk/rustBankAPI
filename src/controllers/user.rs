use rocket::response::status;
use rocket::State;
use sea_orm::*;

use crate::entites::{prelude::*, *};
use crate::models::post_user_dto::PostUserDto;
use chrono::prelude::*;
use rocket::serde::json::Json;
use std::string::ToString;

#[get("/user")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Json<Vec<user::Model>> {
    let db = db as &DatabaseConnection;

    let users = User::find().all(db).await.unwrap();

    Json(users)
}

#[get("/user/<id>")]
pub async fn get_user_by_id(db: &State<DatabaseConnection>, id: i32) -> Json<Option<user::Model>> {
    let db = db as &DatabaseConnection;

    let user = User::find_by_id(id).one(db).await.unwrap();

    Json(user)
}

#[post("/user", format = "application/json", data = "<object>")]
pub async fn create_user(
    connection: &State<DatabaseConnection>,
    object: PostUserDto,
) -> status::Accepted<std::option::Option<&str>> {
    let connection = connection as &DatabaseConnection;
    let record = user::ActiveModel {
        first_name: Set(object.first_name.to_owned()),
        last_name: Set(object.last_name.to_owned()),
        address_line1: Set(object.address_line1.to_owned()),
        address_line2: Set(object.address_line2.to_owned()),
        postal_code: Set(object.postal_code.to_owned()),
        city: Set(object.city.to_owned()),
        province: Set(object.province.to_owned()),
        dob: Set(object.dob.to_owned()),
        created_at: Set(Local::now().to_string().to_owned()),
        updated_at: Set(Local::now().to_string().to_owned()),
        deleted: Set("false".to_owned()),

        ..Default::default()
    };
    let _result = User::insert(record.clone()).exec(connection).await;

    status::Accepted(Some("record created"))
}

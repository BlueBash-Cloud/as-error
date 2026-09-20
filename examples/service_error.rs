use as_error::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignUpError {
    #[error("normal avatar not create")]
    NormalAvatarNotCreate,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn serde_json_error() -> ResultExt<Point> {
    let point = Point { x: 1, y: 2 };
    let mut serialized =
        serde_json::to_string(&point).map_err(Into::<ServiceError>::into)?;
    serialized.push('}'); // make it invalid JSON
    let deserialized: Point =
        serde_json::from_str(&serialized).map_err(Into::<ServiceError>::into)?;
    Ok(deserialized)
}

fn main() {
    let e = SignUpError::NormalAvatarNotCreate;
    let quick_error = static_precondition_failed!(e, "This is a static precondition failed error");
    println!("Quick error: {}", quick_error);
    let e = SignUpError::NormalAvatarNotCreate;
    let info = String::from("This is a string error");
    let error = precondition_failed!(e, info);
    println!("Error: {}", error);
    let error = serde_json_error();
    println!("Error: {:?}", error);
}

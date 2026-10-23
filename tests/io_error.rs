#![cfg(feature = "file_error")]

use actix_web::{ResponseError, body::to_bytes, http::StatusCode};
use as_error::{AsError, FileError, ResultExt, const_define::*};
use std::{error::Error as _, io};

#[test]
fn io_errors_propagate_with_question_mark() {
    fn fail() -> ResultExt<()> {
        Err::<(), _>(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"))?;
        Ok(())
    }

    let AsError::IoError(error) = fail().unwrap_err() else {
        panic!("expected the original I/O error");
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(error.to_string(), "invalid input");
}

#[test]
fn io_error_source_preserves_os_error() {
    let error: AsError = io::Error::from_raw_os_error(13).into();
    let source = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(source.raw_os_error(), Some(13));
}

#[actix_web::test]
async fn io_error_responses_use_a_generic_server_error() {
    for kind in [
        io::ErrorKind::NotFound,
        io::ErrorKind::AlreadyExists,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::InvalidInput,
        io::ErrorKind::Other,
    ] {
        let error: AsError = io::Error::new(kind, "internal I/O details").into();
        assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), FILE_ERROR.as_bytes());
    }
}

#[actix_web::test]
async fn file_errors_keep_their_specific_response() {
    let error: AsError = FileError::FileNotFound.into();
    assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body.as_ref(), FILE_NOT_FOUND.as_bytes());
}

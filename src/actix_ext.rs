use crate::AsError;
use crate::const_define::*;
use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};

impl ResponseError for AsError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            #[cfg(feature = "service_error")]
            Self::ServiceError(o) => {
                use crate::error::service::ServiceError;
                match o {
                    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
                    // Consider adding header validation
                    ServiceError::PreconditionFailed(_, _)
                    | ServiceError::StaticPreconditionFailed(_, _) => {
                        StatusCode::PRECONDITION_FAILED
                    }

                    ServiceError::InvalidRequest(_, _)
                    | ServiceError::StaticInvalidRequest(_, _) => StatusCode::BAD_REQUEST,

                    ServiceError::MutexLockError(_, _)
                    | ServiceError::StaticMutexLockError(_, _)
                    | ServiceError::JsonError(_, _)
                    | ServiceError::StaticJsonError(_, _)
                    | ServiceError::InternalServiceError(_, _)
                    | ServiceError::StaticInternalServiceError(_, _) => {
                        StatusCode::INTERNAL_SERVER_ERROR
                    }

                    ServiceError::RetryError(_, _) | ServiceError::StaticRetryError(_, _) => {
                        StatusCode::SERVICE_UNAVAILABLE
                    }
                }
            }

            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "redis_error")]
            Self::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "file_error")]
            Self::FileError(o) => {
                use crate::error::file::FileError;
                match o {
                    FileError::FileCreateFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileDeleteFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileNotFound => StatusCode::NOT_FOUND,
                    FileError::FileAlreadyExists => StatusCode::CONFLICT,
                    FileError::FileFrozen => StatusCode::FORBIDDEN,
                    FileError::FileWriteFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    FileError::FileReadFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

            #[cfg(feature = "tokio_error")]
            Self::TokioError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "awc_error")]
            Self::AwcConnectError(_)
            | Self::AwcFreezeRequestError(_)
            | Self::AwcHttpError(_)
            | Self::AwcJsonPayloadError(_)
            | Self::AwcPayloadError(_)
            | Self::AwcWsClientError(_)
            | Self::AwcSendRequestError(_)
            | Self::AwcWsHandshakeError(_)
            | Self::AwcWsProtocolError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "reqwest_error")]
            Self::ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "chrono_error")]
            Self::ChronoParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "chrono_error")]
            Self::TimeZoneError | Self::TimestampError | Self::WeekError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            #[cfg(feature = "rust_decimal_error")]
            Self::DecimalError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "flatbuffer_error")]
            Self::InvalidFlatbufferError => StatusCode::BAD_REQUEST,

            #[cfg(feature = "http_response_error")]
            Self::HttpResponseNotOK(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "string_error")]
            Self::StringError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            #[cfg(feature = "serde_error")]
            Self::SerdeJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            #[cfg(feature = "service_error")]
            Self::ServiceError(o) => {
                use crate::error::service::ServiceError;
                match o {
                    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/412 Incomplete implementation
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Match
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/If-Unmodified-Since
                    // Consider adding header validation
                    ServiceError::PreconditionFailed(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(info.clone())
                    }
                    ServiceError::StaticPreconditionFailed(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(*info)
                    }
                    ServiceError::InvalidRequest(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::BAD_REQUEST).body(info.clone())
                    }
                    ServiceError::StaticInvalidRequest(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::BAD_REQUEST).body(*info)
                    }
                    ServiceError::MutexLockError(_err, _info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SERVICE_ERROR)
                    }
                    ServiceError::StaticMutexLockError(_err, _info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SERVICE_ERROR)
                    }
                    ServiceError::JsonError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(info.clone())
                    }
                    ServiceError::StaticJsonError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(*info)
                    }
                    ServiceError::RetryError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).body(info.clone())
                    }
                    ServiceError::StaticRetryError(_err, info) => {
                        HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).body(*info)
                    }
                    ServiceError::InternalServiceError(_err, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(info.clone())
                    }
                    ServiceError::StaticInternalServiceError(_, info) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(*info)
                    }
                }
            }
            #[cfg(feature = "sqlx_error")]
            Self::SQLXError(_) => {
                // TODO: write to log
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SQLX_ERROR)
            }
            #[cfg(feature = "redis_error")]
            Self::RedisError(_) => {
                // TODO: write to log
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(REDIS_ERROR)
            }
            #[cfg(feature = "file_error")]
            Self::FileError(o) => {
                use crate::error::file::FileError;
                match o {
                    FileError::FileCreateFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_CREATE_FAILED)
                    }
                    FileError::FileDeleteFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_DELETE_FAILED)
                    }
                    FileError::FileNotFound => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::NOT_FOUND).body(FILE_NOT_FOUND)
                    }
                    FileError::FileAlreadyExists => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::CONFLICT).body(FILE_ALREADY_EXISTS)
                    }
                    FileError::FileFrozen => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::FORBIDDEN).body(FILE_FROZEN)
                    }
                    FileError::FileWriteFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_WRITE_FAILED)
                    }
                    FileError::FileReadFailed(_) => {
                        // TODO: write to log
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(FILE_READ_FAILED)
                    }
                }
            }
            #[cfg(feature = "tokio_error")]
            Self::TokioError(o) => match o {
                crate::TokioError::TaskJoinError(_join_error) => {
                    // TODO: write to log
                    HttpResponse::build(StatusCode::PRECONDITION_FAILED).body(TOKIO_TASK_JOIN_ERROR)
                }
            },
            #[cfg(feature = "awc_error")]
            Self::AwcConnectError(_)
            | Self::AwcFreezeRequestError(_)
            | Self::AwcHttpError(_)
            | Self::AwcJsonPayloadError(_)
            | Self::AwcPayloadError(_)
            | Self::AwcWsClientError(_)
            | Self::AwcSendRequestError(_)
            | Self::AwcWsHandshakeError(_)
            | Self::AwcWsProtocolError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(AWC_ERROR)
            }
            #[cfg(feature = "reqwest_error")]
            Self::ReqwestError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(REQWEST_ERROR)
            }
            #[cfg(feature = "chrono_error")]
            Self::ChronoParseError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(CHRONO_PARSE_ERROR)
            }
            #[cfg(feature = "chrono_error")]
            Self::TimeZoneError => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(TIME_ZONE_ERROR)
            }
            #[cfg(feature = "chrono_error")]
            Self::WeekError => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(WEEK_ERROR)
            }
            #[cfg(feature = "chrono_error")]
            Self::TimestampError => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(TIMESTAMP_ERROR)
            }
            #[cfg(feature = "rust_decimal_error")]
            Self::DecimalError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(DECIMAL_ERROR)
            }
            #[cfg(feature = "flatbuffer_error")]
            Self::InvalidFlatbufferError => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(FLATBUFFER_ERROR)
            }
            #[cfg(feature = "http_response_error")]
            Self::HttpResponseNotOK(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(HTTP_RESPONSE_ERROR)
            }
            #[cfg(feature = "string_error")]
            Self::StringError(o) => match o {
                crate::StringError::FromUtf8Error(_) => {
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(STRING_FROM_UTF8_ERROR)
                }
            },
            #[cfg(feature = "serde_error")]
            Self::SerdeJsonError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(SERDE_JSON_ERROR)
            }
        }
    }
}

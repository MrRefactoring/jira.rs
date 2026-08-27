// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum BadRequestExceptionResponseStatus {
        Ok => "OK",
        Created => "CREATED",
        Accepted => "ACCEPTED",
        NoContent => "NO_CONTENT",
        ResetContent => "RESET_CONTENT",
        PartialContent => "PARTIAL_CONTENT",
        MovedPermanently => "MOVED_PERMANENTLY",
        Found => "FOUND",
        SeeOther => "SEE_OTHER",
        NotModified => "NOT_MODIFIED",
        UseProxy => "USE_PROXY",
        TemporaryRedirect => "TEMPORARY_REDIRECT",
        BadRequest => "BAD_REQUEST",
        Unauthorized => "UNAUTHORIZED",
        PaymentRequired => "PAYMENT_REQUIRED",
        Forbidden => "FORBIDDEN",
        NotFound => "NOT_FOUND",
        MethodNotAllowed => "METHOD_NOT_ALLOWED",
        NotAcceptable => "NOT_ACCEPTABLE",
        ProxyAuthenticationRequired => "PROXY_AUTHENTICATION_REQUIRED",
        RequestTimeout => "REQUEST_TIMEOUT",
        Conflict => "CONFLICT",
        Gone => "GONE",
        LengthRequired => "LENGTH_REQUIRED",
        PreconditionFailed => "PRECONDITION_FAILED",
        RequestEntityTooLarge => "REQUEST_ENTITY_TOO_LARGE",
        RequestUriTooLong => "REQUEST_URI_TOO_LONG",
        UnsupportedMediaType => "UNSUPPORTED_MEDIA_TYPE",
        RequestedRangeNotSatisfiable => "REQUESTED_RANGE_NOT_SATISFIABLE",
        ExpectationFailed => "EXPECTATION_FAILED",
        InternalServerError => "INTERNAL_SERVER_ERROR",
        NotImplemented => "NOT_IMPLEMENTED",
        BadGateway => "BAD_GATEWAY",
        ServiceUnavailable => "SERVICE_UNAVAILABLE",
        GatewayTimeout => "GATEWAY_TIMEOUT",
        HttpVersionNotSupported => "HTTP_VERSION_NOT_SUPPORTED",
    }
}

crate::open_enum! {
    pub enum BadRequestExceptionScimErrorType {
        InvalidFilter => "invalidFilter",
        TooMany => "tooMany",
        Uniqueness => "uniqueness",
        Mutability => "mutability",
        InvalidSyntax => "invalidSyntax",
        InvalidPath => "invalidPath",
        NoTarget => "noTarget",
        InvalidValue => "invalidValue",
        InvalidVers => "invalidVers",
        Sensitive => "sensitive",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadRequestException {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<Box<Throwable>>,
    #[serde(rename = "stackTrace", default, skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<Vec<StackTraceElement>>,
    #[serde(rename = "responseStatus", default, skip_serializing_if = "Option::is_none")]
    pub response_status: Option<BadRequestExceptionResponseStatus>,
    #[serde(rename = "scimErrorType", default, skip_serializing_if = "Option::is_none")]
    pub scim_error_type: Option<BadRequestExceptionScimErrorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "localizedMessage", default, skip_serializing_if = "Option::is_none")]
    pub localized_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<Vec<Box<Throwable>>>,
}

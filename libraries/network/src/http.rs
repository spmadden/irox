// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;

pub use client::*;
pub use h2::*;
pub use headers::*;
pub use request::*;
pub use response::*;

mod client;
mod h2;
mod headers;
mod request;
mod response;

///
/// Basic enumerated type to pick the HTTP protocol & port
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum HttpProtocol {
    HTTP,

    #[default]
    HTTPS,
}

impl HttpProtocol {
    #[must_use]
    pub const fn port(&self) -> u16 {
        match self {
            HttpProtocol::HTTP => 80,
            HttpProtocol::HTTPS => 443,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            HttpProtocol::HTTP => "http",
            HttpProtocol::HTTPS => "https",
        }
    }
}

/// Methods to be used in HTTP requests
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HttpMethod {
    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    Connect,
    /// The `DELETE` method deletes the specified resource
    Delete,
    /// The `GET` method requests a representation of the specified resource
    Get,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the
    /// response body
    Head,
    /// The `OPTIONS` method describes the communication options for the target resource
    Options,
    /// The `PATCH` method applies partial modifications to a resource
    Patch,
    /// The `POST` method submits an entity to the specified resource, often causing a change in
    /// state or side effects on the server
    Post,
    /// The `PUT` method replaces all current representations of the target resource with the
    /// request payload
    Put,
    /// The `TRACE` method performs a message loop-back test along the path to the target resource
    Trace,

    /// Other/Unknown method
    Other(String),
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpMethod::Connect => "CONNECT",
                HttpMethod::Delete => "DELETE",
                HttpMethod::Get => "GET",
                HttpMethod::Head => "HEAD",
                HttpMethod::Options => "OPTIONS",
                HttpMethod::Patch => "PATCH",
                HttpMethod::Post => "POST",
                HttpMethod::Put => "PUT",
                HttpMethod::Trace => "TRACE",
                HttpMethod::Other(o) => o,
            }
        )
    }
}

/// The general grouping of HTTP Status Codes
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HttpCodeType {
    /// 1xx series, Informational
    Info,
    /// 2xx series, Successful
    Success,
    /// 3xx series, Redirects
    Redirect,
    /// 4xx series, Client Errors
    ClientError,
    /// 5xx series, Server Errors
    ServerError,
    /// Others that aren't above
    UnknownOther,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum HttpCodes {
    /// This interim response indicates that the client should continue the request or ignore the
    /// response if the request is already finished.
    Info_100_Continue,
    /// This code is sent in response to an Upgrade request header from the client and indicates the
    /// protocol the server is switching to.
    Info_101_SwitchingProtocols,
    /// This code indicates that the server has received and is processing the request, but no
    /// response is available yet.
    Info_102_Processing,
    /// This status code is primarily intended to be used with the Link header, letting the user
    /// agent start preloading resources while the server prepares a response or preconnect to an
    /// origin from which the page will need resources.
    Info_103_EarlyHints,

    /// The request succeeded
    Success_200_Ok,
    /// The request succeeded, and a new resource was created as a result.
    Success_201_Created,
    /// The request has been received but not yet acted upon
    Success_202_Accepted,
    /// This response code means the returned metadata is not exactly the same as is available from
    /// the origin server, but is collected from a local or a third-party copy.
    Success_203_NotAuthoritative,
    /// There is no content to send for this request, but the headers may be useful
    Success_204_NoContent,
    /// Tells the user agent to reset the document which sent this request.
    Success_205_ResetContent,
    /// This response code is used when the Range header is sent from the client to request only
    /// part of a resource.
    Success_206_PartialContent,
    /// Conveys information about multiple resources, for situations where multiple status codes
    /// might be appropriate.
    Success_207_MultiStatus,
    /// Used inside a `<dav:propstat>` response element to avoid repeatedly enumerating the internal
    /// members of multiple bindings to the same collection.
    Success_208_AlreadyReported,
    /// The server has fulfilled a GET request for the resource, and the response is a
    /// representation of the result of one or more instance-manipulations applied to the current
    /// instance.
    Success_226_IMUsed,

    /// The request has more than one possible response
    Redirect_300_MultipleChoices,
    /// The URL of the requested resource has been changed permanently
    Redirect_301_MovedPermanently,
    /// This response code means that the URI of requested resource has been changed temporarily
    Redirect_302_Found,
    /// The server sent this response to direct the client to get the requested resource at another
    /// URI with a GET request.
    Redirect_303_SeeOther,
    /// This is used for caching purposes. It tells the client that the response has not been
    /// modified
    Redirect_304_NotModified,
    /// Defined in a previous version of the HTTP specification to indicate that a requested
    /// response must be accessed by a proxy
    Redirect_305_UseProxy,
    /// This response code is no longer used; it is just reserved
    Redirect_306_Unused,
    /// The server sends this response to direct the client to get the requested resource at another
    /// URI with the same method that was used in the prior request
    Redirect_307_Temporary,
    /// This means that the resource is now permanently located at another URI, specified by the
    /// Location: HTTP Response header.
    Redirect_308_PermanentRedirect,

    /// The server cannot or will not process the request due to something that is perceived to be
    /// a client error
    ClientError_400_BadRequest,
    /// Although the HTTP standard specifies "unauthorized", semantically this response means
    /// "unauthenticated".
    ClientError_401_Unauthorized,
    /// This response code is reserved for future use. The initial aim for creating this code was
    /// using it for digital payment systems, however this status code is used very rarely and no
    /// standard convention exists.
    ClientError_402_PaymentRequired,
    /// The client does not have access rights to the content
    ClientError_403_Forbidden,
    /// The server cannot find the requested resource
    ClientError_404_NotFound,
    /// The request method is known by the server but is not supported by the target resource
    ClientError_405_MethodNotAllowed,
    /// This response is sent when the web server, after performing server-driven content
    /// negotiation, doesn't find any content that conforms to the criteria given by the user agent
    ClientError_406_NotAcceptable,
    /// This is similar to 401 Unauthorized but authentication is needed to be done by a proxy.
    ClientError_407_ProxyAuthenticationRequired,
    /// This response is sent on an idle connection by some servers, even without any previous
    /// request by the client. It means that the server would like to shut down this unused
    /// connection.
    ClientError_408_RequestTimeout,
    /// This response is sent when a request conflicts with the current state of the server.
    ClientError_409_Conflict,
    /// This response is sent when the requested content has been permanently deleted from server,
    /// with no forwarding address
    ClientError_410_Gone,
    /// Server rejected the request because the Content-Length header field is not defined and the
    /// server requires it.
    ClientError_411_LengthRequired,
    /// The client has indicated preconditions in its headers which the server does not meet.
    ClientError_412_PreconditionFailed,
    /// Request entity is larger than limits defined by server
    ClientError_413_PayloadTooLarge,
    /// The URI requested by the client is longer than the server is willing to interpret.
    ClientError_414_URITooLong,
    /// The media format of the requested data is not supported by the server, so the server is
    /// rejecting the request.
    ClientError_415_UnsupportedMediaType,
    /// The range specified by the Range header field in the request cannot be fulfilled
    ClientError_416_RangeNotSatisfiable,
    /// This response code means the expectation indicated by the Expect request header field cannot
    /// be met by the server.
    ClientError_417_ExpectationFailed,
    /// The server refuses the attempt to brew coffee with a teapot.
    ClientError_418_ImATeapot,
    /// The request was directed at a server that is not able to produce a response
    ClientError_421_MisdirectedRequest,
    /// The request was well-formed but was unable to be followed due to semantic errors.
    ClientError_422_UnprocessableContent,
    /// The resource that is being accessed is locked.
    ClientError_423_Locked,
    /// The request failed due to failure of a previous request.
    ClientError_424_FailedDependency,
    /// Indicates that the server is unwilling to risk processing a request that might be replayed.
    ClientError_425_TooEarly,
    /// The server refuses to perform the request using the current protocol but might be willing to
    /// do so after the client upgrades to a different protocol
    ClientError_426_UpgradeRequired,
    /// The origin server requires the request to be conditional. This response is intended to
    /// prevent the 'lost update' problem, where a client GETs a resource's state, modifies it and
    /// PUTs it back to the server, when meanwhile a third party has modified the state on the
    /// server, leading to a conflict.
    ClientError_428_PreconditionRequired,
    /// The user has sent too many requests in a given amount of time ("rate limiting").
    ClientError_429_TooManyRequests,
    /// The server is unwilling to process the request because its header fields are too large
    ClientError_431_RequestHeaderFieldsTooLarge,
    /// The user agent requested a resource that cannot legally be provided, such as a web page
    /// censored by a government.
    ClientError_451_UnavailableForLegalReasons,

    /// The server has encountered a situation it does not know how to handle
    ServerError_500_InternalServerError,
    /// The request method is not supported by the server and cannot be handled. The only methods
    /// that servers are required to support (and therefore that must not return this code) are
    /// `GET` and `HEAD`
    ServerError_501_NotImplemented,
    /// This error response means that the server, while working as a gateway to get a response
    /// needed to handle the request, got an invalid response.
    ServerError_502_BadGateway,
    /// The server is not ready to handle the request. Common causes are a server that is down for
    /// maintenance or that is overloaded
    ServerError_503_ServiceUnavailable,
    /// This error response is given when the server is acting as a gateway and cannot get a
    /// response in time
    ServerError_504_GatewayTimeout,
    /// The HTTP version used in the request is not supported by the server.
    ServerError_505_HTTPVersionNotSupported,
    /// The server has an internal configuration error: the chosen variant resource is configured to
    /// engage in transparent content negotiation itself, and is therefore not a proper end point in
    /// the negotiation process
    ServerError_506_VariantAlsoNegotiates,
    /// The method could not be performed on the resource because the server is unable to store the
    /// representation needed to successfully complete the request
    ServerError_507_InsufficientStorage,
    /// The server detected an infinite loop while processing the request
    ServerError_508_LoopDetected,
    /// Further extensions to the request are required for the server to fulfill it
    ServerError_510_NotExtended,
    /// Indicates that the client needs to authenticate to gain network access
    ServerError_511_NetworkAuthenticationRequired,

    /// Unknown/other code
    UnknownOther(u16),
}

impl HttpCodes {
    /// Returns the numeric HTTP status code
    #[must_use]
    pub fn code(&self) -> u16 {
        match self {
            HttpCodes::Info_100_Continue => 100,
            HttpCodes::Info_101_SwitchingProtocols => 101,
            HttpCodes::Info_102_Processing => 102,
            HttpCodes::Info_103_EarlyHints => 103,

            HttpCodes::Success_200_Ok => 200,
            HttpCodes::Success_201_Created => 201,
            HttpCodes::Success_202_Accepted => 202,
            HttpCodes::Success_203_NotAuthoritative => 203,
            HttpCodes::Success_204_NoContent => 204,
            HttpCodes::Success_205_ResetContent => 205,
            HttpCodes::Success_206_PartialContent => 206,
            HttpCodes::Success_207_MultiStatus => 207,
            HttpCodes::Success_208_AlreadyReported => 208,
            HttpCodes::Success_226_IMUsed => 226,

            HttpCodes::Redirect_300_MultipleChoices => 300,
            HttpCodes::Redirect_301_MovedPermanently => 301,
            HttpCodes::Redirect_302_Found => 302,
            HttpCodes::Redirect_303_SeeOther => 303,
            HttpCodes::Redirect_304_NotModified => 304,
            HttpCodes::Redirect_305_UseProxy => 305,
            HttpCodes::Redirect_306_Unused => 306,
            HttpCodes::Redirect_307_Temporary => 307,
            HttpCodes::Redirect_308_PermanentRedirect => 308,

            HttpCodes::ClientError_400_BadRequest => 400,
            HttpCodes::ClientError_401_Unauthorized => 401,
            HttpCodes::ClientError_402_PaymentRequired => 402,
            HttpCodes::ClientError_403_Forbidden => 403,
            HttpCodes::ClientError_404_NotFound => 404,
            HttpCodes::ClientError_405_MethodNotAllowed => 405,
            HttpCodes::ClientError_406_NotAcceptable => 406,
            HttpCodes::ClientError_407_ProxyAuthenticationRequired => 407,
            HttpCodes::ClientError_408_RequestTimeout => 408,
            HttpCodes::ClientError_409_Conflict => 409,
            HttpCodes::ClientError_410_Gone => 410,
            HttpCodes::ClientError_411_LengthRequired => 411,
            HttpCodes::ClientError_412_PreconditionFailed => 412,
            HttpCodes::ClientError_413_PayloadTooLarge => 413,
            HttpCodes::ClientError_414_URITooLong => 414,
            HttpCodes::ClientError_415_UnsupportedMediaType => 415,
            HttpCodes::ClientError_416_RangeNotSatisfiable => 416,
            HttpCodes::ClientError_417_ExpectationFailed => 417,
            HttpCodes::ClientError_418_ImATeapot => 418,
            HttpCodes::ClientError_421_MisdirectedRequest => 421,
            HttpCodes::ClientError_422_UnprocessableContent => 422,
            HttpCodes::ClientError_423_Locked => 423,
            HttpCodes::ClientError_424_FailedDependency => 424,
            HttpCodes::ClientError_425_TooEarly => 425,
            HttpCodes::ClientError_426_UpgradeRequired => 426,
            HttpCodes::ClientError_428_PreconditionRequired => 427,
            HttpCodes::ClientError_429_TooManyRequests => 429,
            HttpCodes::ClientError_431_RequestHeaderFieldsTooLarge => 431,
            HttpCodes::ClientError_451_UnavailableForLegalReasons => 451,

            HttpCodes::ServerError_500_InternalServerError => 500,
            HttpCodes::ServerError_501_NotImplemented => 501,
            HttpCodes::ServerError_502_BadGateway => 502,
            HttpCodes::ServerError_503_ServiceUnavailable => 503,
            HttpCodes::ServerError_504_GatewayTimeout => 504,
            HttpCodes::ServerError_505_HTTPVersionNotSupported => 505,
            HttpCodes::ServerError_506_VariantAlsoNegotiates => 506,
            HttpCodes::ServerError_507_InsufficientStorage => 507,
            HttpCodes::ServerError_508_LoopDetected => 508,
            HttpCodes::ServerError_510_NotExtended => 510,
            HttpCodes::ServerError_511_NetworkAuthenticationRequired => 511,

            HttpCodes::UnknownOther(v) => *v,
        }
    }

    /// Returns the HTTP Code Classification/Type
    pub fn code_type(&self) -> HttpCodeType {
        match self.code() {
            100..=199 => HttpCodeType::Info,
            200..=299 => HttpCodeType::Success,
            300..=399 => HttpCodeType::Redirect,
            400..=499 => HttpCodeType::ClientError,
            500..=599 => HttpCodeType::ServerError,
            _v => HttpCodeType::UnknownOther,
        }
    }
}

impl TryFrom<&str> for HttpCodes {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = u16::from_str(value)?;
        Ok(match val {
            e => HttpCodes::UnknownOther(e),
        })
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum HttpVersion {
    Http1_0,

    #[default]
    Http1_1,
    Http2,
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpVersion::Http1_0 => "HTTP/1.0",
                HttpVersion::Http1_1 => "HTTP/1.1",
                HttpVersion::Http2 => "h2",
                HttpVersion::Http3 => "h3",
            }
        )
    }
}

pub enum HttpBody {
    Empty,
    Read(Box<dyn Read>),
    String(String),
    Bytes(Vec<u8>),
}

impl HttpBody {
    pub fn write_to<T: Write>(self, out: &mut T) -> Result<(), std::io::Error> {
        match self {
            HttpBody::Empty => {}
            HttpBody::Read(mut read) => {
                std::io::copy(read.as_mut(), out)?;
            }
            HttpBody::String(s) => {
                out.write_all(s.as_bytes())?;
            }
            HttpBody::Bytes(b) => {
                out.write_all(&b)?;
            }
        }
        Ok(())
    }
}

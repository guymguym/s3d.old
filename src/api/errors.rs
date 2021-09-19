use crate::util::*;

#[derive(Debug, Clone)]
pub enum S3Error {
    BadRequest,
    BucketAlreadyExists,
    NoSuchBucket,
    NoSuchKey,
    _InternalError,
}

#[derive(Debug, Clone)]
pub struct S3ErrorInfo {
    pub status_code: hyper::StatusCode,
    pub code: String,
    // pub msg: String,
    // pub resource: String,
    // pub request_id: String,
}

impl S3Error {
    fn info(&self) -> S3ErrorInfo {
        match self {
            Self::BadRequest => S3ErrorInfo {
                status_code: hyper::StatusCode::BAD_REQUEST,
                code: "BadRequest".to_string(),
            },
            Self::BucketAlreadyExists => S3ErrorInfo {
                status_code: hyper::StatusCode::CONFLICT,
                code: "BucketAlreadyExists".to_string(),
            },
            Self::NoSuchBucket => S3ErrorInfo {
                status_code: hyper::StatusCode::NOT_FOUND,
                code: "NoSuchBucket".to_string(),
            },
            Self::NoSuchKey => S3ErrorInfo {
                status_code: hyper::StatusCode::NOT_FOUND,
                code: "NoSuchKey".to_string(),
            },
            _ => S3ErrorInfo {
                status_code: hyper::StatusCode::INTERNAL_SERVER_ERROR,
                code: "InternalError".to_string(),
            },
        }
    }
}

impl ResWriter for S3Error {
    fn write(self) -> HttpResponse {
        let mut w = BodyWriter::new_xml();
        let info = self.info();
        w.append("<Error>");
        w.append_xml("Code", info.code.as_str());
        // w.append_xml("Message", info.msg.as_str());
        // w.append_xml("Resource", info.resource.as_str());
        // w.append_xml("RequestId", info.request_id.as_str());
        w.append("</Error>");

        let mut r = HttpResponse::new(w.body());
        *r.status_mut() = info.status_code;
        r
    }
}

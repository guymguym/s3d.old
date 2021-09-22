use crate::api::*;
use hyper::{Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    pub bucket: String,
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub info: BucketInfo,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// DELETE / HTTP/1.1
    /// Host: Bucket.s3.amazonaws.com
    /// x-amz-expected-bucket-owner: ExpectedBucketOwner
    /// ```
    fn parse(req: HttpRequest, bucket: &str, _key: &str) -> Self {
        let (parts, _) = req.into_parts();
        let params = Params {
            bucket: bucket.to_string(),
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 204
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, _) = self.into_parts();
        let mut res = Response::from_parts(parts, Body::empty());
        *res.status_mut() = hyper::StatusCode::NO_CONTENT; // 204
        res
    }
}

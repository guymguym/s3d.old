use crate::api::*;
use hyper::{Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    pub bucket: String,
    pub class: String,
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub info: BucketInfo,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// PUT / HTTP/1.1
    /// Host: Bucket.s3.amazonaws.com
    /// x-amz-acl: ACL
    /// x-amz-grant-full-control: GrantFullControl
    /// x-amz-grant-read: GrantRead
    /// x-amz-grant-read-acp: GrantReadACP
    /// x-amz-grant-write: GrantWrite
    /// x-amz-grant-write-acp: GrantWriteACP
    /// x-amz-bucket-object-lock-enabled: ObjectLockEnabledForBucket
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <CreateBucketConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    ///    <LocationConstraint>string</LocationConstraint>
    /// </CreateBucketConfiguration>
    /// ```    
    fn parse(req: HttpRequest, bucket: &str, _key: &str) -> Self {
        let (parts, _) = req.into_parts();
        let qs = QueryStr::from_parts(&parts);
        let params = Params {
            bucket: bucket.to_string(),
            class: qs.get("bucket-class"),
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 200
    /// Location: Location
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let mut res = Response::from_parts(parts, Body::empty());
        res.headers_mut()
            .insert("Location", format!("/{}", r.info.name).parse().unwrap());
        res
    }
}

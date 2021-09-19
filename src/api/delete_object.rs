use crate::api::*;
use crate::util::*;
use hyper::{http::request::Parts, Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    pub bucket: String,
    pub key: String,
    pub version_id: String,
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub object: ObjectInfo,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// DELETE /Key+?versionId=VersionId HTTP/1.1
    /// Host: Bucket.s3.amazonaws.com
    /// x-amz-mfa: MFA
    /// x-amz-request-payer: RequestPayer
    /// x-amz-bypass-governance-retention: BypassGovernanceRetention
    /// x-amz-expected-bucket-owner: ExpectedBucketOwner
    /// ```
    fn parse(parts: Parts, _body: Body, bucket: &str, key: &str) -> Self {
        let qs = QueryStr::from_parts(&parts);
        let params = Params {
            bucket: bucket.to_string(),
            key: key.to_string(),
            version_id: qs.get("versionId"),
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 204
    /// x-amz-delete-marker: DeleteMarker
    /// x-amz-version-id: VersionId
    /// x-amz-request-charged: RequestCharged
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let mut res = Response::from_parts(parts, Body::empty());
        *res.status_mut() = hyper::StatusCode::NO_CONTENT; // 204
        res.headers_mut()
            .insert("x-amz-version-id", r.object.version_id.parse().unwrap());
        res
    }
}

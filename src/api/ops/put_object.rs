use crate::api::*;
use hyper::{Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug)]
pub struct Params {
    pub bucket: String,
    pub key: String,
    pub body: Option<Body>,
    // TODO partial updates
    // pub head_only: bool, // put only headers but keep content
    // pub range: ObjectRange, // pub only the selected range of the object

    // TODO: conditional uploads (e.g. if-match, if-none-match) ?
    // pub if_modified_since: Option<Time>,
    // pub if_unmodified_since: Option<Time>,
    // pub if_match: Option<String>,
    // pub if_none_match: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub object: ObjectInfo,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    // PUT /Key+ HTTP/1.1
    // Host: Bucket.s3.amazonaws.com
    // Cache-Control: CacheControl
    // Content-Disposition: ContentDisposition
    // Content-Encoding: ContentEncoding
    // Content-Language: ContentLanguage
    // Content-Length: ContentLength
    // Content-MD5: ContentMD5
    // Content-Type: ContentType
    // Expires: Expires
    // x-amz-acl: ACL
    // x-amz-grant-full-control: GrantFullControl
    // x-amz-grant-read: GrantRead
    // x-amz-grant-read-acp: GrantReadACP
    // x-amz-grant-write-acp: GrantWriteACP
    // x-amz-tagging: Tagging
    // x-amz-storage-class: StorageClass
    // x-amz-website-redirect-location: WebsiteRedirectLocation
    // x-amz-expected-bucket-owner: ExpectedBucketOwner
    // x-amz-object-lock-mode: ObjectLockMode
    // x-amz-object-lock-retain-until-date: ObjectLockRetainUntilDate
    // x-amz-object-lock-legal-hold: ObjectLockLegalHoldStatus
    // x-amz-request-payer: RequestPayer
    // x-amz-server-side-encryption: ServerSideEncryption
    // x-amz-server-side-encryption-customer-algorithm: SSECustomerAlgorithm
    // x-amz-server-side-encryption-customer-key: SSECustomerKey
    // x-amz-server-side-encryption-customer-key-MD5: SSECustomerKeyMD5
    // x-amz-server-side-encryption-aws-kms-key-id: SSEKMSKeyId
    // x-amz-server-side-encryption-context: SSEKMSEncryptionContext
    // x-amz-server-side-encryption-bucket-key-enabled: BucketKeyEnabled
    //
    // Body
    /// ```
    fn parse(req: HttpRequest, bucket: &str, key: &str) -> Self {
        let (parts, body) = req.into_parts();
        let params = Params {
            bucket: bucket.to_string(),
            key: key.to_string(),
            body: Some(body),
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    // HTTP/1.1 200
    // ETag: ETag
    // x-amz-version-id: VersionId
    // x-amz-expiration: Expiration
    // x-amz-request-charged: RequestCharged
    // x-amz-server-side-encryption: ServerSideEncryption
    // x-amz-server-side-encryption-customer-algorithm: SSECustomerAlgorithm
    // x-amz-server-side-encryption-customer-key-MD5: SSECustomerKeyMD5
    // x-amz-server-side-encryption-aws-kms-key-id: SSEKMSKeyId
    // x-amz-server-side-encryption-context: SSEKMSEncryptionContext
    // x-amz-server-side-encryption-bucket-key-enabled: BucketKeyEnabled
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let mut res = Response::from_parts(parts, Body::empty());
        res.headers_mut()
            .insert("ETag", r.object.etag.parse().unwrap());
        res.headers_mut()
            .insert("x-amz-version-id", r.object.version_id.parse().unwrap());
        res
    }
}

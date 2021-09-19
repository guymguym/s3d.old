use crate::api::*;
use crate::util::*;
use hyper::http::request::Parts;
use hyper::{Body, Method, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    pub bucket: String,
    pub key: String,
    pub version_id: String,

    // partial reads
    pub head_only: bool, // = HTTP HEAD method - no body should be returned
    pub range: ObjectRange,
    // TODO: conditional reads (e.g. if-match, if-none-match) ?
    // pub if_modified_since: Option<Time>,
    // pub if_unmodified_since: Option<Time>,
    // pub if_match: Option<String>,
    // pub if_none_match: Option<String>,
}

#[derive(Debug)]
pub struct Reply {
    pub object: ObjectInfo,
    pub body: Option<Body>,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// GET /Key+
    ///          ?partNumber=PartNumber
    ///          &response-cache-control=ResponseCacheControl
    ///          &response-content-disposition=ResponseContentDisposition
    ///          &response-content-encoding=ResponseContentEncoding
    ///          &response-content-language=ResponseContentLanguage
    ///          &response-content-type=ResponseContentType
    ///          &response-expires=ResponseExpires
    ///          &versionId=VersionId HTTP/1.1
    /// Host: Bucket.s3.amazonaws.com
    /// If-Match: IfMatch
    /// If-Modified-Since: IfModifiedSince
    /// If-None-Match: IfNoneMatch
    /// If-Unmodified-Since: IfUnmodifiedSince
    /// Range: Range
    /// x-amz-server-side-encryption-customer-algorithm: SSECustomerAlgorithm
    /// x-amz-server-side-encryption-customer-key: SSECustomerKey
    /// x-amz-server-side-encryption-customer-key-MD5: SSECustomerKeyMD5
    /// x-amz-request-payer: RequestPayer
    /// x-amz-expected-bucket-owner: ExpectedBucketOwner
    /// ```
    fn parse(parts: Parts, _body: Body, bucket: &str, key: &str) -> Self {
        let qs = QueryStr::from_parts(&parts);
        let head_only = parts.method == Method::HEAD;
        // let range = ObjectRange::parse(qs.get("range"));
        let range = ObjectRange {
            start: Some(0),
            end: Some(0),
        };
        let params = Params {
            bucket: bucket.to_string(),
            key: key.to_string(),
            version_id: qs.get("versionId"),
            head_only,
            range,
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 200
    /// Last-Modified: LastModified
    /// Content-Length: ContentLength
    /// ETag: ETag
    /// Cache-Control: CacheControl
    /// Content-Disposition: ContentDisposition
    /// Content-Encoding: ContentEncoding
    /// Content-Language: ContentLanguage
    /// Content-Range: ContentRange
    /// Content-Type: ContentType
    /// Expires: Expires
    /// accept-ranges: AcceptRanges
    /// x-amz-version-id: VersionId
    /// x-amz-delete-marker: DeleteMarker
    /// x-amz-restore: Restore
    /// x-amz-expiration: Expiration
    /// x-amz-missing-meta: MissingMeta
    /// x-amz-storage-class: StorageClass
    /// x-amz-request-charged: RequestCharged
    /// x-amz-replication-status: ReplicationStatus
    /// x-amz-mp-parts-count: PartsCount
    /// x-amz-tagging-count: TagCount
    /// x-amz-website-redirect-location: WebsiteRedirectLocation
    /// x-amz-object-lock-mode: ObjectLockMode
    /// x-amz-object-lock-retain-until-date: ObjectLockRetainUntilDate
    /// x-amz-object-lock-legal-hold: ObjectLockLegalHoldStatus
    /// x-amz-server-side-encryption: ServerSideEncryption
    /// x-amz-server-side-encryption-customer-algorithm: SSECustomerAlgorithm
    /// x-amz-server-side-encryption-customer-key-MD5: SSECustomerKeyMD5
    /// x-amz-server-side-encryption-aws-kms-key-id: SSEKMSKeyId
    /// x-amz-server-side-encryption-bucket-key-enabled: BucketKeyEnabled
    ///
    /// Body
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let body = r.body.unwrap_or(Body::empty());
        let mut res = Response::from_parts(parts, body);

        res.headers_mut()
            .insert("Last-Modified", r.object.last_modified.parse().unwrap());
        res.headers_mut()
            .insert("Content-Length", r.object.size.to_string().parse().unwrap());
        res.headers_mut()
            .insert("ETag", r.object.etag.parse().unwrap());

        res
    }
}

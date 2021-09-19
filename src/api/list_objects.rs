use crate::api::*;
use crate::util::*;
use hyper::{http::request::Parts, Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    pub bucket: String,
    pub prefix: String,
    pub delimiter: String,
    pub marker: String,
    pub max_keys: i32,
    pub encoding_type: String,
    // TODO: list objects v2
    // pub start_after: String,
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub bucket: String,
    pub prefix: String,
    pub delimiter: String,
    pub marker: String,
    pub max_keys: i32,
    pub encoding_type: String,

    // TODO: list objects v2
    // pub start_after: String,
    pub is_truncated: bool,
    pub next_marker: String,

    pub objects: Vec<ObjectInfo>,
    pub common_prefixes: Vec<String>,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// GET /
    ///     ?delimiter=Delimiter
    ///     &encoding-type=EncodingType
    ///     &marker=Marker
    ///     &max-keys=MaxKeys
    ///     &prefix=Prefix
    ///     HTTP/1.1
    /// Host: Bucket.s3.amazonaws.com
    /// x-amz-request-payer: RequestPayer
    /// x-amz-expected-bucket-owner: ExpectedBucketOwner
    /// ```
    fn parse(parts: Parts, _body: Body, bucket: &str, _key: &str) -> Self {
        let qs = QueryStr::from_parts(&parts);
        let params = Params {
            bucket: bucket.to_string(),
            prefix: qs.get("prefix"),
            delimiter: qs.get("delimiter"),
            marker: qs.get("marker"),
            max_keys: qs.get_i32("max-keys"),
            encoding_type: qs.get("encoding-type"),
            // start_after: qs.get("start-after"),
        };
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 200
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <ListBucketResult>
    ///    <IsTruncated>boolean</IsTruncated>
    ///    <Marker>string</Marker>
    ///    <NextMarker>string</NextMarker>
    ///    <Contents>
    ///       <ETag>string</ETag>
    ///       <Key>string</Key>
    ///       <LastModified>timestamp</LastModified>
    ///       <Owner>
    ///          <DisplayName>string</DisplayName>
    ///          <ID>string</ID>
    ///       </Owner>
    ///       <Size>integer</Size>
    ///       <StorageClass>string</StorageClass>
    ///    </Contents>
    ///    ...
    ///    <Name>string</Name>
    ///    <Prefix>string</Prefix>
    ///    <Delimiter>string</Delimiter>
    ///    <MaxKeys>integer</MaxKeys>
    ///    <CommonPrefixes>
    ///       <Prefix>string</Prefix>
    ///    </CommonPrefixes>
    ///    ...
    ///    <EncodingType>string</EncodingType>
    /// </ListBucketResult>    
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let mut w = BodyWriter::new_xml();

        w.append("<ListBucketResult>");

        w.append_xml("IsTruncated", r.is_truncated.to_string().as_str());
        w.append_xml("NextMarker", r.next_marker.as_str());

        w.append_xml("Name", r.bucket.as_str());
        w.append_xml("Prefix", r.prefix.as_str());
        w.append_xml("Marker", r.marker.as_str());
        w.append_xml("Delimiter", r.delimiter.as_str());
        w.append_xml("MaxKeys", r.max_keys.to_string().as_str());
        w.append_xml("EncodingType", r.encoding_type.as_str());

        for obj in r.objects {
            w.append("<Contents>");
            w.append_xml("Key", obj.key.to_string().as_str());
            w.append_xml("LastModified", obj.last_modified.to_string().as_str());
            w.append_xml("ETag", obj.etag.to_string().as_str());
            w.append_xml("Size", obj.size.to_string().as_str());
            w.append_xml("StorageClass", obj.storage_class.to_string().as_str());

            w.append("<Owner>");
            w.append_xml("ID", obj.owner.id.as_str());
            w.append_xml("DisplayName", obj.owner.display_name.as_str());
            w.append("</Owner>");

            w.append("</Contents>");
        }

        for prefix in r.common_prefixes {
            w.append("<CommonPrefixes>");
            w.append_xml("Prefix", prefix.to_string().as_str());
            w.append("</CommonPrefixes>");
        }

        w.append("</ListBucketResult>");
        Response::from_parts(parts, w.body())
    }
}

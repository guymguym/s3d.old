use crate::api::*;
use crate::util::*;
use hyper::{http::request::Parts, Body, Request, Response};

pub type Req = Request<Params>;
pub type Res = Response<Reply>;
pub type Ret = Result<Res, S3Error>;

#[derive(Debug, Clone)]
pub struct Params {
    // empty for now
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub buckets: Vec<BucketInfo>,
    pub is_truncated: bool,
    pub next_marker: String,
    pub owner: UserInfo,
}

impl ReqParser for Req {
    /// Request Syntax:
    /// ```
    /// GET / HTTP/1.1
    /// ```
    fn parse(parts: Parts, _body: Body, _bucket: &str, _key: &str) -> Self {
        // let qs = QueryStr::from_parts(&parts);
        let params = Params {};
        Request::from_parts(parts, params)
    }
}

impl ResWriter for Res {
    /// Response Syntax:
    /// ```
    /// HTTP/1.1 200
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <ListAllMyBucketsResult>
    ///    <Buckets>
    ///       <Bucket>
    ///          <CreationDate>timestamp</CreationDate>
    ///          <Name>string</Name>
    ///       </Bucket>
    ///    </Buckets>
    ///    <Owner>
    ///       <DisplayName>string</DisplayName>
    ///       <ID>string</ID>
    ///    </Owner>
    /// </ListAllMyBucketsResult>
    /// ```
    fn write(self) -> HttpResponse {
        let (parts, r) = self.into_parts();
        let mut w = BodyWriter::new_xml();

        w.append("<ListAllMyBucketsResult>");

        w.append("<Buckets>");
        for b in r.buckets {
            w.append("<Bucket>");
            w.append_xml("Name", b.name.as_str());
            w.append_xml("CreationDate", "2021-09-19T00:00:00.000Z");
            w.append("</Bucket>");
        }
        w.append("</Buckets>");

        w.append("<Owner>");
        w.append_xml("ID", r.owner.id.as_str());
        w.append_xml("DisplayName", r.owner.display_name.as_str());
        w.append("</Owner>");

        w.append("</ListAllMyBucketsResult>");
        Response::from_parts(parts, w.body())
    }
}

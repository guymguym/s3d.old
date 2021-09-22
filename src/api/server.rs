use crate::api::*;
use crate::layers::*;
use hyper::Method;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use tokio::sync::OnceCell;

pub async fn serve() -> Result<(), SyncError> {
    let addr = ([127, 0, 0, 1], 3000).into();
    s3_init().await;
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(s3_handler))
    }));
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

type SRV = S3Server<MemLayer>;
// type SRV = S3Server<MockLayer>;
// type SRV = S3Server<S3Layer>;

// keep the server alive statically
// because we need it for the lifetime of the program
static S3D: OnceCell<SRV> = OnceCell::const_new();

pub async fn s3_init() {
    S3D.set(S3Server::new()).unwrap();
}

pub async fn s3_handler(req: HttpRequest) -> HttpResult {
    S3D.get().unwrap().handler(req).await
}

#[derive(Debug)]
pub struct S3Server<API: ApiLayer> {
    api: API,
}

impl<API: ApiLayer> S3Server<API> {
    pub fn new() -> S3Server<API> {
        S3Server { api: API::new() }
    }

    pub async fn handler(&self, req: HttpRequest) -> HttpResult {
        let method = req.method().to_owned();
        let uri = req.uri().to_owned();

        println!("HTTP ==> {} {} {:?}", method, uri, &req.headers());

        // path style addressing
        assert!(uri.path().starts_with("/"));
        let path_items: Vec<_> = uri.path()[1..].splitn(2, "/").collect();
        let bucket = path_items.get(0).unwrap_or(&"");
        let key = path_items.get(1).unwrap_or(&"");
        let op_match = (method.to_owned(), !bucket.is_empty(), !key.is_empty());

        let res: HttpResult = match op_match {
            LIST_BUCKETS => self
                .api
                .list_buckets(list_buckets::Req::parse(req, bucket, key))
                .await
                .write(),

            LIST_OBJECTS => self
                .api
                .list_objects(list_objects::Req::parse(req, bucket, key))
                .await
                .write(),

            GET_BUCKET => self
                .api
                .get_bucket(get_bucket::Req::parse(req, bucket, key))
                .await
                .write(),

            GET_OBJECT | HEAD_OBJECT => self
                .api
                .get_object(get_object::Req::parse(req, bucket, key))
                .await
                .write(),

            PUT_BUCKET => self
                .api
                .put_bucket(put_bucket::Req::parse(req, bucket, key))
                .await
                .write(),

            PUT_OBJECT => self
                .api
                .put_object(put_object::Req::parse(req, bucket, key))
                .await
                .write(),

            DELETE_BUCKET => self
                .api
                .delete_bucket(delete_bucket::Req::parse(req, bucket, key))
                .await
                .write(),

            DELETE_OBJECT => self
                .api
                .delete_object(delete_object::Req::parse(req, bucket, key))
                .await
                .write(),

            _ => Ok(S3Error::BadRequest).write(),
        };

        println!("HTTP <== {} {} {:?}", method, uri, res);
        res
    }
}

/// OpMatch is a tuple for choosing the requested op based on:
/// 1. the http method
/// 2. the existence of a bucket name in the host or path
/// 3. the existence of a key in the path
type OpMatch = (Method, bool, bool);
const LIST_BUCKETS: OpMatch = (Method::GET, false, false);
const LIST_OBJECTS: OpMatch = (Method::GET, true, false);
const GET_BUCKET: OpMatch = (Method::HEAD, true, false);
const GET_OBJECT: OpMatch = (Method::GET, true, true);
const HEAD_OBJECT: OpMatch = (Method::HEAD, true, true);
const PUT_BUCKET: OpMatch = (Method::PUT, true, false);
const PUT_OBJECT: OpMatch = (Method::PUT, true, true);
const DELETE_BUCKET: OpMatch = (Method::DELETE, true, false);
const DELETE_OBJECT: OpMatch = (Method::DELETE, true, true);

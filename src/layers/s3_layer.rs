use crate::api::*;
use async_trait::async_trait;
use hyper::{body::to_bytes, Body};

#[derive(Debug, Clone)]
pub struct S3Layer {
    endpoint: String,
    access_key: String,
    secret_key: String,
    region: String,
    bucket: String,
    // s3_client: Arc<RwLock<S3Client>>,
}

#[async_trait]
impl ApiLayer for S3Layer {
    fn new() -> Self {
        S3Layer {
            endpoint: "".to_string(),
            access_key: "".to_string(),
            secret_key: "".to_string(),
            region: "".to_string(),
            bucket: "".to_string(),
            // s3_client: Arc::new(RwLock::new(S3Client::new())),
        }
    }

    async fn list_buckets(&self, _req: list_buckets::Req) -> list_buckets::Ret {
        let buckets = Vec::<BucketInfo>::new();
        Ok(list_buckets::Res::new(list_buckets::Reply {
            buckets,
            next_marker: String::new(),
            is_truncated: false,
            owner: UserInfo {
                id: String::from("222"),
                display_name: String::from("user222"),
            },
        }))
    }

    async fn get_bucket(&self, req: get_bucket::Req) -> get_bucket::Ret {
        let body = &req.into_body();
        let info = self.make_bucket_info(body.bucket.as_str());
        Ok(get_bucket::Res::new(get_bucket::Reply { info }))
    }

    async fn put_bucket(&self, req: put_bucket::Req) -> put_bucket::Ret {
        let body = &req.into_body();
        let info = self.make_bucket_info(body.bucket.as_str());
        Ok(put_bucket::Res::new(put_bucket::Reply { info }))
    }

    async fn delete_bucket(&self, req: delete_bucket::Req) -> delete_bucket::Ret {
        let body = &req.into_body();
        let info = self.make_bucket_info(body.bucket.as_str());
        Ok(delete_bucket::Res::new(delete_bucket::Reply { info }))
    }

    async fn list_objects(&self, req: list_objects::Req) -> list_objects::Ret {
        let body = &req.into_body();
        let mut objects = Vec::<ObjectInfo>::new();
        for i in 1..4 {
            let object =
                self.make_object_info(body.bucket.as_str(), format!("object_{}", i).as_str());
            if object.key.starts_with(&body.prefix) {
                objects.push(object.clone());
            }
        }
        let common_prefixes = Vec::<String>::new();
        Ok(list_objects::Res::new(list_objects::Reply {
            objects,
            common_prefixes,
            next_marker: String::new(),
            is_truncated: false,
            bucket: body.bucket.to_owned(),
            prefix: body.prefix.to_owned(),
            delimiter: body.delimiter.to_owned(),
            marker: body.marker.to_owned(),
            max_keys: body.max_keys,
            encoding_type: body.encoding_type.to_owned(),
        }))
    }

    async fn get_object(&self, req: get_object::Req) -> get_object::Ret {
        let body = &req.into_body();
        let object = self.make_object_info(body.bucket.as_str(), body.key.as_str());
        Ok(get_object::Res::new(get_object::Reply {
            object,
            body: if body.head_only {
                None
            } else {
                Some(Body::from(""))
            },
        }))
    }

    async fn put_object(&self, req: put_object::Req) -> put_object::Ret {
        let body = req.into_body();
        let buf = to_bytes(body.body.unwrap()).await.unwrap();
        let mut object = self.make_object_info(body.bucket.as_str(), body.key.as_str());
        object.size = buf.len() as u64;
        Ok(put_object::Res::new(put_object::Reply { object }))
    }

    async fn delete_object(&self, req: delete_object::Req) -> delete_object::Ret {
        let body = &req.into_body();
        let object = self.make_object_info(body.bucket.as_str(), body.key.as_str());
        Ok(delete_object::Res::new(delete_object::Reply { object }))
    }
}

impl S3Layer {
    fn make_bucket_info(&self, bucket: &str) -> BucketInfo {
        BucketInfo {
            name: bucket.to_string(),
            class: format!("class_{}", bucket),
            region: format!("region_{}", bucket),
            owner: UserInfo {
                id: format!("user_id_{}", bucket),
                display_name: format!("user_name_{}", bucket),
            },
        }
    }

    fn make_object_info(&self, bucket: &str, key: &str) -> ObjectInfo {
        ObjectInfo {
            bucket: bucket.to_string(),
            key: key.to_string(),
            version_id: format!("version_id_{}_{}", bucket, key),
            last_modified: format!("last_modified_{}_{}", bucket, key),
            etag: format!("etag_{}_{}", bucket, key),
            storage_class: format!("storage_class_{}_{}", bucket, key),
            size: 0,
            owner: UserInfo {
                id: format!("user_id_{}_{}", bucket, key),
                display_name: format!("user_name_{}_{}", bucket, key),
            },
        }
    }
}

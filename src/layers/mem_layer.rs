use crate::api::*;
use async_trait::async_trait;
use hyper::{
    body::{to_bytes, Bytes},
    Body,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type BucketsArc = Arc<RwLock<HashMap<String, BucketArc>>>;
type BucketArc = Arc<RwLock<Bucket>>;
type ObjectArc = Arc<RwLock<Object>>;

#[derive(Debug, Clone)]
pub struct MemLayer {
    buckets_arc: BucketsArc,
}

#[derive(Debug, Clone)]
struct Bucket {
    info: BucketInfo,
    objects: HashMap<String, ObjectArc>,
}

#[derive(Debug, Clone)]
struct Object {
    object: ObjectInfo,
    buf: Bytes,
}

#[async_trait]
impl ApiLayer for MemLayer {
    fn new() -> Self {
        let buckets_arc = Arc::new(RwLock::new(HashMap::new()));
        MemLayer { buckets_arc }
    }

    async fn list_buckets(&self, _req: list_buckets::Req) -> list_buckets::Ret {
        let mut buckets = Vec::<BucketInfo>::new();
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        for it in buckets_rlock.values() {
            let bucket_arc = Arc::clone(it);
            let bucket_rlock = bucket_arc.read().unwrap();
            buckets.push(bucket_rlock.info.clone());
        }
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
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        let bucket_arc = match buckets_rlock.get(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let bucket_rlock = bucket_arc.read().unwrap();
        let info = bucket_rlock.info.clone();
        Ok(get_bucket::Res::new(get_bucket::Reply { info }))
    }

    async fn put_bucket(&self, req: put_bucket::Req) -> put_bucket::Ret {
        let body = &req.into_body();
        let info = self.make_bucket_info(body.bucket.as_str());
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let mut buckets_wlock = buckets_arc.write().unwrap();
        if buckets_wlock.contains_key(&body.bucket) {
            return Err(S3Error::BucketAlreadyExists);
        }
        let bucket_arc = Arc::new(RwLock::new(Bucket {
            info: info.clone(),
            objects: HashMap::new(),
        }));
        buckets_wlock.insert(body.bucket.to_owned(), bucket_arc);
        Ok(put_bucket::Res::new(put_bucket::Reply { info }))
    }

    async fn delete_bucket(&self, req: delete_bucket::Req) -> delete_bucket::Ret {
        let body = &req.into_body();
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let mut buckets_wlock = buckets_arc.write().unwrap();
        let bucket_arc = match buckets_wlock.remove(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let mut bucket_wlock = bucket_arc.write().unwrap();
        let info = bucket_wlock.info.clone();
        bucket_wlock.objects.clear();
        Ok(delete_bucket::Res::new(delete_bucket::Reply { info }))
    }

    async fn list_objects(&self, req: list_objects::Req) -> list_objects::Ret {
        let body = &req.into_body();
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        let bucket_arc = match buckets_rlock.get(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let bucket_rlock = bucket_arc.read().unwrap();
        let mut objects = Vec::<ObjectInfo>::new();
        for it in bucket_rlock.objects.values() {
            let object_arc = Arc::clone(it);
            let object_rlock = object_arc.read().unwrap();
            if object_rlock.object.key.starts_with(&body.prefix) {
                objects.push(object_rlock.object.clone());
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
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        let bucket_arc = match buckets_rlock.get(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let bucket_rlock = bucket_arc.read().unwrap();
        let object_arc = match bucket_rlock.objects.get(&body.key) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchKey),
        };
        let object_rlock = object_arc.read().unwrap();
        Ok(get_object::Res::new(get_object::Reply {
            object: object_rlock.object.clone(),
            body: if body.head_only {
                None
            } else {
                Some(Body::from(object_rlock.buf.clone()))
            },
        }))
    }

    async fn put_object(&self, req: put_object::Req) -> put_object::Ret {
        let body = req.into_body();
        let buf = to_bytes(body.body.unwrap()).await.unwrap();
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        let bucket_arc = match buckets_rlock.get(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let mut bucket_wlock = bucket_arc.write().unwrap();
        let mut object = self.make_object_info(body.bucket.as_str(), body.key.as_str());
        object.size = buf.len() as u64;
        let object_arc = Arc::new(RwLock::new(Object {
            object: object.clone(),
            buf,
        }));
        bucket_wlock.objects.insert(body.key.to_owned(), object_arc);
        Ok(put_object::Res::new(put_object::Reply { object }))
    }

    async fn delete_object(&self, req: delete_object::Req) -> delete_object::Ret {
        let body = &req.into_body();
        let buckets_arc = Arc::clone(&self.buckets_arc);
        let buckets_rlock = buckets_arc.read().unwrap();
        let bucket_arc = match buckets_rlock.get(&body.bucket) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchBucket),
        };
        let mut bucket_wlock = bucket_arc.write().unwrap();
        let object_arc = match bucket_wlock.objects.remove(&body.key) {
            Some(a) => Arc::clone(&a),
            None => return Err(S3Error::NoSuchKey),
        };
        let mut object_wlock = object_arc.write().unwrap();
        let object = object_wlock.object.clone();
        object_wlock.buf.clear();
        Ok(delete_object::Res::new(delete_object::Reply { object }))
    }
}

impl MemLayer {
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

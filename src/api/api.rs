use crate::api::*;
use async_trait::async_trait;

/// ApiLayer is an abstract API to an S3-like object store.
/// implementing this trait allows to extend the composable API's.
#[async_trait]
pub trait ApiLayer
where
    Self: Send + Sync,
{
    fn new() -> Self;

    async fn list_buckets(&self, req: list_buckets::Req) -> list_buckets::Ret;
    async fn get_bucket(&self, req: get_bucket::Req) -> get_bucket::Ret;
    async fn put_bucket(&self, req: put_bucket::Req) -> put_bucket::Ret;
    async fn delete_bucket(&self, req: delete_bucket::Req) -> delete_bucket::Ret;

    async fn list_objects(&self, req: list_objects::Req) -> list_objects::Ret;
    async fn get_object(&self, req: get_object::Req) -> get_object::Ret;
    async fn put_object(&self, req: put_object::Req) -> put_object::Ret;
    async fn delete_object(&self, req: delete_object::Req) -> delete_object::Ret;

    // TODO: multipart upload
    // async fn initiate_multipart_upload(&self, req: InitiateMultipartUpload::Req) -> InitiateMultipartUpload::Res;
    // async fn complete_multipart_upload(&self, req: CompleteMultipartUpload::Req) -> CompleteMultipartUpload::Res;
    // async fn put_upload_part(&self, req: PutUploadPart::Req) -> PutUploadPart::Res;
    // async fn list_upload_parts(&self, req: ListUploadParts::Req) -> ListUploadParts::Res;

    // TODO: batch object operations (see http://docs.aws.amazon.com/AmazonS3/latest/API/batch-ops-deleting-objects.html)
    // async fn get_objects(&self, req: GetObjects::Req) -> GetObjects::Res;
    // async fn put_objects(&self, req: PutObjects::Req) -> PutObjects::Res;
    // async fn delete_objects(&self, req: DeleteObjects::Req) -> DeleteObjects::Res;

    // TODO: versioning
    // async fn list_object_versions(&self, req: ListObjects::Req) -> ListObjects::Res;
}

#[derive(Debug, Clone)]
pub struct BucketInfo {
    pub name: String,
    pub class: String,
    pub region: String,
    pub owner: UserInfo,
}

#[derive(Debug, Clone)]
pub struct ObjectInfo {
    pub bucket: String,
    pub key: String,
    pub version_id: String,
    pub size: u64,
    pub last_modified: String,
    pub etag: String,
    pub storage_class: String,
    pub owner: UserInfo,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct ObjectRange {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

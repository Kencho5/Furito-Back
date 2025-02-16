use crate::prelude::*;
use aws_sdk_s3::operation::put_object::PutObjectOutput;

pub async fn upload_image(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
    image: s3::primitives::ByteStream,
    key: &str,
) -> Result<PutObjectOutput, s3::Error> {
    let body = aws_sdk_s3::primitives::ByteStream::from(image);

    Ok(client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .content_type("image/jpeg")
        .send()
        .await?)
}

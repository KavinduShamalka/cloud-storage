use google_cloud_storage::{client::{ClientConfig, Client}, http::objects::upload::{UploadType, Media, UploadObjectRequest}, http::Error};


pub(crate) async fn run() -> Result<String, Error> {

    println!("Hello from bucket create");
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config);


    // Upload the file
    let upload_type = UploadType::Simple(Media::new("file.png"));
    let uploaded = client.upload_object(&UploadObjectRequest {
        bucket: "first_bucket-1".to_string(),
        ..Default::default()
    }, "hello world".as_bytes(), &upload_type).await?;

    let name =  uploaded.name;

    // let create = client.insert_bucket_access_control(&InsertBucketRequest {
    //     name: "bucket_1".to_string(),
    //     param: InsertBucketParam { project: "project_1".to_string(), 
    //     predefined_acl: (), predefined_default_object_acl: (), projection: () }
    // })

    // let bucket = client.

    Ok(name)
}
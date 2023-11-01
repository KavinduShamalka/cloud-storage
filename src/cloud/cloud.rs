use std::{fs::File, io::Read};

use cloud_storage::{Error, Client, NewBucket, ListRequest};

//"doctest-bucket-test"

/*
Create bucket
*/
pub(crate) async fn _create_bucket(bucket_name: String) -> Result<String, Error> {
    let client = Client::default();
    let bucket = client.bucket().create(&NewBucket {
        name: bucket_name,
        ..Default::default()
    }).await?;

    let name = bucket.name;

    Ok(name)
}

//Read bucket
pub async fn _read_bucket(bucket_name: String) -> Result<String, Error> {

    let client = Client::default();
    let bucket = client.bucket().read(&bucket_name).await?;

    Ok(bucket.name)

}

//Read a file from disk and store it on googles server:
pub async fn _store(bucket_name: String, file_name: String, format: String) -> Result<String, Error> {

    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open("./src/cloud/cat.png")?.bytes() {
        bytes.push(byte?)
    }
   
    let client = Client::default();
    let file = client.object().create(&bucket_name, bytes, &file_name, &format).await?;
    Ok(file.name)

}


//Rename file
pub async fn _content_type_change(bucket_name: String, file_name: String) -> Result<String, Error> {

    let client = Client::default();
    let mut object = client.object().read(&bucket_name, &file_name).await?;
    object.content_type= Some("application/xml".to_string());
    client.object().update(&object).await?;

    let updated_name = match object.content_type {
        Some(name) => name,
        None => "_".to_string()
    };

    Ok(updated_name)
}

//Delete file
pub async fn _delete_file(bucket_name: String, file_name: String) -> Result<(), Error> {

    let client = Client::default();
    let delete = client.object().delete(&bucket_name, &file_name).await?;

    Ok(delete)

}

//Delete bucket
pub async fn _delete_bucket(bucket_name: String) -> Result<(), Error> {

    let client = Client::default();
    let bucket = client.bucket().read(&bucket_name).await?;
    let delete = client.bucket().delete(bucket).await?;
    Ok(delete)

}

//get bucket list 
pub async fn _list_buckets() -> Result<(), Error> {

    let client = Client::default();
    let list = client.bucket().list().await?;

    let _file = client.object().list("doctest-bucket-test", ListRequest::default()).await;

    for buckets in list {

        println!("{}",buckets.name);

    }

    Ok(())

}
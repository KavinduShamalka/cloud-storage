// mod api;
mod cloud;

#[tokio::main]
async fn main() {

    let mut _bucket = "kavindu".to_string();
    let _store_file = "cat_cat.png".to_string();
    let _format = "image/png".to_string();
    //Create bucket
    let bucket = cloud::cloud::_create_bucket(_bucket.clone()).await;

    match bucket {
        Ok(name) => println!("Bucket {:?} created succesfully",name),
        Err(err) => println!("Error code: {:?}", err)
    }

    //Read bucket
    // let read_bucket = cloud::cloud::_read_bucket(_bucket.clone()).await;

    // match read_bucket{
    //     Ok(name) => {
    //         println!();
    //         println!("Bucket {} readed successfully ", name);
    //         println!();
    //     },
    //     Err(err) => println!("Error code: {:?}", err)
    // }

    //Store file
    // let store_bucket = cloud::cloud::_store(_bucket.clone(), _store_file.clone(), _format).await;

    // match store_bucket {
    //     Ok(name) => {
    //         println!();
    //         println!("File {} stored successfully ", name);
    //         println!();
    //     },
    //     Err(err) => println!("Error code: {:?}", err)   
    // }

    //Rename file
    // let rename =  cloud::cloud::_content_type_change(_bucket.clone(), _store_file).await;

    // match rename {
    //     Ok(name) => {   
    //         println!();
    //         println!("File content type change to {} successfully ", name);
    //         println!();
    //     },
    //     Err(err) => println!("Error code: {:?}", err)   
    // }

    //delete file
    // let delete  = cloud::cloud::_delete_file(_bucket.clone(), _store_file.clone()).await;

    // match delete {
    //     Ok(_) => {   
    //         println!();
    //         println!("File deleted successfully ");
    //         println!();
    //     },
    //     Err(err) => println!("Error deleting file : {:?}", err)   
    // }

    //list
    // match cloud::cloud::_list_buckets().await {
    //     Ok(()) => (),
    //     Err(error) => println!("Error : {:?}", error) 
    // }

    //delete bucket
    // let delete_bucket = cloud::cloud::_delete_bucket(_bucket.clone()).await;

    // match delete_bucket {
    //     Ok(_) => {
    //         println!();
    //         println!("Bucket {} deleted successfully ", _bucket);
    //         println!();  
    //     },
    //     Err(error) => {
    //         println!();
    //         println!("Error deleting bucket: {:?} ", error);
    //         println!();  
    //     }
    // }

}

use std::env;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::Client;

async fn get_client() -> aws_sdk_s3::Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);

    return client;
}

// pub async fn upload(form: FormData, s3_client: Client) {
//     let mut uploaded_image = None;
//     let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");

//     // Process the form data
//     let parts = form.try_collect().await.map_err(|e| {
//         eprintln!("Error collecting form data: {}", e);
//         warp::reject::reject()
//     });

//     match parts {
//         Ok(parts) => {
//             for part in parts {
//                 if part.name() == "file" {
//                     let filename = part.filename().unwrap_or("upload").to_string();
//                     let data = part
//                         .stream()
//                         .try_fold(Vec::new(), |mut vec, bytes| async move {
//                             vec.extend(bytes);
//                             Ok(vec)
//                         })
//                         .await
//                         .map_err(|_| warp::reject::reject());
//                     uploaded_image = Some((filename, data));
//                 }
//             }

//             if let Some((filename, file))
//         }
//     }
// }

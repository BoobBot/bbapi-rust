extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;


#[derive(Debug)]
#[derive(Deserialize)]
pub struct Response {
    url: String,
}

/*
 * Get's a image url for the specified type
 */
pub fn get_img(img_type: String, api_key: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let img_url = "https://boob.bot/api/v2/img/";
    let url_to_praise = reqwest::Url::parse(&img_url).ok().expect("Expected a valid URL");
    let img_url = url_to_praise.join(&img_type).ok().expect("Failed to join url");
    let builder = client.get(img_url).header("key", api_key);
    let mut req = builder.send().ok().expect("Failed to send request");
    if req.status().is_success() {
        let img: Response = req.json().unwrap();
        Ok(img.url)
    } else {
        Err(format!("Something went wrong. Status code: {:?}", req.status()))
    }
}

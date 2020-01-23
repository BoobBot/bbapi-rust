extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;


#[derive(Debug, Deserialize)]
pub struct Response {
    url: String,
}

/*
 * Get's a image url for the specified type
 */
pub fn get_img(img_type: String, api_key: String) -> Result<String, String> {
    let img_url = {
        let img_url = "https://boob.bot/api/v2/img/";
        let url_to_praise = reqwest::Url::parse(&img_url).map_err(|_| "Expected a valid URL".to_string())?;
        url_to_praise.join(&img_type).map_err(|_| "Failed to join url".to_string())?
    };

    let resp = reqwest::blocking::Client::new().get(img_url).header("key", api_key)
        .send().map_err(|_| "Failed to send request".to_string())?;

    if resp.status().is_success() {
        let img: Response = resp.json().unwrap();
        Ok(img.url)
    } else {
        Err(format!("Something went wrong. Status code: {}", resp.status()))
    }
}

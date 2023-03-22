/// Downloads a website to a String
pub fn download_text(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url)
        .call()?
        .into_string()?;
    Ok(body)
}
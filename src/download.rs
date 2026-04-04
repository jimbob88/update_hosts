/// Downloads a website to a String
pub fn download_text(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url)
        .call()?
        .body_mut()
        .read_to_string()?;
    Ok(body)
}

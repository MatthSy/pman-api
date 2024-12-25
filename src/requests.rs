#[allow(dead_code)]
pub async fn get_password(password_id: &str) -> Result<String, reqwest::Error> {
    let url = format!("http://localhost:8000/passwords/{}", password_id);
    let response = reqwest::get(&url).await?.text().await?;
    Ok(response)
}

#[allow(dead_code)]
pub async fn get_all_passwords() -> Result<String, reqwest::Error> {
    let url = "http://localhost:8000/passwords";
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}
use reqwest;

#[derive(Debug, thiserror::Error)]
pub enum NgrokError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Ngrok tunnel not found")]
    NgrokTunelNotFound,
}

#[derive(serde_derive::Deserialize)]
struct Tunnel {
    public_url: String,
    name: String,
}

#[derive(serde_derive::Deserialize)]
struct ApiResponse {
    tunnels: Vec<Tunnel>,
}

pub async fn fetch_ngrok_url(tunnel_name: &str) -> Result<String, NgrokError> {
    let resp = reqwest::get("http://127.0.0.1:4040/api/tunnels")
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(resp
        .tunnels
        .into_iter()
        .find(|t| t.name == tunnel_name)
        .map(|t| t.public_url)
        .ok_or_else(|| NgrokError::NgrokTunelNotFound)?)
}

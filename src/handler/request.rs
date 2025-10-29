use std::fmt::Display;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{SPClient, SPResult};

pub async fn get<T: DeserializeOwned>(client: &SPClient, path: impl Display) -> SPResult<T> {
    let resp = client.http_client.get(format!("{}{}", client.base_url(), path))
        .header("Authorization", format!("{}", client.token))
        .header("Accept", "application/json")
        .send().await?;

    resp.json::<T>().await
}

pub async fn post_clear<I: Serialize>(client: &SPClient, path: impl Display, data: &I, strip_quotes: bool) -> SPResult<String> {
    let resp = client.http_client.post(format!("{}{}", client.base_url(), path))
        .header("Authorization", format!("{}", client.token))
        .header("Content-Type", "application/json")
        .json(data)
        .send()
        .await?;

    let s = resp.text().await?;

    if cfg!(test) {
        println!("post: {}", s);
    }

    if strip_quotes {
        Ok(s.trim_matches('"').to_string())
    } else {
        Ok(s)
    }

}

async fn patch0<I: Serialize>(client: &SPClient, path: impl Display, data: &I) -> SPResult<Response> {
    client.http_client.patch(format!("{}{}", client.base_url(), path))
        .header("Authorization", format!("{}", client.token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(data)
        .send()
        .await
}

pub async fn patch_without_response<I: Serialize>(client: &SPClient, path: impl Display, data: &I) -> SPResult<()> {
    let resp = patch0(client, path, data).await?;

    if cfg!(test) {
        println!("patch: {}", resp.text().await?);
    }

    Ok(())
}

pub async fn patch<I: Serialize, T: DeserializeOwned>(client: &SPClient, path: impl Display, data: &I) -> SPResult<T> {
    let resp = patch0(client, path, data).await?;

    resp.json::<T>().await
}

pub async fn delete_without_response(client: &SPClient, path: impl Display) -> SPResult<()> {
    client.http_client.delete(format!("{}{}", client.base_url(), path))
        .header("Authorization", format!("{}", client.token))
        .send()
        .await?;

    Ok(())
}
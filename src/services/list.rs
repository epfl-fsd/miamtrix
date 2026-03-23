use crate::utils::api::ApiClient;
use crate::models::cafeteria::Cafeteria;
use std::fmt::Write;

pub async fn list_restaurant() -> String {
    let response = ApiClient::get().await.unwrap();
    let cafeterias: Vec<Cafeteria> = response.json().await.unwrap();
    let mut message = String::new();
    let _ = writeln!(message, "### Restaurant list :\n");
    for resto in cafeterias {
        let _ = writeln!(message, "- {}", &resto.name);
    }
    message
}

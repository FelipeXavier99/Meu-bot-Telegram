// script que mostra no terminal os id do clientes após clicar no link do bot


use reqwest::Client;

async fn capturar_chat_ids(token: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.telegram.org/bot{}/getUpdates",
        token
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Retorno da API: {}", body);
    } else {
        println!(
            "Erro ao capturar Chat IDs: {}",
            response.text().await.unwrap_or_else(|_| "Erro desconhecido".to_string())
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = "botar seu TOKEN de bot";
    if let Err(e) = capturar_chat_ids(token).await {
        eprintln!("Erro: {}", e);
    }
}

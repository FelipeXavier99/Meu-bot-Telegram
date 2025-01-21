//funcioando normalmente direto no meu bot pra mandar msg

use reqwest::Client;

async fn enviar_mensagem(chat_id: &str, mensagem: &str) -> Result<(), reqwest::Error> {
    let token = "77599...:AAFLFnS_-NI....."; // Token do bot
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token
    );

    let params = [
        ("chat_id", chat_id),
        ("text", mensagem),
    ];

    let client = Client::new();
    let response = client.post(&url).form(&params).send().await?;

    if response.status().is_success() {
        println!("Mensagem enviada com sucesso!");
    } else {
        println!(
            "Erro ao enviar mensagem: {}",
            response.text().await.unwrap_or_else(|_| "Erro desconhecido".to_string())
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Substitua "123456" pelo seu Chat ID e personalize a mensagem se necessário. (abaix ira mandar msg pro Telegram)
    if let Err(e) = enviar_mensagem("123456", "Pagamento atrasado!").await {
        eprintln!("Erro: {}", e);
    }
}

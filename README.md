# Meu-bot-Telegram

###  Criando mensagens automáticas no Telegram para clientes que estão atrasando o pagamento mensalmente.

1º) Criar um Bot no Telegram
- Abra o Telegram e procure pelo usuário @BotFather.
- Envie o comando /newbot e siga as instruções para criar o bot.
- Você receberá um token de API do bot, algo como:
  
  123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11

2º) Nesse mesmo bot irá mostrar esse exemplo de link  t.me/teste_bot 
- primeito da /start no meu bot e manda qualquer mensagem nele
- após isso acima copia o link t.me.. e passara para o cliente clicar

3º) Executa o script "chatid" que irá mostrar o histórico de quem acessou o link t.me..
- com isso já da pra pegar o id do cliente que fica em
- "chat": {
          "id": 123456789

4º) Por último só executar o script main com o meu token bot e o id do cliente cadastrado que irá enviar a mensagem
  para o bot do cliente pelo telegram!

# trabalho-redes
Trabalho de Administração de Redes do terceiro trimestre do quarto ano do curso de Desenvolvimento de Sistemas do IFRS Campus Canoas sobre Sockets desenvolvido com a linguagem Rust

## Ideia
- A ideia principal é demonstrar o uso de sockets.

## Teoria

- Um servidor WebSocket será iniciado, e irá ouvir por conexões em uma determinada porta por meio do protocolo Websocket, que está rodando com base no protocolo TCP.
- Um cliente iniciará uma conexão Websocket com o destino do endereço do servidor. Essa conexão ocorrerá, primeiramente, via HTTP. A conexão HTTP, posteriormente, será transformada (o que é chamado de "upgrade") em uma conexão Websocket, e então tanto servidor quanto cliente poderão trocar informações em tempo real (ou seja, o Websocket permite uma comunicação em tempo real _e_ de mão dupla, ou seja, tanto o servidor quanto o cliente podem enviar e receber dados).

### Autores
* Arthur Von Groll, Milena Schmitt Scheidt e Vinícius Amorim

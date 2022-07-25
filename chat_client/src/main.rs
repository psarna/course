use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use std::env;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct ChatClient {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
}

impl ChatClient {
    async fn connect(addr: &str) -> Result<Self> {
        let (read_half, write_half) = TcpStream::connect(addr).await?.into_split();
        Ok(Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
        })
    }

    async fn register(&mut self, username: &str) -> Result<()> {
        self.writer.write_all(b"login ").await?;
        self.writer.write_all(username.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<String> {
        let mut line = String::new();
        let n = self
            .reader
            .read_line(&mut line)
            .await
            .expect("failed to read data from socket");
        Ok(line)
    }

    async fn send(&mut self, recipient: &str, msg: &str) -> Result<()> {
        self.writer.write_all(b"send ").await?;
        self.writer.write_all(recipient.as_bytes()).await?;
        self.writer.write_all(b" ").await?;
        self.writer.write_all(msg.as_bytes()).await?;
        Ok(())
    }
}

async fn accept_message() -> Result<(String, String)> {
    let mut recipient = String::new();
    let mut message = String::new();

    let mut stdin = BufReader::new(tokio::io::stdin());
    println!("Enter recipient:");
    stdin.read_line(&mut recipient).await?;
    println!("Enter message:");
    stdin.read_line(&mut message).await?;
    Ok((recipient, message))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let addr = args.next().unwrap_or_else(|| "localhost:8123".to_string());

    let mut client = ChatClient::connect(&addr).await?;
    println!("Please type in your username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    client.register(&username).await?;

    /*loop {
        tokio::select! {
            command
        }
    }*/

    /*loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let (raw_socket, peer) = accept_result?;
                println!("Accepted a connection from {}", peer);
                let (read_half, write_half) = raw_socket.into_split();
                let mut read_half = tokio::io::BufReader::new(read_half);

                let chat = global_chat.clone();
                tokio::spawn(async move {
                    let mut user: Option<String>;
                    loop {
                        let mut line = String::new();
                        let n = read_half.read_line(&mut line).await.expect("failed to read data from socket");
                        if n == 0 {
                            println!("{} disconnected", peer);
                            return
                        }
                        let line = strip_newline(&line);
                        let mut chat_locked = chat.lock().await;
                        user = chat_locked.handle_login(line).await;
                        match user {
                            Some(ref name) => {
                                chat_locked.register(&name, write_half).await;
                                break
                            },
                            None => (),
                        }
                    }
                    let user = user.unwrap();
                    loop {
                        let mut line = String::new();
                        let n = read_half
                            .read_line(&mut line)
                            .await
                            .expect("failed to read data from socket");
                        if n == 0 {
                            println!("{} disconnected", peer);
                            return
                        }
                        let line = strip_newline(&line);

                        let mut chat_locked = chat.lock().await;
                        chat_locked.handle_message(&user, line).await;
                    }
                });
            },
            msg = receiver.recv() => {
                let msg = msg.unwrap();
                let mut chat_locked = global_chat.lock().await;
                match chat_locked.peers_by_name.get_mut(&msg.recipient) {
                    Some(write_half) => {
                        write_half.write_all(msg.body.as_bytes()).await.expect("failed to write response into socket");
                    },
                    None => {
                        println!("Nonexistent recipient: {}", msg.recipient);
                    }
                }
            }
        }
    }*/
    Ok(())
}

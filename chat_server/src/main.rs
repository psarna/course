use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Message {
    sender: String,
    recipient: String,
    body: String,
}

struct ChatServer {
    peers_by_name: HashMap<String, OwnedWriteHalf>,
    peers_by_addr: HashMap<SocketAddr, String>,
    outbox: mpsc::Sender<Message>,
}

impl ChatServer {
    fn new(outbox: mpsc::Sender<Message>) -> Self {
        Self {
            peers_by_name: HashMap::new(),
            peers_by_addr: HashMap::new(),
            outbox,
        }
    }

    async fn handle_login(&mut self, msg_str: &str) -> Option<String> {
        let mut msg_parts = msg_str.split(' ');
        let command = msg_parts.next().unwrap();
        match command {
            "login" => {
                let user = msg_parts.next().unwrap();
                if self.peers_by_name.contains_key(user) {
                    println!("Username {} is already taken!", user);
                    return None;
                }
                return Some(user.to_string());
            }
            _ => println!("First command should be 'login', not: {}", command),
        }
        return None;
    }

    async fn register(&mut self, user: &str, write_half: OwnedWriteHalf) {
        let addr = write_half.local_addr().unwrap();
        self.peers_by_name.insert(user.to_owned(), write_half);
        self.peers_by_addr.insert(addr, user.to_owned());
        println!("Peer {} registered in as {}", addr, user);
    }

    async fn handle_message(&mut self, user: &str, msg_str: &str) {
        println!("<{}>: {}", user, msg_str);
        let mut msg_parts = msg_str.split(' ');
        let command = msg_parts.next().unwrap();
        match command {
            "login" => {
                println!("User {} is already authenticated", user);
            }
            "send" => {
                let recipient = msg_parts.next().unwrap().to_string();
                let body = msg_parts.collect::<Vec<&str>>().join(" ");
                println!("Sending {} to {}", body, recipient);
                self.outbox
                    .send(Message {
                        sender: user.to_string(),
                        recipient,
                        body,
                    })
                    .await
                    .expect("failed to write message into the channel");
            }
            _ => println!("Unknown command: {}", command),
        }
    }
}

fn strip_newline(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or(line.strip_suffix("\n"))
        .unwrap_or(line)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost:8123".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let (sender, mut receiver) = mpsc::channel(1024);
    let global_chat: Arc<Mutex<ChatServer>> = Arc::new(Mutex::new(ChatServer::new(sender)));

    loop {
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
                        write_half.write_all(format!("{}:{}\n", msg.sender, msg.body).as_bytes()).await.expect("failed to write response into socket");
                    },
                    None => {
                        println!("Nonexistent recipient: {}", msg.recipient);
                    }
                }
            }
        }
    }
}

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use std::env;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct ChatClient {
    writer: BufWriter<OwnedWriteHalf>,
}

impl ChatClient {
    async fn connect(addr: &str) -> Result<(Self, BufReader<OwnedReadHalf>)> {
        let (read_half, write_half) = TcpStream::connect(addr).await?.into_split();
        Ok((
            Self {
                writer: BufWriter::new(write_half),
            },
            BufReader::new(read_half),
        ))
    }

    async fn register(&mut self, username: &str) -> Result<()> {
        self.writer.write_all(b"login ").await?;
        self.writer.write_all(username.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await?;
        println!("Registered successfully as {}", username);
        Ok(())
    }

    async fn receive(reader: &mut BufReader<OwnedReadHalf>) -> Result<String> {
        let mut line = String::new();
        reader
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
        self.writer.flush().await?;
        Ok(())
    }
}

async fn accept_message() -> Result<(String, String)> {
    let mut recipient = String::new();
    let mut message = String::new();

    let mut stdin = BufReader::new(tokio::io::stdin());
    println!("Enter recipient:");
    stdin.read_line(&mut recipient).await?;
    recipient.pop(); // get rid of the trailing newline character
    println!("Enter message:");
    stdin.read_line(&mut message).await?;
    Ok((recipient, message))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let addr = args.next().unwrap_or_else(|| "localhost:8123".to_string());

    let (mut client, mut reader) = ChatClient::connect(&addr).await?;
    println!("Please type in your username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    username.pop(); // get rid of the trailing newline character
    client.register(&username).await?;

    // A task for receiving and printing messages from other clients
    tokio::spawn(async move {
        loop {
            let incoming_msg = ChatClient::receive(&mut reader)
                .await
                .expect("Receiving message failed");
            println!("> {}", incoming_msg)
        }
    });

    // A task for reading a message request from standard input (usually just keyboard),
    // and sending it to other clients
    loop {
        let (recipient, msg) = accept_message()
            .await
            .expect("Accepting a message command from stdin failed");
        client
            .send(&recipient, &msg)
            .await
            .expect("Sending a message failed");
    }
}

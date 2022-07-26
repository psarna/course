# Interactive chat client

The chat client connects to the chat server, asks you for your username
and then continuously asks for messages to be sent by first asking
for the recipient and then for the message body.

Example run:
```
[sarna@localhost chat_client]$ cargo run
   Compiling rustchatclient v0.1.0 (/home/sarna/repo/course/chat_client)
    Finished dev [unoptimized + debuginfo] target(s) in 1.13s
     Running `/home/sarna/repo/course/target/debug/rustchatclient`
Please type in your username:
user1
Registered successfully as user1
Enter recipient:
user2
Enter message:
hello user2!
Enter recipient:
```

It also immediately prints messages received from other clients
as soon as they arrive from the server.

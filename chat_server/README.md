# Chat server

Implemented protocol:
  * `login <username>`
    
    this command logs us in as a specified user
    
    Example: `login user1`
  * `send <recipient> <message>`
    
    this command can only be used by an authenticated user, and it sends message <message> to recipient <recipient>
    
    Example: `send user2 hello there!`

How to run:
```
cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/home/sarna/repo/course/target/debug/rustchat`
Listening on: localhost:8123

```

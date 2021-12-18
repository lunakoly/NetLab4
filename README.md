# Home Assignment #3

The goal of this lab is team development of 2 protocols according to some formal tasks.

The protocols below have been in pair with [FenstonSingel](https://github.com/FenstonSingel).

## Testing

Run the:

```bash
cargo run -p <package>
```

and the

```bash
cargo run -p <package>-check
```

commands in separate terminals.

## Message Format

The message format is the same for both of the protocols.

A message is represented as a BSON document. BSON serialization is handy, because the first 4 bytes of a BSON document are the length of the whole message (little-endian), which makes accepting messages really easy. The receiver simply waits for 4 bytes first, then - for the `length - 4` bytes more, and after that they can simply pass the bytes to the deserializer.

From my point of view, a better approach would be to simply run a deserializer over the input stream, but implementing a proper incremental deserializer takes too much time, and common Rust implementations don't seem to provide a such (or I failed to properly understand the docs, idk).

## Amogus Terminal (1.2.10)

Also known as Terminal Access System.

### Terms

A _location_ is an absolute path to a directory.

A _role_ is a named list of allowed commands. _To have a permission to run a command_ means to have the command name in the list of commands of the current role.

A _session_ is a connection between a client and a server. There may be multiple sessions for the same user.

### About

The goal of TAS is to provide terminal access to a remote machine running the server. The user is assumed to be able to run the following commands:

- `ls` - See the list of files in the current directory.
- `cd <target>` - Move to another directory.
- `who` - Get the list of all users with the directories they are currently in.
- `login <user> <pass>` - Change the current user to a different one.
- `kill <user>` - Kill all sessions with the matching `user` located in the same directory as the command caller.

By default, any new client is a special `guest` user (the `guest` user has no password). The client must use the `login` command to become someone else.

The registration capabilities are not a part of the protocol, it's up to the server implementation to decide on how new users appear.

The protocol does not define any upper limits for any textual data or the message size.

The default port is 6969.

### Available Messages

```rust
pub enum ClientMessage {
    Execute { command: Vec<String> },
}

pub enum ServerMessage {
    Notification { message: String },
    Role { title: String, allowed_commands: Vec<String> },
    FilesList { files: Vec<String> },
    UsersList { users: Vec<(String, String)> },
    MoveTo { location: String },
    KillResult { killed_users_count: u32 }
}
```

The `Execute`'s `command` is an array of command arguments, starting with the command itself.

The `UsersList`'s `users` is an array of pairs `(user,location)`.

### Procedure

1. The connection is established. The server first sends a `Role` message describing the default role for the user (the name of the role + the list of the names of the allowed commands). Then, the server sends a `MoveTo` message to let the user know of their default location (a directory absolute path). The default user is `guest`.
1. The client issues commands:
    1. For each of the commands, the client receives either the corresponding server message with the result or a general `Notification` describing an error.
1. If the server drops the connection, then someone has killed the client.
1. The client simply drops the connection.

### Implementation

This repo contains a server implementation. The corresponding package is `tas-server`. The `tas-server-check` package is a trivial pseudo-client needed for testing purposes.

The server relies on the `members.json` file describing the existing users and roles. Right now it contains 3 roles:

```json
"roles": {
    "ghost": ["login", "ls", "cd", "who"],
    "crew": ["login", "ls", "cd", "who"],
    "amogus": ["login", "ls", "cd", "who", "kill"]
}
```

and 4 sample users with their corresponding passwords stored in plain text. `ghost` is the default role for the `guest` user.

## Distributed Primes Calculation (1.2.20)

### About

The idea of the protocol is to have a server that can distribute prime numbers calculations across multiple connected clients, and the clients may ask the server some information about the computation status (the `N` greatest prime numbers computed so far).

The protocol works according to the following logic: it's always the client who sends some requests, and the server simply responds to them. So, calculations are performed in a way that the client first asks for "something to compute", then computes the result, and then submits it to the server as a standalone request.

The protocol does not define any upper limits for any textual data or the message size.

The default port is 6969.

### Available Messages

```rust
pub enum ClientMessage {
    GetMaxNPrimes { count: u32 },
    GetRange { count: u32 },
    PublishResults { primes: Vec<u32> },
}

pub enum ServerMessage {
    Notification { message: String },
    MaxNPrimes { primes: Vec<u32> },
    Range { lower_bound: u32 },
    PublishingResult {},
}
```

The `GetRange`'s `count` is the number of sequential integers the client is willing to check.

When the server accepts a `PublishResults` message, it knows the corresponding range from the context (the `lower_bound` has been generated by the server, and the `count` has been passed to it with the `GetRange` message).

The `lower_bound` must be inclusive, and the `lower_bound + count` bound is exclusive.

### Procedure

1. The connection is established. The server waits for requests.
1. The client sends some requests:
    1. If the clients sends `GetMaxNPrimes`, and the server doesn't have the desired number of primes, it returns all available ones.
1. The client simply drops the connection.

### Implementation

This repo contains a client implementation. The corresponding package is `primes-client`. The `primes-client-check` package is a trivial pseudo-server needed for testing purposes.

The client has been implemented as an interactive console, but only the `GetMaxNPrimes` request may be performed by the user manually (via the `max` command). All the calculations are performed automatically in a separate thread, and all the necessary requests are sent in background. So the client can simply type `max 1` to see the newest prime number.

If the client can do some calculations, it sends a `GetRange { count: 10 }` message (10 is hardcoded). When it receives a `Range`, it starts computing the numbers within the range. The client code contains a `thread::sleep()` call waiting for 3 secs for debugging purposes. When the client is ready, it sends the `PublishResults` message with the prime numbers within the range.

The client starts in a way that it's not connected to anything. Use the `connect [host] [port]` command to perform the connection (by default, the address is `localhost:6969`).

## Links
- Formal requirements: https://insysnw.github.io/practice/hw/custom-protocol/

# words of wisdom

A project written in Rust that contains two applications:
- an asynchronous TCP server
- a command line client

## wisdom-server

```
wisdom server 0.1.0
a server providing words of wisdom to its clients
it requires that clients implement proof of work algorithm and solve the challenge

USAGE:
    server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --difficulty <difficulty>    difficulty setting
    -p, --port <port>                port to run on
    -w, --words <words>              database location (txt)
```

## wisdom-client

```
wisdom client 0.1.0
a client for retreiving words of wisdom from the wisdom server
it implements a proof of work algorithm by solving a challenge sent by the server

USAGE:
    client [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    interactive mode
    -V, --version        Prints version information

OPTIONS:
    -a, --addr <addr>    ip address of the server
    -p, --port <port>    port to connect to
```

### the protocol of communication

1. client initiates a connection
2. client sends `GET`
3. server sends `SLV 2:zC9iarFfZP34DYPutwnh4PMb6YVUmH:0`
4. client sends `2:zC9iarFfZP34DYPutwnh4PMb6YVUmH:18950`
5. server sends `QUO “Always Do What You Are Afraid To Do” – Ralph Waldo Emerson`
6. client sends `THX`

### proof of work

In order to get a quote from the server, the client must solve the challenge first. The challenge is a **Proof of Work** algorithm based on the [HashCash](https://en.wikipedia.org/wiki/Hashcash). SHA256 function is used for hashing.

The server sends `SLV 2:zC9iarFfZP34DYPutwnh4PMb6YVUmH:0` where
- `2` is the minimum number of zero *bytes* the hash must start with
- `zC9iarFfZP34DYPutwnh4PMb6YVUmH` is a randomly generated text
- `0` is the initial counter value

The difficulty of the challenge can be specified as the command line option. It defines the minumun number of zero bytes that the hash of the challenge must start with.

The client iteratively calculates the hash of the message and checks the condition. If the condition is met, that is, the hash starts with at least the number of zero bytes specified (in this example, at least two leading zeros), the answer is found. If the condition is not met, the client increments the counter and repeats the steps - over and over again, until the solution is found.

### quotes database

When the client solves the challenge, it is served with random quote from the database of quotes. One can provide his own database of quotes using a command line option. The format of the database is a text file containing each quote separated by a new line. First line of the file is ignored.

### interactive mode

By default, when the client gets the quote from the server, it prints the quote to the stdout and closes.

If the `-i` command line flag is specified when running the client, it runs in the *interactive mode*. In this mode, the client asks the user if it should get the quote and, based on the user input, gets the quote or quits.

## docker

The Dockerfiles for both the server and the client are provided.

### building

```
docker build -t wisdom-server -f dockerfiles/server/Dockerfile .
docker build -t wisdom-client -f dockerfiles/client/Dockerfile .
```

### running

```
docker run -p 3962:3962 wisdom-server
docker run -ti wisdom-client -a 172.17.0.2
```

When running in a container, the client should know the address of the server.

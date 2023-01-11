# askrs - DNS lookup tool

askrs is a CLI tool to perform basic DNS lookups over a UDP socket connection for:

- A Records
- AAAA Records
- CNAME Records
- NS Records
- MX Records

As source DNS Servers the user has the choice between:

- google DNS (8.8.8.8)
- cloudfalre DNS (1.1.1.1)
- quad 9 DNS (9.9.9.9)

This repository serves as a learning-by-doing project for me to get comfortable with rust and network fundamentals. The first goal was to get everything working, so the quality / performance of the code is more than sub-optimal.

The following sources and references were used to create this project:
- [The Rust book](https://doc.rust-lang.org/book/)
- [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035)
- [DNS Message Blog](https://cabulous.medium.com/dns-message-how-to-read-query-and-response-message-cfebcb4fe817)
- [Hand writing DNS queries](http://russellcoleman.org/posts/1)
- [UDP Socket programming: DNS](https://w3.cs.jmu.edu/kirkpams/OpenCSF/Books/csf/html/UDPSockets.html)

## Usage 

```
USAGE:
    cli [OPTIONS] --uri <URI>

OPTIONS:
    -h, --help                 Print help information
    -r, --recursion_desired    Recursive Query
        --record <RECORD>      Record type to request (A, AAAA, MX, NS, CNAME) [default: A]
    -s, --server <SERVER>      DNS server to use for request [default: google]
    -u, --uri <URI>            Target adress or domain to request records for
    -V, --version              Print version information
EXAMPLE:
    cargo run -- -u google -s cloudflare --record NS -r
```

## Roadmap

To continue learning about rust and improve the code base I am thinking about the following upcoming changes:

1. Imporve Output formatting, add a CLI flag for verbosity
2. Refactoring data parsing for better performance
3. Implementing persisting / caching response data with files (i.e JSON) or with SQLite
4. Taking a look into async Websockets with [tokio](https://tokio.rs/) to make multiple requests efficiently

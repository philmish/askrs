# askrs - DNS lookup tool

![Tests](https://github.com/philmish/askrs/workflows/rust.yml/badge.svg)

askrs is a CLI tool to perform basic DNS lookups over a UDP socket connection for:

- A Records
- AAAA Records
- CNAME Records
- NS Records
- MX Records

As source DNS Servers the user has the choice between:

- google (8.8.8.8)
- cloudflare (1.1.1.1)
- quad 9 (9.9.9.9)

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
    cargo run -- -u google.com -s cloudflare --record NS -r
```

## Example

### Command
```
cargo run -- -u microsoft.com -s cloudflare --record A -r
```

### Printed Result
```
Socket bound to local address 0.0.0.0:0
ID: 0xc0af
Question count: 1
Resource Records: 5
Name Server Records: 0
Additional Records: 0
Domain: microsoft.com
Question Type: A
Answer Records: 5
NS Records: 0
Additional Records: 0
---------------------
	Name: microsoft.com
	Type: A
	Class: 1
	TTL: 1552
	Length: 4
	IPv4: 20.53.203.50
---------------------
---------------------
	Name: microsoft.com
	Type: A
	Class: 1
	TTL: 1552
	Length: 4
	IPv4: 20.81.111.85
---------------------
---------------------
	Name: microsoft.com
	Type: A
	Class: 1
	TTL: 1552
	Length: 4
	IPv4: 20.84.181.62
---------------------
---------------------
	Name: microsoft.com
	Type: A
	Class: 1
	TTL: 1552
	Length: 4
	IPv4: 20.103.85.33
---------------------
---------------------
	Name: microsoft.com
	Type: A
	Class: 1
	TTL: 1552
	Length: 4
	IPv4: 20.112.52.29
---------------------
```

## Roadmap

To continue learning about rust and improve the code base I am thinking about the following upcoming changes:

1. Implement Inverse DNS Queries
2. Implement persisting / caching response data with files (i.e JSON) or with SQLite
3. Refactor the DNS servers to have secondary IPs
4. Take a look into async Websockets with [tokio](https://tokio.rs/) to make multiple requests efficiently

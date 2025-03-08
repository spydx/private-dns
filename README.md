# Private DNS

Guid [Building a DNS Server in Rust: part 1](https://rust-trends.com/posts/building-a-dns-server-in-rust/)

## Part 1

### Server 

```shell
❯ cargo r
   Compiling private-dns v0.1.0 (/Users/kenneth.fossen/GIT_PRIV/private-dns)
     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
     Running `/Users/kenneth.fossen/GIT_PRIV/private-dns/target/debug/private-dns`
Hello, world!
I am Private DNS
> listening on port: 1053

Received query from 127.0.0.1:58204 with length 40 bytes

### DNS Query: ###
00000000: 5c 6b 01 20 00 01 00 00 00 00 00 01 03 77 77 77   \k. .........www
00000010: 04 6b 65 66 6f 02 6e 6f 00 00 01 00 01 00 00 29   .kefo.no.......)
00000020: 10 00 00 00 00 00 00 00                           ........

### HEADER ###

Header { id: 23659, qr: false, opcode: 0, aa: false, tc: false, rd: true, ra: false, z: 2, rcode: 0, qdcount: 1, ancount: 0, nscount: 0, arcount: 1 }

### QUESTION: ###
00000000: 03 77 77 77 04 6b 65 66 6f 02 6e 6f 00 00 01 00   .www.kefo.no....
00000010: 01 00 00 29 10 00 00 00 00 00 00 00               ...)........

Labels:
[Label("www")]
[Label("www"), Label("kefo")]
[Label("www"), Label("kefo"), Label("no")]

Question { name: [Label("www"), Label("kefo"), Label("no")], qtype: A, qclass: IN }
´´´

### Client

```shell
❯ dig @localhost -p 1053 www.kefo.no

; <<>> DiG 9.10.6 <<>> @localhost -p 1053 www.kefo.no
; (2 servers found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 23659
;; flags: qr rd; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0
;; WARNING: recursion requested but not available

;; QUESTION SECTION:
;www.kefo.no.                   IN      A

;; ANSWER SECTION:
www.kefo.no.            60      IN      A       76.76.21.21

;; Query time: 2 msec
;; SERVER: 127.0.0.1#1053(127.0.0.1)
;; WHEN: Sat Mar 08 21:35:34 CET 2025
;; MSG SIZE  rcvd: 56
```
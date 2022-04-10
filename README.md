# To test using docker

```bash
cd dns-proxy/

docker build --tag dns-test .
docker run --rm -it -p 1553:53/tcp -p 1553:53/udp \
    -e DOT_SERVER_ADDRESS=1.1.1.1:853 \
    -e DOT_SERVER_NAME=one.one.one.one dns-test
```

Use a client like [dnslookup](https://github.com/ameshkov/dnslookup/):

```bash
#UDP
dnslookup www.google.com localhost:1553

#TCP
dnslookup www.google.com tcp://localhost:1553
```

# To test without Docker

You need [rust](https://rustup.rs/) installed.

The code bellow was tested on Mac and Ubuntu 20.04.

```bash
DOT_SERVER_ADDRESS=1.1.1.1:853 DOT_SERVER_NAME=one.one.one.one PORT=1553 cargo run
```

Use a client like [dnslookup](https://github.com/ameshkov/dnslookup/):

```bash
dnslookup www.google.com tcp://localhost:1553
```

Note: the code was tested only with a subset of DNS protocols. Some things are not fully implemented (e.g. domain name compression). It should be lightweight enough to be used as a sidecar and have a basic caching features.

# Questions:

- **Security concerns?**:
  
  > The software was made using Rust to reduce problems related to memory and concurrency. Still, it needs more testing against [common attacks](https://www.cloudflare.com/learning/dns/dns-security/) and security-focused features, like DNSSEC, allow/block lists, better logging, etc. It's not ready for production yet and should be used with caution.

- **How to integrate in a distributed, microservices-oriented and containerized architecture?**
  
  > The application can be used as a sidecar pod due to its lower memory/cpu consumption and caching features taking to consideration the question above.

- **Other improvements?**
  
  > The code was made in less than 2 days, so it need lots of improvements: more tests, better documentation, tracing, other DNS RFCs like DNSSEC and benchmarks using different clients and backends (tested only with coredns and cloudflare dns) to search for bottle necks. More environment variables for fine tuning, like changing the size of the cache and allow/block lists.

## Resources used during the development

https://datatracker.ietf.org/doc/html/rfc1035

https://gist.github.com/andreif/6069838

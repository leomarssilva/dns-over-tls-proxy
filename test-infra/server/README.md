# Instructions

## Disclaimer
All ips and hostnames in the files `random-ip-list` and `hosts.txt` were **generated randomly** using [this website](https://commentpicker.com/ip-address-generator.php) and the following command, and none were previously tested for their existence:

```bash
./random-list.sh random-ip-list > hosts.txt
```

## Build the server image
This image is needed due to lack of support for aarch64 CPUs (e.g., Apple M1 chips and Raspberry PIs) by the official image.

```bash
podman build --tag coredns .
```

## Quick test

Run the server:
```bash
podman run --rm -it -p 5353:53/udp -p 5553:5553/tcp coredns:latest
```

Using [dnslookup](https://github.com/ameshkov/dnslookup):
```bash
VERIFY=0 dnslookup 11f7b2fd8b1272b15f6ea35e_113.testinternal.beta tls://ns1.testinternal.local:5553 127.0.0.1
```
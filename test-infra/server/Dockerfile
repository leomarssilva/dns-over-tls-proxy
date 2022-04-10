FROM golang:1.17 as builder
ARG coredns_version=v1.9.1

# build coredns
RUN git clone https://github.com/coredns/coredns /coredns &&\
 cd /coredns &&\
 git checkout ${coredns_version}
# compile only a few plugins
WORKDIR /coredns
ADD plugin.cfg ./plugin.cfg
RUN make

# build cert
# TODO: use secrets instead
RUN openssl req -x509 -newkey rsa:2048 -sha256 -days 30\
 -nodes -keyout /privatekey.pem\
 -out /servercert.pem\
 -subj '/CN=ns1.testinternal.local'


### 
FROM alpine:3.15.0
WORKDIR /
COPY --from=builder /privatekey.pem .
COPY --from=builder /servercert.pem .
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /coredns/coredns /coredns

# TODO: change image to use volumes and secrets
COPY hosts.txt .
COPY Corefile .

# Classic DNS
EXPOSE 53/udp
EXPOSE 53/tcp
# DoH
EXPOSE 443/tcp
# DoT
EXPOSE 5553/tcp
ENTRYPOINT [ "/coredns" ]
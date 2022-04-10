#!/bin/sh

export CA_PASS=12345
export SERVER_PASS=

if [ ! -e ca.key ]; then
    openssl genrsa -des3 -passout env:CA_PASS -out ca.key 4096
fi;

if [ ! -e ca.crt ]; then
    openssl req -x509 -new -nodes -key ca.key -passin env:CA_PASS -sha256 -days 3650 -out ca.crt \
        -subj "/C=BR/ST=RJ/L=RJ/O=Test/OU=IT/CN=testinternal.local/emailAddress=support@testinternal.local"
fi;

if [ ! -e server.key ]; then
    openssl genrsa -out server.key -passout env:SERVER_PASS 2048
fi;

if [ ! -e server.csr ]; then
    openssl req -new -key server.key -passin env:SERVER_PASS -out server.csr \
        -addext "certificatePolicies = 1.2.3.4" \
        -addext "subjectAltName = DNS:*.testinternal.local,DNS:testinternal.local,DNS:localhost" \
        -subj "/C=BR/ST=RJ/L=RJ/O=Test/OU=IT/CN=*.testinternal.local/emailAddress=support@testinternal.local"
fi;

if [ ! -e server.crt ]; then
    openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key \
        -passin env:CA_PASS -CAcreateserial -days 1500 -sha256 \
        -extfile extfile.txt -out server.crt
fi;

openssl x509 -in server.crt -text -noout
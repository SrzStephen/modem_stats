openssl genrsa -passout pass:foobarbaz -des3 -out ca.key 2048
openssl req -new -x509 -days 1826 -key ca.key -out ca.crt -passin pass:foobarbaz -subj "/C=AU/ST=CICD/L=Perth/O=CICD/OU=CICD/CN=127.0.0.1"
openssl genrsa -out server.key 2048
openssl req -new -out server.csr -key server.key -subj "/C=AU/ST=CICD/L=Perth/O=CICD/OU=CICD/CN=127.0.0.1"
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 360 -passin pass:foobarbaz
openssl genrsa -out client.key 2048
openssl req -new -out client.csr -key client.key -subj "/C=AU/ST=CICD/L=Perth/O=CICD/OU=CICD/CN=127.0.0.1"
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client.crt -days 360 -passin pass:foobarbaz

# How to regenerate self-signed certificates?

To regenerate self-signed certificates for timescaleDB you can do the following steps:

1. Create a certificate signing request (CSR) and a public/private key file

`
openssl req -new -nodes -text -out root.csr \
-keyout root.key -subj "/CN=root.yourdomain.com"
chmod og-rwx root.key
`,

replacing `dbhost.yourdomain.com` with the server's host name

2. Sign the request with the key to create a root certificate authority

`
openssl x509 -req -in root.csr -text -days 3650 \
-extfile /etc/ssl/openssl.cnf -extensions v3_ca \
-signkey root.key -out root.crt
`

3. Create a server certificate signed by the new root certificate authority

`
openssl req -new -nodes -text -out server.csr \
-keyout server.key -subj "/CN=dbhost.yourdomain.com"
chmod og-rwx server.key
`

`
openssl x509 -req -in server.csr -text -days 365 \
-CA root.crt -CAkey root.key -CAcreateserial \
-out server.crt
`

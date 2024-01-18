
# How to regenerate client self-signed certificates?

To regenerate self-signed certificates for client you can do the following steps:

`
openssl req -new -nodes -text -out client.csr \
-keyout client.key -subj "/CN=dbhost.yourdomain.com"
chmod og-rwx client.key
`

`
openssl x509 -req -in client.csr -text -days 365 \
-CA root.crt -CAkey root.key -CAcreateserial \
-out client.crt
`

replacing `dbhost.yourdomain.com` with the server's host name

Use `timescaleDB/certs/root.crt` to generate a certificate 

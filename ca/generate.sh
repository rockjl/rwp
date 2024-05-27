
# generate root secret key
openssl genrsa -out root-ca.key.pem 4096
# generate root certificate request file
openssl req -x509 -new -key root-ca.key.pem -out root-ca.pem -days 365 -SHA256
# generate root certificate
openssl x509 -outform der -in root-ca.pem -out root-ca.der

cat <<EOF >> localhost.cnf
authorityKeyIdentifier = keyid,issuer
basicConstraints = CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
EOF

openssl genrsa -out server.key.pem 4096
openssl req -out server.csr -key server.key.pem -new -days 365 -SHA256
openssl x509 -req -days 365 -SHA256 -in server.csr -CA root-ca.pem -CAkey root-ca.key.pem -CAcreateserial -extfile localhost.cnf -out cert.pem 
openssl x509 -outform der -in cert.pem -out cert.der
openssl pkcs12 -export -out identity.p12 -inkey server.key.pem -in cert.pem -passout pass:your_password
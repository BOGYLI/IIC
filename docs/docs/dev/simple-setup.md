
# Production
## Docker


# Development
## Docker
### PostgreSQL
    - cd server/docker-dev
    - docker-compose up -d

## Rust
### diesel.rs
    - apt-get install libpq-dev
    - cargo install diesel_cli --no-default-features --features postgres
    - cd server/docker-dev
    - docker-compose up -d
    - cd ..
    - diesel setup
    - diesel migration run

## Https (für Camera - Mebislogin)
### Zertifikate
    - cd server/certs
    - ```openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"```
    - `openssl rsa -in key.pem -out nopass.pem`
    - mv nopass.pem key.pem
FROM rust:1.55 as builder

WORKDIR /usr/src/hetznerdns

COPY . .

RUN apt update && apt install -y libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:buster-slim

RUN apt update && apt install -y libssl-dev ca-certificates cron && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-hetzner-dns /usr/local/bin/rust-hetzner-dns

WORKDIR /app
RUN echo "#!/bin/bash" \
    && echo "declare -p | grep -Ev 'BASHOPTS|BASH_VERSINFO|EUID|PPID|SHELLOPTS|UID' > /container.env" >> env-setup.sh \
    && echo "crontab -u root ./crontab" >> env-setup.sh \
    && echo "/usr/local/bin/rust-hetzner-dns" >> env-setup.sh \
    && echo "cron -f" >> env-setup.sh \
    && chmod +x env-setup.sh \
    && echo "SHELL=/bin/bash" > crontab \
    && echo "BASH_ENV=/container.env" >> crontab \
    && echo "0 * * * * root /usr/local/bin/rust-hetzner-dns" >> crontab
CMD ["bash", "./env-setup.sh"]

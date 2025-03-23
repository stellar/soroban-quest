# Soroban Quest <!-- omit in toc -->

![Stellar-Quest-email][series-5-img]

## Table of Contents <!-- omit in toc -->

- [Welcome](#welcome)
- [Feeling Lost?](#feeling-lost)
    - [Re-Visit the Pioneer Quest](#re-visit-the-pioneer-quest)
- [Join us on Discord](#join-us-on-discord)

## Welcome

Soroban Quest has been updated!

It's a faster, more curated, guided developer journey through Stellar Smart Contracts.

## Join us on Discord

In the [Stellar Developers Discord server][dev-discord], you will find a large,
active, and helpful community! We have recently announced a $100M Soroban
Adoption Fund, which SDF created to support the growth and development of the
Soroban ecosystem. We'll be sharing more about additional programs on the
Stellar Dev Discord in the not-too-distant future, so make sure to join today to
be the among the first to hear those announcements.

VPN

```
wg --help
```

Tunnel

CloudFlare [Tunnels]: https://github.com/cloudflare/cloudflared

```
cloudflared
cloudflared tunnel -hello-world
cloudflared tunnel --url http://localhost:8080
```

OAuth CLI

```
oauth2c https://discord.com \
  --client-id 1309255291048558632 \
  --client-secret  \
  --response-types code \
  --response-mode query \
  --grant-type authorization_code \
  --auth-method client_secret_basic \
  --scopes identity \
  --redirect-url https://fluffy-sniffle-5rj7v4pv97f4qwq-20241.app.github.dev:20241 \
  --callback-tls-cert https://raw.githubusercontent.com/cloudentity/oauth2c/master/data/cert.pem \
  --callback-tls-key https://raw.githubusercontent.com/cloudentity/oauth2c/master/data/key.pem
```

DNote

```
https://github.com/dnote/dnote/wiki/Dnote-CLI#commands
```

Launch Codespace from Github CLI

Install github and setup auth token for API
```
brew install gh

gh codespace list

gh codespace create --repo anataliocs/soroban-quest -b migrate-to-dev-containers --status --web -l WestUs2

```

[dev-discord]: https://discord.gg/stellardev

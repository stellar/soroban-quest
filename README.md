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

## Feeling Lost?

There's a lot going on here! If you're feeling confused, or you're not sure how
this whole thing is supposed to work, don't fret. We have a couple options for
you:

### Re-Visit the Pioneer Quest

please checkout our [pioneer quest][pioneer]. It covers the basic structure of
this repository, all the tools you'll need, and the process of getting
everything working together.

It's an important resource to keep handy during all these live quests. Some of
the most important bits you'll need to know from it:

- Understanding what a "Gitpod workspace" even is, and how we've set it up for
  you to successfully complete these quests.
- Using the `sq` CLI to login to your Stellar Quest account, play quests, verify
  them, and more.
- Interacting with a Soroban sandbox, as well as the Futurenet, from within this
  gitpod workspace.

If you feel lost on any of that, you could probably use a refresher. Go ahead,
[be a pioneer][pioneer] once more. We don't judge!

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

```
cloudflared
cloudflared tunnel -hello-world
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

[series-5-img]: https://user-images.githubusercontent.com/4383610/200077219-de8e1f20-9878-4705-bec6-ced9a3904694.jpg

[dev-discord]: https://discord.gg/stellardev

# Helpful Python Scripts

If you're going to use any of these, make sure you install the `soroban` branch
of the Python `stellar_sdk` first, as described below. Also please note these
scripts are **NOT REQUIRED** for this Quest, but we found them to be
_incredibly_ helpful while developing it. So, we wanted to pass them along for
your benefit and future use, learnings, and experiments.

Enjoy!

```bash
pip install git+https://github.com/StellarCN/py-stellar-base.git@soroban
```

The most recent preview release of Soroban has introduced some breaking changes
that have not yet been fixed in the Python sdk. In some cases, using the
`soroban-auth-next` branch could resolve errors. Soroban compatibility with any
SDK is still a bit hit-or-miss, and they remain works-in-progress.

```bash
pip install git+https://github.com/StellarCN/py-stellar-base.git@soroban-auth-next
```

Massive props to [Jun Luo (@overcat)](https://github.com/overcat) for his
incredible work on the Python SDK.

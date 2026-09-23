# PA-4 EET Tweaks installation evidence

On 2026-09-23 IEPM fetched the exact official v1.12 tag ZIP, SHA-256
`c43b8a070755a52ef8eff8c3865be203545a01b4a75e0e6474ea1d5aa975880c`,
and installed the [generic total-XP-cap fixture](../examples/bg2ee-eet-tweaks/modpack.yaml)
on clean disposable BG2EE 2.6.6. `review-tp2` matched all 65 declared
numeric selectors. Seven custom-value choices invoke `ACTION_READLN` and
are classified unavailable until IEPM records portable typed answers.

- Registry revision `7c1d6ee`, WeiDU 25100, clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `8941a8c5cbd35484030ac87da3ba5cdfcd11a1dd1cfe4e3375c06e20941d43db`;
  sealed target fingerprint
  `32a5fef05d647bd95bfa4e7fd14b120d9f8cd1a5d415e5fd52347627d9c34e46`.
- WeiDU.log records `#2001` (total XP cap 8,000,000). Stderr was empty,
  with no warning or skipped-component marker.

This test does **not** exercise an EET transformation or any EET-only
choice. The [scoped assertion](../evidence/pa4-bg2ee-eet-tweaks.json) is
**Untested** under public status gates pending EET fixture, other selectors,
menu, and gameplay smoke. Raw logs and the sealed build remain local.

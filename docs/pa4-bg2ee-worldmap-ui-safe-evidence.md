# PA-4 Worldmap independent UI selector evidence

On 2026-09-23 IEPM fetched the exact official v13.1.1 tag ZIP,
290,473,374 bytes, SHA-256
`3890046ab83ae9316299087345cc7011c484fbd343db3958862004bc053314a7`.
`review-tp2` matches all six LABEL-bearing selectors. The main Worldmap
component can request interactive map-size/travel answers and remains
unavailable until those answers are portable, typed build inputs. The two
larger-map UI choices are independent TP2 subcomponents without that prompt.

The [safe fixture](../examples/bg2ee-worldmap-ui-safe/modpack.yaml)
installed BG2-style larger worldmap UI selector `#4` in a clean disposable
BG2EE 2.6.6 copy and sealed. WeiDU.log records `#4`; stderr was empty and
stdout had no warning or skipped-component markers.

- Registry revision `9c3b944`, WeiDU 25100, clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `35b417e7ae57a237093bf875f74f58c9c0f338f79887849dada357bc62e88839`;
  sealed target fingerprint
  `f060374b7669a7a6cc76374ad8cfe9903f8a04afc47ad6958f0f35b4ea4d445e`.

This does **not** prove IEPM can unattended-install main Worldmap, nor that
the UI appears correctly in game. The [scoped assertion](../evidence/pa4-bg2ee-worldmap-ui-safe.json)
remains **Untested** pending visual smoke. Raw logs and sealed build remain
local.

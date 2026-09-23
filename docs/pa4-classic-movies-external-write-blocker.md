# PA-4 BG:EE Classic Movies external-write blocker

The exact BG:EE Classic Movies V2.4.1 author-tag ZIP was acquired and hashed
in the [PA-4 tagged-artifact cohort](../registry/cohorts/pa4-tagged-artifacts.json):
SHA-256 `c740bce60ee33bed5652d51652015a2ad3ca5b199480a49f804dbee35f65cc58`.
Its TP2 mechanically declares eight LABEL-bearing selectors, including two
alternatives for restoring movies and four alternatives for startup movies.
This is not yet an executable IEPM release record.

The TP2's `patch_Baldur_config` function copies to
`%USER_DIRECTORY%/Baldur.lua` or `Baldur.ini`, creating an INI there if
neither exists. Both restore-movies choices call this function. The in-game
movie-menu selector independently copies to the same user config path. The
startup-movie alternatives call the function when a logo movie is found.
These writes escape the disposable game workspace and can change the user's
active game settings even though IEPM later seals or discards its build.
The main A5 invariant is therefore not demonstrated for this exact release.
We did not run a disposable install merely to see whether the external write
would occur.

Before promotion, isolate WeiDU's user directory into a per-build sandbox
or otherwise prove that the selected component cannot write outside the
workspace, then test the relevant choices and inspect both the game root and
external-write boundary. Avoid claiming the mod itself is incompatible;
the issue is IEPM's currently uncontained installer side effect. The 586 MB
exact ZIP and eight observed selectors remain discoverable through the
cohort record while the registry release stays discovery-only.

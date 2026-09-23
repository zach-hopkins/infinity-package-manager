# PA-4 author-hosted Weasel Mods BG2EE install batch

On 2026-09-23, eight exact SHA-256-pinned archives from the author's
[download site](https://downloads.weaselmods.net/) were installed separately
into fresh disposable BG2EE 2.6.6 workspaces with WeiDU 25100. Each selected
component was recorded in WeiDU.log and each build sealed at registry revision
`3019d45`. All eight stderr logs were empty; the selected install stdout logs
had no warning, error, skipped, or not-installed markers. These are bounded
installer receipts, not menu, gameplay, quest, or EET verification.

| Package | Archive SHA-256 | Lock SHA-256 | Sealed fingerprint |
| --- | --- | --- | --- |
| Tales of the Deep Gardens 12.97 | `a9f6c31efd7b134aff8f8a5a990a2d4363f04000b953395fbb702e7f5e23db5a` | `d5d903ed6bdecc3c763396245aa5583e29cc2104ed1f83eec6e102c640179188` | `a08f38aa590f6ba11f181d4b930273818808f787b0dbbfd39d796c57e659eb4e` |
| The White Queen v7.4.2 | `4a846e0bfc15cfa714c325c7350faa64e4fd4b92b0a23c0037e109bf206fdd3f` | `43a07c2555b58f1c7d46463f465449faccde4529a074d710f184e82ed691560` | `8410675999ddd2aca08cf07c5e570eeb18b9c4ce6821013e9b7f5a69286f6c80` |
| Eilistraee's Song v7.5.2 | `63c4e6562c6db724c74057332f034e94344d3754839f1cd45b4da5ea6920609e` | `1522d8af53a0039d204e8b36c27ca6f5a5da1862df930c252ec8427b764b0661` | `0d03c05ae5eac528485bf6187379e9e9cad91152e207c32e25f945399d01c3ba` |
| Southern Edge v6.0 | `9fe060abad6ccc92c004e1da7e2d8aab6170350f3441479a99079d8fbfab71a8` | `f6d134c8a5e25b505d783208c02b2fadbed82620c0cd6d99b6173399f1ac3644` | `aa6063fd9effe3d2d2ca4f38979e8d927477debb3e54e731cd691bae1e3d177a` |
| The Ooze's Lounge v3.1 | `2ce6ed79010ae87a474a3f161d91f321dc359543ee6f26a420f287be0e5d0c65` | `fabc0f9d3901c487aabb8d7ed4460ae97786d260591c7bd3ac49a3e55945dd1e` | `45e3df466ef15d292d28917a8f6fe397135c076482725878cb3e517c1e53f922` |
| The Tangled Oak Isle v4.7 | `e67587d0eb805a1b0eeaa0c25f5526a79f6fd737389db014f66ed8ca810aa7e8` | `5a94d1777ba55e5ee07455ce8aaffdb7417f524e5afe65636082e9a03428b11f` | `e5576aa7574738a00a2c57c41645ff174de48d1fcf60787cf0f92eecb45e313b` |
| Bridge's Block v2.02 | `79d9a0fa619cf2bd5336272e349ca5ed5978d60ff7f0b4430cc0697f8543375f` | `559d5d21102e49fe9e4169436d377605f0434ddecb64630068f6f7b901004189` | `cbdd9d6b28de9784ca7a2b9ad032fcd57a37c8fe6874fcffa3a9f8885d5fc259` |
| Alabaster Sands v2.0 | `dd7d2d5b6d660f6bb392c7241270516c5f664b50fd478a96d03dda3a1edeafbc` | `dc49de0a125dcb2d6c317613f242f04a079eae1b497705208c8b98297e9e1878` | `bdc66bb03171d54bb559932344b31e9a7af5ee6869854e1ced6d3715e4c1c4b8` |

All eight assertions remain **Untested** under the support policy because
menu and gameplay smoke were not performed. For the six two-choice packages,
only the no-save-patching choice was exercised; the existing-save-patching
alternative remains unavailable until IEPM contains possible writes outside
the disposable game workspace. The other two packages have one live choice.
Raw logs and sealed game trees remain in the local IEPM store, not Git.

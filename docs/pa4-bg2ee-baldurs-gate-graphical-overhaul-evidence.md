# PA-4 Baldur's Gate Graphical Overhaul installation evidence

On 2026-09-23, IEPM installed the exact official v3.6 tag ZIP using the
[BG2EE fixture](../examples/bg2ee-baldurs-gate-graphical-overhaul/modpack.yaml)
in a fresh disposable BG2EE 2.6.6 workspace. The source archive SHA-256 was
`4bb821d971336364548019cf8423c5476cc914934ec109f4003e5aab9a5870a2`
(1,461,539,779 bytes). `review-tp2` matched all four live selectors with
none unmapped. It reported language-mapping drift because the TP2 declares no
LANGUAGE; the actual WeiDU 251 run accepted English index 0 and installed.

- Registry revision `6f1778f`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `eb6b0d579c25ac21034e1d4a9baadaee5882ea9984513e0e729a2bd766bb232a`;
  final sealed fingerprint
  `34210c258b2a768027cd9816132830964279216e68acea13a7835473f273fddd`.
- WeiDU.log records core selector `#0`; stderr was empty. Stdout has no
  warning or skipped-component marker.

The official ZIP contains no BGGO-Android directory. Two platform-choice
selectors explicitly require that directory; the core auto-selects Windows
files without them. Alternate Graphics was not selected. The
[scoped assertion](../evidence/pa4-bg2ee-baldurs-gate-graphical-overhaul.json)
remains **Untested** pending menu, visual, optional-selector, and EET smoke.
Raw logs and the sealed game tree remain in the local IEPM store, not Git.

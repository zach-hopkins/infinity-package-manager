# PA-4 Ajantis BG1 Expansion BGEE fixture

The first clean disposable BGEE 2.6.6 run of Ajantis BG1 Expansion v22
stopped before installing component 0. Its TP2 detected the source game's
unmerged SoD DLC and explicitly required DLC Merger. This is a known
prerequisite for this exact source layout, not evidence that Ajantis is
generally incompatible with BGEE. The failed workspace is not reusable.

The revised [fixture](../examples/bgee-ajantis-bg1-expansion/modpack.yaml)
selects DLC Merger's `merge-sod` before Ajantis's main component. The outcome
of that clean retry will be recorded here. Optional shield art and SoD NPC
crossmod content remain Untested.

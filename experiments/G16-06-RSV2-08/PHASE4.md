# G16-06-RSV2-08 Phase 4

Status: complete; 8/8 complete. CPUs12,13; nice=19; CARGO_BUILD_JOBS=2; skyline plate backend. No physical source, threshold, load or support constant changed.

Outcome: **0/8 joint passes**; all40 spine observations are below1 (range0.371928145–0.550197106). Native geometry passes0/8, GCI admission1/8, export6/8, side-A4 8/8. No gate is relaxed.

Eight cases use the two exact Phase-3 named parents, each at alpha=.25/.50/.75/1 toward CP29=2, CP30=1.73008 and blade_drop_deg=0. Both parent drops are already zero; only CP29/30 change numerically. CP06..08 and all other genes remain exactly parental. CP02..05/CP23..25 are already3. No spatial remapping of literal CP indices or Q7 section/solver import is performed.

Assignment discrepancy: local_04 S1 is **.500**, while the assignment parenthetical range starts at .502. The exact named admissible-GCI parent is preserved (.052/.500/fill.65); placement_05 uses .05/.525/fill.70. All windows remain width6. The external Q7 report uses a different section law and explicit faer; its reported HM spine margins are contextual and are not asserted to transfer to this study.

| Case | Alpha | Cert spine | Strict spine | Cert lat N | Strict lat N | Cert fatigue | Strict fatigue | Cert seq | Strict seq | GCI | Export | A4 | Joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| placement_05_a0.25 | 0.25 | 0.477871 | 0.477185 | 395.036239 | 406.424552 | 1.103465 | 1.135276 | 1.176038 | 1.264459 | False | True | True | False |
| placement_05_a0.50 | 0.5 | 0.460050 | 0.500294 | 374.103397 | 370.823270 | 1.044992 | 1.035830 | 1.312788 | 1.340907 | False | True | True | False |
| placement_05_a0.75 | 0.75 | 0.479905 | 0.454040 | 359.125536 | 407.022889 | 1.003154 | 1.136947 | 1.364724 | 1.146176 | True | False | True | False |
| placement_05_a1.00 | 1.0 | 0.458775 | 0.487667 | 399.369526 | 352.414310 | 1.115569 | 0.984408 | 1.190983 | 1.281511 | False | True | True | False |
| local_04_a0.25 | 0.25 | 0.529732 | 0.477594 | 383.039313 | 382.957014 | 1.069953 | 1.069723 | 1.273490 | 1.342619 | False | True | True | False |
| local_04_a0.50 | 0.5 | 0.523523 | 0.471478 | 372.030774 | 366.693209 | 1.039203 | 1.024293 | 1.344757 | 1.356011 | False | True | True | False |
| local_04_a0.75 | 0.75 | 0.472473 | 0.503531 | 382.258983 | 401.276226 | 1.067774 | 1.120895 | 1.232299 | 1.163714 | False | False | True | False |
| local_04_a1.00 | 1.0 | 0.539801 | 0.467878 | 411.002593 | 400.740172 | 1.148064 | 1.119398 | 1.157784 | 1.126992 | False | True | True | False |

Joint scope requires geometry, HM spine>=1, lateral>=358N, fatigue>=1, sequence>=1.20 and hunter>=1.377 at certification/strict, native GCI admission, export and side-A4. Every one of the five mesh observations is retained; coarse failures are never waived or substituted for certification. Full RS-v2/independent-solid/manufacturing qualification remains false.

Raw inputs, full measurements, FEM diagnostics and meshes are retained under target/G16-06-RSV2-08/phase4/. No early numerical prefilter is applied. All five meshes, actual export and side-A4 are evaluated even after an earlier failure.

Recorded invocation using retained lane binaries (the runner refuses to overwrite existing progress evidence):
```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase4.py
```

## Full Phase-4 measurements

All 40 requested mesh observations and all eight export/support evaluations completed. Exact assigned interpolations, preserved hand/other genes, canonical inputs, source/material identity, raw hashes and fatigue identities pass. The unchanged native requirements consistency test passes (1/1). `phase4_candidates/` contains the eight exact evaluated CSVs.

| Parent reference (Phase 3) | Cert HM spine | Strict HM spine | Cert lat N | Strict lat N | Cert seq | Strict seq |
| --- | --- | --- | --- | --- | --- | --- |
| placement_05 | 0.461001599 | 0.466847918 | 399.115859478 | 386.841161381 | 1.231105586 | 1.325637584 |
| local_04 | 0.498241747 | 0.489095558 | 406.556125802 | 410.461355420 | 1.211514978 | 1.249353638 |

| Case | Area mm² | HM spine SF | Lateral N | Fatigue SF | Sequence | Spine peak x,y |
| --- | --- | --- | --- | --- | --- | --- |
| placement_05_a0.25 | 4.0 | 0.388080962 | 383.663110630 | 1.071695778 | 1.270623200 | [119.75191761350759, -13.774233815727921] |
| placement_05_a0.25 | 1.0 | 0.380223199 | 369.522623754 | 1.032196802 | 1.343433941 | [119.02980889618739, -13.574466399245757] |
| placement_05_a0.25 | 0.25 | 0.502303575 | 395.036239476 | 1.103464624 | 1.260456983 | [114.47393382380416, -12.364478050949028] |
| placement_05_a0.25 | 0.125 | 0.477870828 | 414.784156354 | 1.158626975 | 1.176037700 | [114.33010248934059, -12.304152367230408] |
| placement_05_a0.25 | 0.0625 | 0.477185091 | 406.424551681 | 1.135275881 | 1.264458764 | [114.40201815657238, -12.334315209089718] |
| placement_05_a0.50 | 4.0 | 0.399120899 | 349.832284808 | 0.977195285 | 1.382654403 | [114.62210461206013, -12.425848093971682] |
| placement_05_a0.50 | 1.0 | 0.419558967 | 402.845628155 | 1.125278785 | 1.088220848 | [119.75587316012732, -13.77515591891429] |
| placement_05_a0.50 | 0.25 | 0.460050415 | 374.103396504 | 1.044992389 | 1.357619942 | [114.33445496822509, -12.305253332152212] |
| placement_05_a0.50 | 0.125 | 0.517779909 | 387.245490266 | 1.081702529 | 1.312788387 | [114.62182690458536, -12.292058259351304] |
| placement_05_a0.50 | 0.0625 | 0.500293668 | 370.823270480 | 1.035829931 | 1.340906867 | [119.33220713547792, -13.621063381208705] |
| placement_05_a0.75 | 4.0 | 0.437345352 | 322.956575002 | 0.902122691 | 1.267807711 | [119.00045776652541, -13.55525989307175] |
| placement_05_a0.75 | 1.0 | 0.523432666 | 389.110810430 | 1.086912974 | 1.326259062 | [114.53587268212296, -12.402693291133074] |
| placement_05_a0.75 | 0.25 | 0.491277041 | 361.671150273 | 1.010265084 | 1.364723792 | [114.39187721205298, -12.342710242709332] |
| placement_05_a0.75 | 0.125 | 0.479904944 | 359.125535855 | 1.003154356 | 1.409783269 | [114.247881741983, -12.28272719428559] |
| placement_05_a0.75 | 0.0625 | 0.454040492 | 407.022889207 | 1.136947233 | 1.146176001 | [119.34724387833508, -13.613601207417084] |
| placement_05_a1.00 | 4.0 | 0.468076958 | 404.299009453 | 1.129338551 | 1.111341005 | [119.75455245533503, -13.774848003314599] |
| placement_05_a1.00 | 1.0 | 0.479191324 | 366.275859409 | 1.023127534 | 1.260794986 | [119.43037067733994, -13.635871980284097] |
| placement_05_a1.00 | 0.25 | 0.472318733 | 403.912342233 | 1.128258464 | 1.190983049 | [114.47682873182714, -12.365192554384674] |
| placement_05_a1.00 | 0.125 | 0.458775085 | 399.369526241 | 1.115568902 | 1.194104886 | [119.43037067733994, -13.635871980284097] |
| placement_05_a1.00 | 0.0625 | 0.487666554 | 352.414310100 | 0.984407721 | 1.281511067 | [114.40491523364335, -12.33503913956975] |
| local_04_a0.25 | 4.0 | 0.474036791 | 397.234295398 | 1.109604508 | 1.225037879 | [114.61776515826773, -12.424803734667648] |
| local_04_a0.25 | 1.0 | 0.510884462 | 408.871862654 | 1.142112017 | 1.214144393 | [114.61776515826773, -12.424803734667648] |
| local_04_a0.25 | 0.25 | 0.534222404 | 387.635905489 | 1.082793085 | 1.284520445 | [114.47393382380416, -12.364478050949028] |
| local_04_a0.25 | 0.125 | 0.529732188 | 383.039313197 | 1.069953309 | 1.273490332 | [114.05605623358463, -12.111792503754888] |
| local_04_a0.25 | 0.0625 | 0.477593749 | 382.957013664 | 1.069723420 | 1.342618689 | [119.42773891792078, -13.635185847881923] |
| local_04_a0.50 | 4.0 | 0.463546644 | 377.366864175 | 1.054108315 | 1.281768636 | [119.75587316012732, -13.77515591891429] |
| local_04_a0.50 | 1.0 | 0.450951500 | 416.086155592 | 1.162263882 | 1.053591713 | [119.03375902754757, -13.575605674829527] |
| local_04_a0.50 | 0.25 | 0.533285372 | 377.682076956 | 1.054988807 | 1.344756994 | [114.47827979014261, -12.365550713061946] |
| local_04_a0.50 | 0.125 | 0.523523279 | 372.030774311 | 1.039202882 | 1.366331639 | [114.21513347412568, -12.284728121569072] |
| local_04_a0.50 | 0.0625 | 0.471477505 | 366.693208839 | 1.024293326 | 1.356011338 | [119.43168983812137, -13.636215950001764] |
| local_04_a0.75 | 4.0 | 0.460615331 | 393.212584509 | 1.098370563 | 1.281013774 | [119.67164625394841, -13.751994373558524] |
| local_04_a0.75 | 1.0 | 0.540238016 | 409.201198837 | 1.143031960 | 1.261144282 | [128.46447818896323, -16.208738935892075] |
| local_04_a0.75 | 0.25 | 0.472473164 | 400.536988387 | 1.118830003 | 1.232298546 | [114.247881741983, -12.28272719428559] |
| local_04_a0.75 | 0.125 | 0.523032495 | 382.258983025 | 1.067773593 | 1.323490145 | [114.34387872202964, -12.32271589323475] |
| local_04_a0.75 | 0.0625 | 0.503530982 | 401.276226417 | 1.120894936 | 1.163713874 | [114.31987947701799, -12.312718718497461] |
| local_04_a1.00 | 4.0 | 0.371928145 | 331.034374433 | 0.924686611 | 1.218055085 | [119.75455245533503, -13.774848003314599] |
| local_04_a1.00 | 1.0 | 0.380326104 | 425.173695977 | 1.187648336 | 1.086141541 | [114.62065572819469, -12.42549938401452] |
| local_04_a1.00 | 0.25 | 0.539800791 | 415.501844460 | 1.160631711 | 1.157784202 | [119.59246156633748, -13.705359991799348] |
| local_04_a1.00 | 0.125 | 0.550197106 | 411.002592717 | 1.148063838 | 1.160458669 | [119.29772716163528, -13.615656405663273] |
| local_04_a1.00 | 0.0625 | 0.467877844 | 400.740172140 | 1.119397562 | 1.126991764 | [119.43037067733994, -13.635871980284097] |

| Case | GCI fit / p | Max half-angle ° | Max apex mm | Point | Census | A4 eigenvalue | A4 seq / cert |
| --- | --- | --- | --- | --- | --- | --- | --- |
| placement_05_a0.25 | oscillatory / None | 23.802165998 | 0.195140030 | True | [0, 0, 1] | 8.998130452 | 2.639226076 / 1.176037700 |
| placement_05_a0.50 | oscillatory / None | 23.881639369 | 0.195136539 | True | [0, 0, 1] | 9.576270203 | 3.890546450 / 1.312788387 |
| placement_05_a0.75 | admissible_estimate / 0.6037002015686626 | 29.031136547 | 0.195138705 | True | [0, 0, 1] | 7.939500812 | 3.455084865 / 1.364723792 |
| placement_05_a1.00 | oscillatory / None | 23.829447306 | 0.195144137 | True | [0, 0, 1] | 8.245556912 | 4.556016552 / 1.190983049 |
| local_04_a0.25 | oscillatory / None | 23.802165998 | 0.195140030 | True | [0, 0, 1] | 8.998527119 | 3.195754543 / 1.273490332 |
| local_04_a0.50 | oscillatory / None | 23.881639369 | 0.195136539 | True | [0, 0, 1] | 8.295410102 | 2.066498117 / 1.344756994 |
| local_04_a0.75 | nonpositive_order / -0.5378029889246035 | 29.031136547 | 0.195138705 | True | [0, 0, 1] | 9.090130803 | 3.199526117 / 1.232298546 |
| local_04_a1.00 | oscillatory / None | 23.829447306 | 0.195144137 | True | [0, 0, 1] | 9.086325425 | 3.417631375 / 1.157784202 |

| Case | Geometry legal | Native geometry failures | Cert HM twist SF | Strict HM twist SF |
| --- | --- | --- | --- | --- |
| placement_05_a0.25 | False | spine not a continuous convex crescent (0.89 < C.4 0.94-0.03) | 0.300205584 | 0.300272242 |
| placement_05_a0.50 | False | spine not a continuous convex crescent (0.84 < C.4 0.94-0.03) | 0.268129194 | 0.300552779 |
| placement_05_a0.75 | False | spine not a continuous convex crescent (0.82 < C.4 0.94-0.03) | 0.272087678 | 0.312460410 |
| placement_05_a1.00 | False | spine not a continuous convex crescent (0.82 < C.4 0.94-0.03) | 0.266431195 | 0.305079113 |
| local_04_a0.25 | False | spine not a continuous convex crescent (0.89 < C.4 0.94-0.03) | 0.302929358 | 0.298780889 |
| local_04_a0.50 | False | spine not a continuous convex crescent (0.84 < C.4 0.94-0.03) | 0.281774973 | 0.300552780 |
| local_04_a0.75 | False | spine not a continuous convex crescent (0.82 < C.4 0.94-0.03) | 0.290899974 | 0.308535077 |
| local_04_a1.00 | False | spine not a continuous convex crescent (0.82 < C.4 0.94-0.03) | 0.292624623 | 0.317385468 |

Pass counts among eight cases: {'geometry_legal': 0, 'gci_admitted': 1, 'export_pass': 6, 'side_a4_pass': 8, 'fine_target_pass': 0, 'all_five_target_pass': 0, 'all_five_sequence_monotonic': 0, 'scoped_joint_pass': 0}.

Scoped joint passes: []. This finite eight-case study does not establish global infeasibility or an optimum. Full RS-v2/manufacturing qualification remains false. Human-max twist, independent 3D/cross-solver checks and the inherited collar/flat-stock inconsistency are not waived.

Native GCI admission is a checked three-level estimate, with asymptotic validation still not established. Every coarse failure and all-five sequence monotonicity are retained separately. The raw native GCI/A4 adapter retains its Phase-2 label; exact Phase-4 inputs, meshes and content hashes bind each evaluation. These are law+config+genome prototype results, not production CSV-only exporter qualification.

Audit after a completed run:
```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/audit_phase4.py
```

Delivery: WIP commit with all required p4 Agent-* trailers was attempted; staging failed because shared Git index.lock is read-only. Snapshot HEAD `5a7f01aff78ea51af9712481b3aa48aa566ffee7` contains complete matching Phase-4 data=False; no agent-created commit is claimed. Complete Phase-4 report, receipt, eight CSVs and audit remain local for captain checkpointing. Source/raw archives and manifest are retained under target/G16-06-RSV2-08/. Lane COMMS HANDOFF is recorded under shared flock; shared main COMMS requires captain relay. No PR/push/stacking/deletes; await captain's next milestone.

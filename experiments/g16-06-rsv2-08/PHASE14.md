# G16-06-RSV2-08 Phase 14

Complete:16/16 assigned candidates and48/48 native mesh observations. Geometry passes=0/16; aesthetic passes=16/16; both=0/16. **The requested confirmation of16/16 passing both gates is contradicted by native measurement.** Every candidate fails the unchanged holder-envelope gate (measured oriented boxes exceed200x100mm); none is qualified. All native rejection strings are retained below.

Candidates are generated mechanically from native `Genome::c4_seed(6.0)` (Ti-6Al-4V). Fixed genes: EDGE_TH=.192mm,TAPER_X0=148mm,T_TIP=4.4mm,RING_WALL=6.20mm,P1_ON=1,POCKET_D=.25mm. The native P1 canonicalization additionally sets the seed FLUTE gene78 from.6 to0; no other unassigned seed gene changes. Raw native C4 seed, before-clamp genes, canonical P1 seed, material, full128-gene offspring CSVs and native control points are recorded. Offspring change only CP36=.65*dn37,CP37=dn37,CP38=.80*dn37 and EA_IN=EA_SP. Positive CP genes are native outward-normal distances, not Cartesian coordinates. dn37=.30/.40/.50/.60mm crossed with EA=18.0/18.2/18.4/18.6deg. Every offspring must already equal native clamp; no repair or design alteration.

Mesh and observable definitions were unspecified in the assignment. A clarification was submitted and the stated native defaults were used: coarse/medium/fine maximum triangle areas4/1/.25mm², h=2/1/.5mm, uniform refinement r=2. Each uses unbiased production `fem::report_fields` and production `export::thickness_at`; geometry, loads, supports, exclusions, material and gates are unchanged. These levels support the native GCI method and do not supply the prior .125mm² certification or .0625mm² strict observations.

`F_lat`=native Fem.lat_rob (joint-adjusted lateral tip-load capacity in N). `F_ax`=native Fem.hm_tip_inplane_n, the conservative envelope of chord-axial and transverse in-plane tip-load capacities in N; pure axial-only capacity is not separately exposed by this native API. `SF_fat`=native Fem.fat_sf, Goodman fatigue under180N cyclic lateral load. Tip GCI applies to F_lat, a tip-loaded whole-plate capacity outside the native4mm exclusion, not a local tip-zone-only or solid3D observable. Torsional GCI applies to Fem.twist_joint_nmm, native global tip-loaded chord torsion with native section correction. The separate native annular tipzone SF remains recorded in the receipt.

Both estimates call unmodified native `gci::compute_gci`, relative_gci.v2, Fs=1.25, fine-to-coarse values and distinct exact mesh/thickness SHA256 identities. All native fit flags, rejection reasons, order p, extrapolated value and relative fine GCI are retained. Python reference arithmetic independently recomputes both estimates. `None` GCI means no admitted numerical estimate; asymptotic_validation=not_established for this three-level study. Admission is not qualification and no new GCI acceptance threshold is invented.

The assignment cites another lane's EA<=18.6deg/26deg solid Gate S result. This lane retains its native25deg law limit and.20mm apex limit unchanged: all assigned genes pass that law predicate. No exported solid is generated here; native law bounds alone cannot establish 3D S3 or full actual-export Gate S. All16 native geometry failures are binding. Existing engineering thresholds remain358N lateral, fatigue>=1, sequence1.20 certification/1.25 strict and native torsion/human-max floors. The receipt additionally reports each all-three-study-level comparison using the stricter1.25 sequence floor, explicitly as observations rather than certification.

Execution: `nice -n 10 taskset -c 12,13`, effective process nice=19 (host inherited nice10), CARGO_BUILD_JOBS=2, two native workers, faer solver, CARGO_INCREMENTAL=0 and lane-local target. Native FEM/thickness/GCI sources unchanged. Full qualification=false. Actual-export3D Gate S, finer certification/strict levels, A4 and remaining independent3D gates remain unverified for these offspring. No staging, commit, push, PR or stacking.

Outcome counts: {"aesthetic_pass": 16, "fatigue": 0, "fem_complete": 16, "geometry_and_aesthetic_pass": 0, "geometry_legal": 0, "human_max_twist": 0, "input_canonical": 16, "lateral": 0, "native_gate_s_law_pass": 16, "native_utility_twist": 0, "reported_targets_pass": 0, "sequence": 16, "tip_gci_admitted": 4, "tip_loaded_twist": 0, "torsional_gci_admitted": 4}.

| Candidate | Geometry pass | Aesthetic pass | Fine F_lat N | Fine F_ax envelope N | Fine SF_fat | Tip GCI fit | Tip relative GCI | Torsional GCI fit | Torsional relative GCI | Reported conjunction |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| dn0.30_ea18.0 | False | True | 107.435183647 | 290.716684341 | 0.300101389822 | admissible_estimate | 0.181101261402 | nonpositive_order | None | False |
| dn0.30_ea18.2 | False | True | 109.514539472 | 292.778477913 | 0.305909706536 | admissible_estimate | 0.177488271286 | nonpositive_order | None | False |
| dn0.30_ea18.4 | False | True | 111.618988254 | 294.843751274 | 0.311788116037 | admissible_estimate | 0.174185397014 | nonpositive_order | None | False |
| dn0.30_ea18.6 | False | True | 113.748738119 | 296.912582881 | 0.317737199689 | admissible_estimate | 0.171155984699 | nonpositive_order | None | False |
| dn0.40_ea18.0 | False | True | 109.088028749 | 302.435785245 | 0.304718323451 | oscillatory | None | oscillatory | None | False |
| dn0.40_ea18.2 | False | True | 111.173272779 | 304.614270581 | 0.310543087837 | oscillatory | None | oscillatory | None | False |
| dn0.40_ea18.4 | False | True | 113.28345495 | 306.79662343 | 0.316437512555 | oscillatory | None | oscillatory | None | False |
| dn0.40_ea18.6 | False | True | 115.41878236 | 308.98292535 | 0.322402176102 | oscillatory | None | oscillatory | None | False |
| dn0.50_ea18.0 | False | True | 116.639309934 | 337.591504585 | 0.32581150635 | nonpositive_order | None | admissible_estimate | 0.00262361961632 | False |
| dn0.50_ea18.2 | False | True | 118.915012457 | 340.126015085 | 0.332168283215 | nonpositive_order | None | admissible_estimate | 0.00272735319602 | False |
| dn0.50_ea18.4 | False | True | 121.218321361 | 342.665348825 | 0.338602173678 | nonpositive_order | None | admissible_estimate | 0.00283082103827 | False |
| dn0.50_ea18.6 | False | True | 123.549465494 | 345.209597003 | 0.345113816981 | nonpositive_order | None | admissible_estimate | 0.00293394577249 | False |
| dn0.60_ea18.0 | False | True | 109.905434445 | 348.206794295 | 0.307001603256 | oscillatory | None | oscillatory | None | False |
| dn0.60_ea18.2 | False | True | 112.004522336 | 351.567400346 | 0.312865038048 | oscillatory | None | oscillatory | None | False |
| dn0.60_ea18.4 | False | True | 114.128733009 | 354.934280721 | 0.318798648933 | oscillatory | None | oscillatory | None | False |
| dn0.60_ea18.6 | False | True | 116.27827518 | 358.307763811 | 0.324803018928 | oscillatory | None | oscillatory | None | False |

| Candidate | Level | Area mm² | F_lat N | F_ax envelope N | SF_fat | Torsional capacity Nmm | Sequence | Mesh/thickness SHA256 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| dn0.30_ea18.0 | coarse | 4 | 95.0319398834 | 388.981035388 | 0.265455098305 | 233.663327348 | 2.52392231217 | 5f23f108691d4383d8b1db913269160df8985a2ce5c82b5877c4779afb999c7c |
| dn0.30_ea18.0 | medium | 1 | 102.135709352 | 356.226433204 | 0.285298235516 | 248.487183731 | 2.52623256019 | 676836cbbee544f5b09675dbb01a9dc3bf6c4f5f424b4130a36e57651a3b036c |
| dn0.30_ea18.0 | fine | 0.25 | 107.435183647 | 290.716684341 | 0.300101389822 | 303.097159674 | 2.2803373828 | deeeaa32a8d7ac707a902b014c09c03cf0afa7f14bbef3b7204d6c66447fe0c9 |
| dn0.30_ea18.2 | coarse | 4 | 96.7283468314 | 392.680803417 | 0.270193714329 | 237.75406067 | 2.47965678972 | af2eaf9f8b8cfb0b27d04daa6fe4e1b342b03914f09feedafe7072c1f6a05566 |
| dn0.30_ea18.2 | medium | 1 | 104.073401673 | 359.076607436 | 0.290710840019 | 252.952663047 | 2.47918159363 | 2cb01fc886899dc44552f799bf2898b44076fbdfd4b509f8eece3527ed98b9e9 |
| dn0.30_ea18.2 | fine | 0.25 | 109.514539472 | 292.778477913 | 0.305909706536 | 308.599687774 | 2.23701941063 | 4cb65a46de545286456e86751f3cb8bf088c01f76126e58a8530e3a4e3114e4f |
| dn0.30_ea18.4 | coarse | 4 | 98.4438213766 | 396.387378326 | 0.27498559235 | 241.887929498 | 2.43644530643 | 690f6f4a75cdb3adda17d9999e1bbca896b407e67b6024eba8f1f32c18659a71 |
| dn0.30_ea18.4 | medium | 1 | 106.034083419 | 361.933074565 | 0.296187661455 | 257.465297535 | 2.43332380637 | 4889d6d5c8366ec2ab8d7c8e5aff3805d87581ca236e29ec78d82805bb884ced |
| dn0.30_ea18.4 | fine | 0.25 | 111.618988254 | 294.843751274 | 0.311788116037 | 314.159506347 | 2.19482338478 | 4d218b02448ed7652b443bba92a1b76fe640dde3b85a0a02d1ce4ec7a0ce4168 |
| dn0.30_ea18.6 | coarse | 4 | 100.178524241 | 400.10090256 | 0.279831181316 | 246.065243495 | 2.39425446201 | 7b1f1be8b42f32328846f79322c0fbbec48fe9819cc902ef9409c8278a263910 |
| dn0.30_ea18.6 | medium | 1 | 108.017946602 | 364.795931347 | 0.301729236182 | 262.025379478 | 2.3886187976 | 8e6e8a2897f6107bc9f58875d726c870e3889d980c4ff8c284ee36540dbd335a |
| dn0.30_ea18.6 | fine | 0.25 | 113.748738119 | 296.912582881 | 0.317737199689 | 319.776936461 | 2.15371061602 | 4a15997da6de3c27fec913a50b89d8425764f28f09279bd49216d47331242400 |
| dn0.40_ea18.0 | coarse | 4 | 99.084821323 | 385.212700573 | 0.276776113558 | 244.485640301 | 2.46613358206 | 3aadfeac5b2b7094985c1c3ad02f0bcae9e2156f436222f6a8bcf62048ca0c0b |
| dn0.40_ea18.0 | medium | 1 | 95.2165522239 | 378.317536803 | 0.26597078058 | 219.10712036 | 2.69881836888 | 46e703d444839292972a82b8c45ac60af09c7a2f29f7e26301f60fcd20ad2c4f |
| dn0.40_ea18.0 | fine | 0.25 | 109.088028749 | 302.435785245 | 0.304718323451 | 294.339606724 | 2.24614373848 | 01352d1dab79f9789f54ac4e4592c504857ea00013aa977698439ad62f4d20d7 |
| dn0.40_ea18.2 | coarse | 4 | 100.863153456 | 388.871423733 | 0.281743573255 | 248.779860536 | 2.42264109021 | 30a4a4beafbcb40927c8f43c333b061ff03896055bf65775d0ee49fa6373da2a |
| dn0.40_ea18.2 | medium | 1 | 96.9874824783 | 381.908241036 | 0.270917564423 | 222.983581629 | 2.64955803765 | 840ed66bad4f8492f5430ee7d8f826d88ce026e4f93173b880eda1bbd472b5f4 |
| dn0.40_ea18.2 | fine | 0.25 | 111.173272779 | 304.614270581 | 0.310543087837 | 299.70773562 | 2.20399303472 | fe4045188b9d0994edbfcdc93701e44626fa7d7ecbd7254e05cd9da3e6fc1be8 |
| dn0.40_ea18.4 | coarse | 4 | 102.661560598 | 392.536901805 | 0.286767108977 | 253.119441596 | 2.38019154235 | f5b3411d468ea70f135d4e94e6a2a7746cf1f228448849b3676b6613e6f39ea6 |
| dn0.40_ea18.4 | medium | 1 | 98.7791923329 | 385.505600309 | 0.275922392444 | 226.900412849 | 2.60151640417 | 9c3511309b068e52a27524c5a61f40fea58f284fce1aedb6bb6b9701142a1387 |
| dn0.40_ea18.4 | fine | 0.25 | 113.28345495 | 306.79662343 | 0.316437512555 | 305.133091052 | 2.1629189038 | 9177835be76c4159bbc17951161074af1505350e5cb610094a43f47528625168 |
| dn0.40_ea18.6 | coarse | 4 | 104.480211715 | 396.209273091 | 0.2918471927 | 257.504707262 | 2.33875027172 | 50d6cf2d618734f8afcf292578a3dea7a82829e6ac8ad1253a2900f27ef9fec8 |
| dn0.40_ea18.6 | medium | 1 | 100.591855354 | 389.109751327 | 0.280985749468 | 230.857860698 | 2.55465375196 | f5e250826a6d864b94a136e1f3037d70fb13903d6c4617923bb12bbc78de45c1 |
| dn0.40_ea18.6 | fine | 0.25 | 115.41878236 | 308.98292535 | 0.322402176102 | 310.616020139 | 2.12288515253 | 4a84ddf1a5b1770871e50c2fef5c49037f5520a0f674a273df8d8830d25a4761 |
| dn0.50_ea18.0 | coarse | 4 | 103.020687612 | 383.938723646 | 0.28777026746 | 255.974401859 | 2.33052401465 | 1731baa66b844952a89fba53418fced909ce94895d050288b1be923478337d26 |
| dn0.50_ea18.0 | medium | 1 | 105.364733495 | 349.553209013 | 0.294317949546 | 314.99287751 | 2.35137363432 | 96a2c1c9b96fba86f3743264075faa0acd6b733ac9f338a1d61086b8ad88198f |
| dn0.50_ea18.0 | fine | 0.25 | 116.639309934 | 337.591504585 | 0.32581150635 | 320.970562493 | 2.10139524676 | c5678cb93ce2944b0cda8e580c50f69de28dfa020a8ab264ec3edc522f8b7fad |
| dn0.50_ea18.2 | coarse | 4 | 104.874108719 | 387.590332397 | 0.292947475067 | 260.482362743 | 2.28933564781 | 423547b0ef817ff82555a0c77698d3f9db448bca70d0f7f82f4cdab2c4c492e4 |
| dn0.50_ea18.2 | medium | 1 | 107.399593325 | 352.699784164 | 0.300001974483 | 320.678266573 | 2.30681019356 | 59a27a346dee2abf68f18e2710a09aa1ccdffb9cd1be7cefd01cf08f0e949e4f |
| dn0.50_ea18.2 | fine | 0.25 | 118.915012457 | 340.126015085 | 0.332168283215 | 326.883682993 | 2.06116115475 | c6e2182c6822fbad17519baa6f4fb07d5efccfd8d29d5c6c0931cf4213745ee5 |
| dn0.50_ea18.4 | coarse | 4 | 106.748460733 | 391.244689414 | 0.298183149504 | 265.037946804 | 2.24913692759 | 2202c404309a37b73653e9c5c08916c44ac0c8c36108e9bb6ff51d1fdf75e057 |
| dn0.50_ea18.4 | medium | 1 | 109.458942762 | 355.850583591 | 0.305754406854 | 326.424348337 | 2.2633979239 | d0abb783a84a5c5f814a74e1fb0c5634a6fd5c811c6def906c84b252f44aa673 |
| dn0.50_ea18.4 | fine | 0.25 | 121.218321361 | 342.665348825 | 0.338602173678 | 332.860371397 | 2.02197839611 | 017bbacf119e25a05838fc7b53e4bace9bfb64dee826f9030bca6f2a5a29d9d4 |
| dn0.50_ea18.6 | coarse | 4 | 108.643920003 | 394.905988959 | 0.303477783364 | 269.64149336 | 2.20989625666 | ded8e7eda0c8f517f80230ac3704a86891103a8365dfffea549285a363bccf03 |
| dn0.50_ea18.6 | medium | 1 | 111.542985743 | 359.005743842 | 0.311575816319 | 332.231516142 | 2.22109772643 | 5503938d20693c9a730f900f3e06714cc8030aa52ebda9993aea5ff0c6b67de7 |
| dn0.50_ea18.6 | fine | 0.25 | 123.549465494 | 345.209597003 | 0.345113816981 | 338.901013287 | 1.98381039894 | 8909753035ddbaf91d0a6e6fb5042b91dd5bb0b1e0990d94c152a083443d412e |
| dn0.60_ea18.0 | coarse | 4 | 110.652665397 | 408.474219913 | 0.309088862194 | 263.130312185 | 2.17178215857 | a14765c230825dfc793825b5ea4db86e84bd6b2be2cb484a768b052a55526651 |
| dn0.60_ea18.0 | medium | 1 | 125.898366332 | 360.327580142 | 0.351675060534 | 424.144684677 | 2.04731169691 | 2f6c3bedc8daf9eaebea867b5512642b34f5f637a98827bbfa6bcf4741bf4f1e |
| dn0.60_ea18.0 | fine | 0.25 | 109.905434445 | 348.206794295 | 0.307001603256 | 281.936072845 | 2.22973360814 | 39aecf559523c2b05c7fe569cf425ad88325555cc24645e3e9125399bd737493 |
| dn0.60_ea18.2 | coarse | 4 | 112.733036355 | 412.41721035 | 0.314900014505 | 267.807700877 | 2.13170302464 | c24b1c55cb99085399c9d543de92bdc7951dc94753d063cd2f8aa562c666e2ea |
| dn0.60_ea18.2 | medium | 1 | 128.395952952 | 363.077498257 | 0.358651631804 | 432.061924646 | 2.00747374809 | cc9cf01ecdec322a3c1fcedceb09da8701ce31cc0b3edfc44da7aa08a86ae1a6 |
| dn0.60_ea18.2 | fine | 0.25 | 112.004522336 | 351.567400346 | 0.312865038048 | 287.022649936 | 2.18792564547 | 6a37d40be00125072e36ea5faf9f86bd369e881fd6aa7749fd56eba823b92bdc |
| dn0.60_ea18.4 | coarse | 4 | 114.83785329 | 416.368361175 | 0.320779452379 | 272.533850009 | 2.09263111106 | f7cf5a98f8aea308169ed609dc148ee69d2b3f3b0528167ebebaf0b996a2746a |
| dn0.60_ea18.4 | medium | 1 | 130.924180011 | 365.832873078 | 0.365713791783 | 440.062460131 | 1.96869596311 | 047b0be3a3f621f4f8bbf4b6b97d7a6f6a98920ae9ffd6e67a79a9378bfbe177 |
| dn0.60_ea18.4 | fine | 0.25 | 114.128733009 | 354.934280721 | 0.318798648933 | 292.161923797 | 2.14718384666 | b3ed25a2cafbf86fc333cecbe9b5d47330f08813428586eb596b9c3cd6639258 |
| dn0.60_ea18.6 | coarse | 4 | 116.967320904 | 420.327816588 | 0.326727747608 | 277.309066285 | 2.05453259468 | d40a6b22589574f2cf36829ea1f6d142f1d5533faac87596bce71db38cf0c77a |
| dn0.60_ea18.6 | medium | 1 | 133.483301589 | 368.593806149 | 0.372862250195 | 448.146747672 | 1.93094088901 | a7b02ca9497c8da1ca96d2ae7c3673fbeb02e81c65a112e6e3b8bbf19c05af7c |
| dn0.60_ea18.6 | fine | 0.25 | 116.27827518 | 358.307763811 | 0.324803018928 | 297.354189165 | 2.10747259946 | 012847da36e41552add6081fa74f3bb1077579939498ed9aedfbdd1afae496c0 |

| Candidate | Native gates::check rejects | Native aesthetic::evaluate failures |
| --- | --- | --- |
| dn0.30_ea18.0 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.30_ea18.2 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.30_ea18.4 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.30_ea18.6 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.40_ea18.0 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.40_ea18.2 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.40_ea18.4 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.40_ea18.6 | ["Holder envelope: best oriented box 200.7 x 100.1 > 200 x 100"] | [] |
| dn0.50_ea18.0 | ["Holder envelope: best oriented box 200.8 x 100.1 > 200 x 100"] | [] |
| dn0.50_ea18.2 | ["Holder envelope: best oriented box 200.8 x 100.1 > 200 x 100"] | [] |
| dn0.50_ea18.4 | ["Holder envelope: best oriented box 200.8 x 100.1 > 200 x 100"] | [] |
| dn0.50_ea18.6 | ["Holder envelope: best oriented box 200.8 x 100.1 > 200 x 100"] | [] |
| dn0.60_ea18.0 | ["Holder envelope: best oriented box 200.9 x 100.1 > 200 x 100"] | [] |
| dn0.60_ea18.2 | ["Holder envelope: best oriented box 200.9 x 100.1 > 200 x 100"] | [] |
| dn0.60_ea18.4 | ["Holder envelope: best oriented box 200.9 x 100.1 > 200 x 100"] | [] |
| dn0.60_ea18.6 | ["Holder envelope: best oriented box 200.9 x 100.1 > 200 x 100"] | [] |

Only dependent shared-test and receipt hash references in Phase9..13 were refreshed. The portable verifier restores recorded previous references to reconstruct every original parent receipt byte for byte. Prior measurements, verdicts, ranking, reports and historical checkers are unchanged. Raw target evidence is optional when absent; present evidence must match its hash and published measurement. The verifier needs no target, Git, Cargo, solver, affinity or worker environment and launches no processes.

Validation commands: `nice -n 10 taskset -c 12,13 python3 ops/ci/g16_06_rsv2_08/audit_phase14.py`; `nice -n 10 taskset -c 12,13 python3 -m unittest discover -s ops/ci/tests -p "test_portable_phase7.py"`; `git diff --check`. A relocated source/data fixture without target/.git is also checked. Floating-point tolerances apply only to cross-language GCI arithmetic and native CP distance identities; gate comparisons remain exact.

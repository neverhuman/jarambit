# G16-06-RSV2-08 Phase 10

Status: complete; 36/36 cases; 108/108 native plate observations. Counts: {'plate_gci_valid_3level': 36, 'seq_ratio_gci_admitted': 0, 'sequence_valid': 36, 'observable_valid': 36, 'phase9_fine_binding': 36}.

All36 unchanged Phase9 delayed-taper placement_05/local_01 profiles run through production jarambit_crescent::gci::run_plate_gci_3level. The API evaluates areas.25/1/4mm² fine to coarse (the requested4/1/.25mm²), h=.5/1/2mm, r=2, GCI safety factor1.25 and method relative_gci.v2. Canonical genomes, production export thickness, native FEM loads/supports/regions and thresholds are unchanged. Solver=faer, CPUs12,13, nice -n10 prefix (effective19), Cargo jobs2, two workers.

sequence_valid and observable_valid validate the bound three-level sequence and observable. Admitted additionally requires a monotone positive-order finite relative-error estimate. These flags are reported separately; checked metadata alone does not imply an admitted fit or asymptotic validity. The production battery plate_gci_valid_3level predicate is sequence_valid AND observable_valid; it is preserved and shown beside the stronger seq_ratio_gci.admitted predicate. No gate is replaced or relaxed.

The independently rerun.25mm² mesh hash must match Phase9 exactly. Sequence and hunter observables are compared against that Phase9 observation using the existing1e-9 replay bar. No Phase9 numerical results, report, candidates, section law or acceptance thresholds change. Full qualification=false; Phase9 Gate S and lateral/fatigue failures remain failures.

| Case | sequence_valid | observable_valid | admitted | Fit status | Reason | p | GCI fine | Phase9 binding |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| local_01_x136_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x136_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x136_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x136_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x136_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x136_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x142_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| local_01_x148_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x136_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x142_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t3.6_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t3.6_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t4.0_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t4.0_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t4.4_ea20.0 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |
| placement_05_x148_t4.4_ea21.8 | True | True | False | oscillatory | oscillatory_sequence | None | None | True |

Secondary hunter GCI is retained in the receipt; native sequence and hunter observations and exact mesh hashes follow.

| Case | Area mm² | h mm | Nodes | Elements | Sequence ratio | Hunter SF | Mesh/thickness SHA256 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| local_01_x136_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430449252 | 0.592284997078 | cb0d89637344411307b9e9f3f71e111c68013c0746510d252ca8562998551763 |
| local_01_x136_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.1382303799 | 0.715383716011 | 7800ef98c35089d641c50947e7bb2cdfc31dc70c67c334f73cc20f41435c4619 |
| local_01_x136_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47617138276 | 0.628075988959 | feee9ea7ec81c6f4eaf287642fd3dcb3ec71943859c25101a0664931ba59f54a |
| local_01_x136_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.3201518355 | 0.695416104244 | a43365bacea73a9303a0bf461f97a60222dca473e947e1af9ffe904b9b23d159 |
| local_01_x136_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81596120424 | 0.842291903778 | 29846b99a17a782234cc3278ea3bcaab507fc649ecd583795df0d74bead3e776 |
| local_01_x136_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10501359876 | 0.738791724414 | 06633b45ab6077e6e24afa5650da2efe328b4bbd152686f24a1e249e9db35041 |
| local_01_x136_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430375433 | 0.592285150856 | 478467d1aa13bf286758d7cd1c9a8588ca764e14f39c897cb25107bfec1fbbf5 |
| local_01_x136_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822840943 | 0.715384373217 | b2d47e4760f06e9b910fcd644c5f1db2f647aff04d52e1f111f0e64dc928950c |
| local_01_x136_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614880637 | 0.628081715091 | 5df24596dc274785580f14312b0a139dd3bb5ef9ac70571e5db2283cfe05d129 |
| local_01_x136_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014694939 | 0.695417540154 | 97174b6ae77381a489bf7d83f65b4ccf9caef0cfc10e523be8189a5f28ff8349 |
| local_01_x136_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159491174 | 0.842297511661 | 8856d3c32bdc9069e1d0deae6c9ebc4e986fbbf0df8ab2513764f108b232be71 |
| local_01_x136_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10494291818 | 0.738816531686 | 1b6e769cbe125edddbe487f2822d5bd1852f01ded61e19a7a01b5a91d43b0188 |
| local_01_x136_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430367735 | 0.59228516846 | bda196929fe37f0230996fce2e75b8ecc4fb75913d40db0776d7e7c86c7bc294 |
| local_01_x136_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822810451 | 0.715384476637 | c0c0d9f434c0de925af379305146eaf14123b777e68ab07d3ec8bd3731ec8c74 |
| local_01_x136_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614502291 | 0.628082674654 | 66ff8b1c72b1f755888cc0f5dec3929065bba835fe8b6df648e330e35a674edb |
| local_01_x136_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014626785 | 0.695417760308 | 18e4a445317b510c2dbc4603f74491b6ce43a5333ff8ec397b1886ebaaeb42f1 |
| local_01_x136_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594715588 | 0.842298412042 | cc534ced8d9a71e4a2efb1159ae7e729c63ced34c7d2121167ffe2db64304d96 |
| local_01_x136_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10492057585 | 0.738824373392 | 379766123cf15c4cc6cd9740c6335289e444c0c6ba356ccf5cf47bee59605215 |
| local_01_x142_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430418419 | 0.59228505382 | 5dbe1aff4d0fcc5871767570300e80751d362963910435ed16b18163ba4c1b4a |
| local_01_x142_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822962454 | 0.715383968967 | a4d96fdf2c42f81660810fc30033a38e8c13ff8a884a0fd1c64e984bec82b507 |
| local_01_x142_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47616428196 | 0.628077789827 | 71955c243e63be68769f2731a59e754a87af97dc317cba0b1b5b6d0e1aa49c64 |
| local_01_x142_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32015047446 | 0.695416464188 | b9665eb8aae54ef9f29b045b418a54e5dc18d0bcbeb0e0cadc97b07290d5a302 |
| local_01_x142_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81595764034 | 0.842293559551 | 302e7d4073034e868a72d26a2551cb8738bb4b0fb22603bc828c97db7781403e |
| local_01_x142_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10499428196 | 0.73879850372 | a404f0254d33f43dca128726809f383e283dbc0a2a07a7e9ba7266dad8de1589 |
| local_01_x142_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430374836 | 0.592285156929 | f65254657742870e5e9e6512baf615e5f37770db26802b33db8f1d458219b7e8 |
| local_01_x142_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822830315 | 0.71538441075 | 0001702eca2c124f400ac48438cd049f6c5a0a36d84cd89abc3c0355b5d4e742 |
| local_01_x142_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614744456 | 0.628082060644 | 6fa167152c919a55f78dd099e36060dfbf39cf3320d37ae85383c9defabf2e77 |
| local_01_x142_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.3201467848 | 0.695417601091 | 74577c4da5713affb13dbc619e9ee639b8964428f301d646a7f2fab8fde2ed2e |
| local_01_x142_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159486308 | 0.842297740306 | 3cb574d1fc48efcab0c5c0c08eff3cc94ec602c8650e3bc9cd6404b84d4c68fa |
| local_01_x142_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10493707096 | 0.738818583739 | dca694143039ceef23040bea82a050418ceef5bc432708f13660210931d0f4c2 |
| local_01_x142_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430365808 | 0.592285169107 | 4414c4c4bbfc10f6c9f78b3a823ba43a0a264c51fadf94aaff88588e84c7863b |
| local_01_x142_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822809731 | 0.715384480726 | dcb1cedbdcca67e806e681a439d8e729c0ce0b0d36f6a8e0c0320ab6d43e2176 |
| local_01_x142_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614491345 | 0.628082702384 | cf959fb0ed70955148a245615274537c1d82d8fb8611f68a21ec1bde34cbe08c |
| local_01_x142_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014624173 | 0.695417768121 | 66f925b6d430489c1580d2d182a088fcc632a64443fd6ecd48ec4fd053d67e3b |
| local_01_x142_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594708099 | 0.842298453571 | d69b1f615fff72fe3a6e44835de12e9d6ab5d8ebe9edd532568d51b6dc3eb7b6 |
| local_01_x142_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10491968254 | 0.738824687402 | d8edcbb04d2e0778439076058456268fa68608881574b10b7105e1c446585dd9 |
| local_01_x148_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430393445 | 0.592285101184 | d1d2967a14b3f820d6e77eb39c7627a966c8c183c3e2faf084e7542ba0f21815 |
| local_01_x148_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822906966 | 0.715384158113 | 6413379ada8456b82a71a1a218f3c9fa16d0392c6d9987ba588119c6ad67c981 |
| local_01_x148_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47615597479 | 0.628079896427 | 94c054f1dd8b6367b3bb02f8b0f5a3449f9c39ee03117bf1024f7a686a244b9c |
| local_01_x148_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014907858 | 0.695416898138 | 30a086c4482e94c8a402bd231dacc9a22516d4786edf8c895786000f2e9b8860 |
| local_01_x148_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81595462787 | 0.84229495451 | 1e2b949ea31a6d093a1d8ec7e29c1117a0e315d5d1bf7b176fa72d781136eaa4 |
| local_01_x148_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.1049747811 | 0.738805348289 | 082f3e4f462cece48ba58a126d1ed4ba313309f4bf022ebd870eeb6c9c668b58 |
| local_01_x148_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430356376 | 0.592285162905 | b0b9baaa31305db87c06cd6dbf35bed549217d8e123869aff9e8379299390c52 |
| local_01_x148_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822821479 | 0.715384447476 | fb3c8d2c2b11e953bc0a178fd1fa365218c6625d3576be61301693add40c0fcc |
| local_01_x148_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614621242 | 0.628082372964 | ab2baf5ea3a73c2cda3bbadbbb84f5b34f8f9b34cb13274395b3e012ba4aeb5e |
| local_01_x148_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014658019 | 0.695417663595 | d32487bc5897b0816cdc7b29ba1c80474a6726e410959eee7322a6fdcbfeab93 |
| local_01_x148_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159480056 | 0.842298025607 | 3aa569e40ad8fea9f803c819315430c28ea53037105c61010104a1ec9bd35949 |
| local_01_x148_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10493132664 | 0.738820600788 | 407ffd37a4078cdd93b1ef6dfc191f219c52be284487b45bd2d8c9ff424fb8c8 |
| local_01_x148_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430353648 | 0.5922851697 | 1de4b81273959236088c009334958f326f93283742598caee19a077843f0b5a9 |
| local_01_x148_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822809542 | 0.715384484094 | 77a075abb3d2374129f44d5fe531608da953ff1e9c286f9676a7f511237e0eff |
| local_01_x148_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614463617 | 0.628082772647 | d622c32fb94bebe1e9bc9399d27906f00f3f97053839685f64239c4e157de8ca |
| local_01_x148_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014617495 | 0.695417776777 | 4c4307a6c530414c07957ec102147ed359aac51aa15e7bdd4f466aa1e2d5ddd6 |
| local_01_x148_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594695709 | 0.842298513485 | 9d9952271cc8b0c107211d49476e82ebdb0ad54af4a779a4e25bce30e2b946d2 |
| local_01_x148_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.1049176717 | 0.73882539345 | 5d606727f15a6341a940ce744bf1ce1f243bcd07a98751719a2f988e62baafed |
| placement_05_x136_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430449252 | 0.592284997078 | cb0d89637344411307b9e9f3f71e111c68013c0746510d252ca8562998551763 |
| placement_05_x136_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.1382303799 | 0.715383716011 | 7800ef98c35089d641c50947e7bb2cdfc31dc70c67c334f73cc20f41435c4619 |
| placement_05_x136_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47617138276 | 0.628075988959 | feee9ea7ec81c6f4eaf287642fd3dcb3ec71943859c25101a0664931ba59f54a |
| placement_05_x136_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.3201518355 | 0.695416104244 | a43365bacea73a9303a0bf461f97a60222dca473e947e1af9ffe904b9b23d159 |
| placement_05_x136_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81596120424 | 0.842291903778 | 29846b99a17a782234cc3278ea3bcaab507fc649ecd583795df0d74bead3e776 |
| placement_05_x136_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10501359876 | 0.738791724414 | 06633b45ab6077e6e24afa5650da2efe328b4bbd152686f24a1e249e9db35041 |
| placement_05_x136_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430375433 | 0.592285150856 | 478467d1aa13bf286758d7cd1c9a8588ca764e14f39c897cb25107bfec1fbbf5 |
| placement_05_x136_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822840943 | 0.715384373217 | b2d47e4760f06e9b910fcd644c5f1db2f647aff04d52e1f111f0e64dc928950c |
| placement_05_x136_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614880637 | 0.628081715091 | 5df24596dc274785580f14312b0a139dd3bb5ef9ac70571e5db2283cfe05d129 |
| placement_05_x136_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014694939 | 0.695417540154 | 97174b6ae77381a489bf7d83f65b4ccf9caef0cfc10e523be8189a5f28ff8349 |
| placement_05_x136_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159491174 | 0.842297511661 | 8856d3c32bdc9069e1d0deae6c9ebc4e986fbbf0df8ab2513764f108b232be71 |
| placement_05_x136_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10494291818 | 0.738816531686 | 1b6e769cbe125edddbe487f2822d5bd1852f01ded61e19a7a01b5a91d43b0188 |
| placement_05_x136_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430367735 | 0.59228516846 | bda196929fe37f0230996fce2e75b8ecc4fb75913d40db0776d7e7c86c7bc294 |
| placement_05_x136_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822810451 | 0.715384476637 | c0c0d9f434c0de925af379305146eaf14123b777e68ab07d3ec8bd3731ec8c74 |
| placement_05_x136_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614502291 | 0.628082674654 | 66ff8b1c72b1f755888cc0f5dec3929065bba835fe8b6df648e330e35a674edb |
| placement_05_x136_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014626785 | 0.695417760308 | 18e4a445317b510c2dbc4603f74491b6ce43a5333ff8ec397b1886ebaaeb42f1 |
| placement_05_x136_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594715588 | 0.842298412042 | cc534ced8d9a71e4a2efb1159ae7e729c63ced34c7d2121167ffe2db64304d96 |
| placement_05_x136_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10492057585 | 0.738824373392 | 379766123cf15c4cc6cd9740c6335289e444c0c6ba356ccf5cf47bee59605215 |
| placement_05_x142_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430418419 | 0.59228505382 | 5dbe1aff4d0fcc5871767570300e80751d362963910435ed16b18163ba4c1b4a |
| placement_05_x142_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822962454 | 0.715383968967 | a4d96fdf2c42f81660810fc30033a38e8c13ff8a884a0fd1c64e984bec82b507 |
| placement_05_x142_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47616428196 | 0.628077789827 | 71955c243e63be68769f2731a59e754a87af97dc317cba0b1b5b6d0e1aa49c64 |
| placement_05_x142_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32015047446 | 0.695416464188 | b9665eb8aae54ef9f29b045b418a54e5dc18d0bcbeb0e0cadc97b07290d5a302 |
| placement_05_x142_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81595764034 | 0.842293559551 | 302e7d4073034e868a72d26a2551cb8738bb4b0fb22603bc828c97db7781403e |
| placement_05_x142_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10499428196 | 0.73879850372 | a404f0254d33f43dca128726809f383e283dbc0a2a07a7e9ba7266dad8de1589 |
| placement_05_x142_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430374836 | 0.592285156929 | f65254657742870e5e9e6512baf615e5f37770db26802b33db8f1d458219b7e8 |
| placement_05_x142_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822830315 | 0.71538441075 | 0001702eca2c124f400ac48438cd049f6c5a0a36d84cd89abc3c0355b5d4e742 |
| placement_05_x142_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614744456 | 0.628082060644 | 6fa167152c919a55f78dd099e36060dfbf39cf3320d37ae85383c9defabf2e77 |
| placement_05_x142_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.3201467848 | 0.695417601091 | 74577c4da5713affb13dbc619e9ee639b8964428f301d646a7f2fab8fde2ed2e |
| placement_05_x142_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159486308 | 0.842297740306 | 3cb574d1fc48efcab0c5c0c08eff3cc94ec602c8650e3bc9cd6404b84d4c68fa |
| placement_05_x142_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10493707096 | 0.738818583739 | dca694143039ceef23040bea82a050418ceef5bc432708f13660210931d0f4c2 |
| placement_05_x142_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430365808 | 0.592285169107 | 4414c4c4bbfc10f6c9f78b3a823ba43a0a264c51fadf94aaff88588e84c7863b |
| placement_05_x142_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822809731 | 0.715384480726 | dcb1cedbdcca67e806e681a439d8e729c0ce0b0d36f6a8e0c0320ab6d43e2176 |
| placement_05_x142_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614491345 | 0.628082702384 | cf959fb0ed70955148a245615274537c1d82d8fb8611f68a21ec1bde34cbe08c |
| placement_05_x142_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014624173 | 0.695417768121 | 66f925b6d430489c1580d2d182a088fcc632a64443fd6ecd48ec4fd053d67e3b |
| placement_05_x142_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594708099 | 0.842298453571 | d69b1f615fff72fe3a6e44835de12e9d6ab5d8ebe9edd532568d51b6dc3eb7b6 |
| placement_05_x142_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10491968254 | 0.738824687402 | d8edcbb04d2e0778439076058456268fa68608881574b10b7105e1c446585dd9 |
| placement_05_x148_t3.6_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430393445 | 0.592285101184 | d1d2967a14b3f820d6e77eb39c7627a966c8c183c3e2faf084e7542ba0f21815 |
| placement_05_x148_t3.6_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822906966 | 0.715384158113 | 6413379ada8456b82a71a1a218f3c9fa16d0392c6d9987ba588119c6ad67c981 |
| placement_05_x148_t3.6_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47615597479 | 0.628079896427 | 94c054f1dd8b6367b3bb02f8b0f5a3449f9c39ee03117bf1024f7a686a244b9c |
| placement_05_x148_t3.6_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014907858 | 0.695416898138 | 30a086c4482e94c8a402bd231dacc9a22516d4786edf8c895786000f2e9b8860 |
| placement_05_x148_t3.6_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81595462787 | 0.84229495451 | 1e2b949ea31a6d093a1d8ec7e29c1117a0e315d5d1bf7b176fa72d781136eaa4 |
| placement_05_x148_t3.6_ea21.8 | 4 | 2 | 2048 | 3385 | 2.1049747811 | 0.738805348289 | 082f3e4f462cece48ba58a126d1ed4ba313309f4bf022ebd870eeb6c9c668b58 |
| placement_05_x148_t4.0_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430356376 | 0.592285162905 | b0b9baaa31305db87c06cd6dbf35bed549217d8e123869aff9e8379299390c52 |
| placement_05_x148_t4.0_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822821479 | 0.715384447476 | fb3c8d2c2b11e953bc0a178fd1fa365218c6625d3576be61301693add40c0fcc |
| placement_05_x148_t4.0_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614621242 | 0.628082372964 | ab2baf5ea3a73c2cda3bbadbbb84f5b34f8f9b34cb13274395b3e012ba4aeb5e |
| placement_05_x148_t4.0_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014658019 | 0.695417663595 | d32487bc5897b0816cdc7b29ba1c80474a6726e410959eee7322a6fdcbfeab93 |
| placement_05_x148_t4.0_ea21.8 | 1 | 1 | 4541 | 8223 | 1.8159480056 | 0.842298025607 | 3aa569e40ad8fea9f803c819315430c28ea53037105c61010104a1ec9bd35949 |
| placement_05_x148_t4.0_ea21.8 | 4 | 2 | 2048 | 3385 | 2.10493132664 | 0.738820600788 | 407ffd37a4078cdd93b1ef6dfc191f219c52be284487b45bd2d8c9ff424fb8c8 |
| placement_05_x148_t4.4_ea20.0 | 0.25 | 0.5 | 16188 | 30747 | 2.72430353648 | 0.5922851697 | 1de4b81273959236088c009334958f326f93283742598caee19a077843f0b5a9 |
| placement_05_x148_t4.4_ea20.0 | 1 | 1 | 4541 | 8223 | 2.13822809542 | 0.715384484094 | 77a075abb3d2374129f44d5fe531608da953ff1e9c286f9676a7f511237e0eff |
| placement_05_x148_t4.4_ea20.0 | 4 | 2 | 2048 | 3385 | 2.47614463617 | 0.628082772647 | d622c32fb94bebe1e9bc9399d27906f00f3f97053839685f64239c4e157de8ca |
| placement_05_x148_t4.4_ea21.8 | 0.25 | 0.5 | 16188 | 30747 | 2.32014617495 | 0.695417776777 | 4c4307a6c530414c07957ec102147ed359aac51aa15e7bdd4f466aa1e2d5ddd6 |
| placement_05_x148_t4.4_ea21.8 | 1 | 1 | 4541 | 8223 | 1.81594695709 | 0.842298513485 | 9d9952271cc8b0c107211d49476e82ebdb0ad54af4a779a4e25bce30e2b946d2 |
| placement_05_x148_t4.4_ea21.8 | 4 | 2 | 2048 | 3385 | 2.1049176717 | 0.73882539345 | 5d606727f15a6341a940ce744bf1ce1f243bcd07a98751719a2f988e62baafed |

The portable verifier checks tracked data and recomputes GCI arithmetic with the repository standard-library Python reference; it does not replay native solves and requires no target directory, Git metadata, Cargo settings or CPU mask. Present raw evidence is hash-checked. Asymptotic validation remains not_established; GCI admission is not full product qualification.

Shared test metadata: Phase9 receipt changes only the SHA256 entry for ops/ci/tests/test_portable_phase7.py, which now includes Phase10 tests. The original receipt hash is e686af0a5bd674479acfbcfc30af135f9705785cb479e4ce15b13e999b94c6fe. Restoring that single original digest reconstructs the original receipt byte for byte. Its scientific-data fingerprint and PHASE9.md hash are unchanged.

Reference arithmetic retains the repository Unicode15 identity contract (Python3.12 here). Cross-language p/extrapolation/GCI numeric comparisons use relative1e-12/absolute1e-14 only to compare libm arithmetic; all validity/admission decisions, mesh identities, method constants and gate predicates compare exactly without an added tolerance.

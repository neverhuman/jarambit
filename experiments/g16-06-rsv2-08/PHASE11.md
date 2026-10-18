# G16-06-RSV2-08 Phase 11

Status: complete; 36/36 cases. Counts: {'a4_complete': 36, 'buckling_pass': 36, 'support_sequence_pass': 36, 'side_a4_pass': 36, 'parent_mesh_binding': 36}.

All36 unchanged Phase9/10 placement_05/local_01 delayed-taper profiles use the native production side_a4 elastic-foundation solver through battery::run_side_a4_native_for_genome, on the full-precision in-memory production FEM .25mm² mesh. Exact binary xy/f64LE, triangle/u64LE and element-thickness/f64LE hashes bind to both Phase9 and Phase10; the complete unrounded mesh JSON is retained as optional raw evidence. The native battery d1_sha256 zero sentinel is preserved and is not claimed as a D1 certificate; separate mesh JSON and binary hashes provide the binding.

seq_cert is the unchanged Phase9 certificate minimum over.25/.125mm², not a substituted floor or the lone.25 result. The requested buckling predicate is eigenvalue>=1.0; support consistency is seq_a4>=seq_cert, evaluated independently and jointly without a tolerance. Native solver status is retained separately: it checks eigenvalue>0 and support consistency, so status alone is not the requested buckling verdict. Buckling critical load is260N times eigenvalue.

The unmodified native5-DOF CST/DKT solver uses radial/tangential/flange bore springs50000/5000/100000, grip Winkler foundation200, E=113800MPa and nu=.34. Native tip loading,15mm stress-region exclusion,32-step deterministic K-orthogonal Lanczos and regularization are unchanged. Report_fields remains the ring-clamp plate model; A4 support results do not replace its certified sequence. Explicit faer, CPUs12,13, nice -n10 prefix/effective19, Cargo jobs2, two workers.

Phase9 Gate S/lateral/fatigue failures and Phase10 oscillatory/unadmitted GCI remain unchanged. This A4 study does not establish all-gate qualification. Full qualification=false. No native source, design, support, load, gate or threshold changed.

| Case | Eigenvalue | Buckling margin | seq_a4 | seq_cert | Sequence margin | Buckling pass | Sequence pass | A4 pass | Native status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| local_01_x136_t3.6_ea20.0 | 7.78045057385 | 6.78045057385 | 2.80573151869 | 2.63437392866 | 0.17135759003 | True | True | True | pass |
| local_01_x136_t3.6_ea21.8 | 9.44896745494 | 8.44896745494 | 2.64524349075 | 2.24282694931 | 0.402416541443 | True | True | True | pass |
| local_01_x136_t4.0_ea20.0 | 7.82815647585 | 6.82815647585 | 2.7719374153 | 2.63437296003 | 0.137564455268 | True | True | True | pass |
| local_01_x136_t4.0_ea21.8 | 9.54194717614 | 8.54194717614 | 2.52515401665 | 2.24282058464 | 0.282333432012 | True | True | True | pass |
| local_01_x136_t4.4_ea20.0 | 7.85406700091 | 6.85406700091 | 2.76967706157 | 2.63437274181 | 0.135304319757 | True | True | True | pass |
| local_01_x136_t4.4_ea21.8 | 9.59854576426 | 8.59854576426 | 2.48505287693 | 2.24281971431 | 0.24223316262 | True | True | True | pass |
| local_01_x142_t3.6_ea20.0 | 7.80727475828 | 6.80727475828 | 2.79481034609 | 2.63437340555 | 0.160436940544 | True | True | True | pass |
| local_01_x142_t3.6_ea21.8 | 9.49964260193 | 8.49964260193 | 2.62025708667 | 2.24282512397 | 0.377431962703 | True | True | True | pass |
| local_01_x142_t4.0_ea20.0 | 7.84268844491 | 6.84268844491 | 2.77120002519 | 2.63437295954 | 0.136827065658 | True | True | True | pass |
| local_01_x142_t4.0_ea21.8 | 9.57213501885 | 8.57213501885 | 2.5148173599 | 2.2428201842 | 0.271997175696 | True | True | True | pass |
| local_01_x142_t4.4_ea20.0 | 7.86116642821 | 6.86116642821 | 2.76982016658 | 2.63437255844 | 0.135447608142 | True | True | True | pass |
| local_01_x142_t4.4_ea21.8 | 9.61604600975 | 8.61604600975 | 2.48430622038 | 2.24281956256 | 0.241486657819 | True | True | True | pass |
| local_01_x148_t3.6_ea20.0 | 7.82990180312 | 6.82990180312 | 2.78317925553 | 2.6343729964 | 0.148806259128 | True | True | True | pass |
| local_01_x148_t3.6_ea21.8 | 9.54650135961 | 8.54650135961 | 2.59202258596 | 2.24282312891 | 0.349199457047 | True | True | True | pass |
| local_01_x148_t4.0_ea20.0 | 7.85431937954 | 6.85431937954 | 2.77024393485 | 2.63437255797 | 0.135871376885 | True | True | True | pass |
| local_01_x148_t4.0_ea21.8 | 9.60009430396 | 8.60009430396 | 2.50475613805 | 2.24281984505 | 0.261936293002 | True | True | True | pass |
| local_01_x148_t4.4_ea20.0 | 7.86638229385 | 6.86638229385 | 2.76983486514 | 2.63437263965 | 0.13546222549 | True | True | True | pass |
| local_01_x148_t4.4_ea21.8 | 9.63229794436 | 8.63229794436 | 2.48313705671 | 2.24281964471 | 0.240317412002 | True | True | True | pass |
| placement_05_x136_t3.6_ea20.0 | 7.78045057385 | 6.78045057385 | 2.80573151869 | 2.63437392866 | 0.17135759003 | True | True | True | pass |
| placement_05_x136_t3.6_ea21.8 | 9.44896745494 | 8.44896745494 | 2.64524349075 | 2.24282694931 | 0.402416541443 | True | True | True | pass |
| placement_05_x136_t4.0_ea20.0 | 7.82815647585 | 6.82815647585 | 2.7719374153 | 2.63437296003 | 0.137564455268 | True | True | True | pass |
| placement_05_x136_t4.0_ea21.8 | 9.54194717614 | 8.54194717614 | 2.52515401665 | 2.24282058464 | 0.282333432012 | True | True | True | pass |
| placement_05_x136_t4.4_ea20.0 | 7.85406700091 | 6.85406700091 | 2.76967706157 | 2.63437274181 | 0.135304319757 | True | True | True | pass |
| placement_05_x136_t4.4_ea21.8 | 9.59854576426 | 8.59854576426 | 2.48505287693 | 2.24281971431 | 0.24223316262 | True | True | True | pass |
| placement_05_x142_t3.6_ea20.0 | 7.80727475828 | 6.80727475828 | 2.79481034609 | 2.63437340555 | 0.160436940544 | True | True | True | pass |
| placement_05_x142_t3.6_ea21.8 | 9.49964260193 | 8.49964260193 | 2.62025708667 | 2.24282512397 | 0.377431962703 | True | True | True | pass |
| placement_05_x142_t4.0_ea20.0 | 7.84268844491 | 6.84268844491 | 2.77120002519 | 2.63437295954 | 0.136827065658 | True | True | True | pass |
| placement_05_x142_t4.0_ea21.8 | 9.57213501885 | 8.57213501885 | 2.5148173599 | 2.2428201842 | 0.271997175696 | True | True | True | pass |
| placement_05_x142_t4.4_ea20.0 | 7.86116642821 | 6.86116642821 | 2.76982016658 | 2.63437255844 | 0.135447608142 | True | True | True | pass |
| placement_05_x142_t4.4_ea21.8 | 9.61604600975 | 8.61604600975 | 2.48430622038 | 2.24281956256 | 0.241486657819 | True | True | True | pass |
| placement_05_x148_t3.6_ea20.0 | 7.82990180312 | 6.82990180312 | 2.78317925553 | 2.6343729964 | 0.148806259128 | True | True | True | pass |
| placement_05_x148_t3.6_ea21.8 | 9.54650135961 | 8.54650135961 | 2.59202258596 | 2.24282312891 | 0.349199457047 | True | True | True | pass |
| placement_05_x148_t4.0_ea20.0 | 7.85431937954 | 6.85431937954 | 2.77024393485 | 2.63437255797 | 0.135871376885 | True | True | True | pass |
| placement_05_x148_t4.0_ea21.8 | 9.60009430396 | 8.60009430396 | 2.50475613805 | 2.24281984505 | 0.261936293002 | True | True | True | pass |
| placement_05_x148_t4.4_ea20.0 | 7.86638229385 | 6.86638229385 | 2.76983486514 | 2.63437263965 | 0.13546222549 | True | True | True | pass |
| placement_05_x148_t4.4_ea21.8 | 9.63229794436 | 8.63229794436 | 2.48313705671 | 2.24281964471 | 0.240317412002 | True | True | True | pass |

| Case | Nodes | Elements | Bore nodes | Grip nodes | Parent binding | Mesh/thickness SHA256 |
| --- | --- | --- | --- | --- | --- | --- |
| local_01_x136_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | cb0d89637344411307b9e9f3f71e111c68013c0746510d252ca8562998551763 |
| local_01_x136_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | a43365bacea73a9303a0bf461f97a60222dca473e947e1af9ffe904b9b23d159 |
| local_01_x136_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 478467d1aa13bf286758d7cd1c9a8588ca764e14f39c897cb25107bfec1fbbf5 |
| local_01_x136_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 97174b6ae77381a489bf7d83f65b4ccf9caef0cfc10e523be8189a5f28ff8349 |
| local_01_x136_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | bda196929fe37f0230996fce2e75b8ecc4fb75913d40db0776d7e7c86c7bc294 |
| local_01_x136_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 18e4a445317b510c2dbc4603f74491b6ce43a5333ff8ec397b1886ebaaeb42f1 |
| local_01_x142_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 5dbe1aff4d0fcc5871767570300e80751d362963910435ed16b18163ba4c1b4a |
| local_01_x142_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | b9665eb8aae54ef9f29b045b418a54e5dc18d0bcbeb0e0cadc97b07290d5a302 |
| local_01_x142_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | f65254657742870e5e9e6512baf615e5f37770db26802b33db8f1d458219b7e8 |
| local_01_x142_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 74577c4da5713affb13dbc619e9ee639b8964428f301d646a7f2fab8fde2ed2e |
| local_01_x142_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 4414c4c4bbfc10f6c9f78b3a823ba43a0a264c51fadf94aaff88588e84c7863b |
| local_01_x142_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 66f925b6d430489c1580d2d182a088fcc632a64443fd6ecd48ec4fd053d67e3b |
| local_01_x148_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | d1d2967a14b3f820d6e77eb39c7627a966c8c183c3e2faf084e7542ba0f21815 |
| local_01_x148_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 30a086c4482e94c8a402bd231dacc9a22516d4786edf8c895786000f2e9b8860 |
| local_01_x148_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | b0b9baaa31305db87c06cd6dbf35bed549217d8e123869aff9e8379299390c52 |
| local_01_x148_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | d32487bc5897b0816cdc7b29ba1c80474a6726e410959eee7322a6fdcbfeab93 |
| local_01_x148_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 1de4b81273959236088c009334958f326f93283742598caee19a077843f0b5a9 |
| local_01_x148_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 4c4307a6c530414c07957ec102147ed359aac51aa15e7bdd4f466aa1e2d5ddd6 |
| placement_05_x136_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | cb0d89637344411307b9e9f3f71e111c68013c0746510d252ca8562998551763 |
| placement_05_x136_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | a43365bacea73a9303a0bf461f97a60222dca473e947e1af9ffe904b9b23d159 |
| placement_05_x136_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 478467d1aa13bf286758d7cd1c9a8588ca764e14f39c897cb25107bfec1fbbf5 |
| placement_05_x136_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 97174b6ae77381a489bf7d83f65b4ccf9caef0cfc10e523be8189a5f28ff8349 |
| placement_05_x136_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | bda196929fe37f0230996fce2e75b8ecc4fb75913d40db0776d7e7c86c7bc294 |
| placement_05_x136_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 18e4a445317b510c2dbc4603f74491b6ce43a5333ff8ec397b1886ebaaeb42f1 |
| placement_05_x142_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 5dbe1aff4d0fcc5871767570300e80751d362963910435ed16b18163ba4c1b4a |
| placement_05_x142_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | b9665eb8aae54ef9f29b045b418a54e5dc18d0bcbeb0e0cadc97b07290d5a302 |
| placement_05_x142_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | f65254657742870e5e9e6512baf615e5f37770db26802b33db8f1d458219b7e8 |
| placement_05_x142_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 74577c4da5713affb13dbc619e9ee639b8964428f301d646a7f2fab8fde2ed2e |
| placement_05_x142_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 4414c4c4bbfc10f6c9f78b3a823ba43a0a264c51fadf94aaff88588e84c7863b |
| placement_05_x142_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 66f925b6d430489c1580d2d182a088fcc632a64443fd6ecd48ec4fd053d67e3b |
| placement_05_x148_t3.6_ea20.0 | 16188 | 30747 | 131 | 7813 | True | d1d2967a14b3f820d6e77eb39c7627a966c8c183c3e2faf084e7542ba0f21815 |
| placement_05_x148_t3.6_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 30a086c4482e94c8a402bd231dacc9a22516d4786edf8c895786000f2e9b8860 |
| placement_05_x148_t4.0_ea20.0 | 16188 | 30747 | 131 | 7813 | True | b0b9baaa31305db87c06cd6dbf35bed549217d8e123869aff9e8379299390c52 |
| placement_05_x148_t4.0_ea21.8 | 16188 | 30747 | 131 | 7813 | True | d32487bc5897b0816cdc7b29ba1c80474a6726e410959eee7322a6fdcbfeab93 |
| placement_05_x148_t4.4_ea20.0 | 16188 | 30747 | 131 | 7813 | True | 1de4b81273959236088c009334958f326f93283742598caee19a077843f0b5a9 |
| placement_05_x148_t4.4_ea21.8 | 16188 | 30747 | 131 | 7813 | True | 4c4307a6c530414c07957ec102147ed359aac51aa15e7bdd4f466aa1e2d5ddd6 |

The portable audit checks tracked results, input identity, certificate minima, both exact gate comparisons, margins, support constants, parent mesh bindings and published tables. Present raw outputs/mesh files must match hashes; missing target files are explicitly optional. It performs no native solve and needs no target directory, Git metadata, worker CPU affinity or Cargo settings.

Final native observations: {'a4_complete': 36, 'buckling_pass': 36, 'support_sequence_pass': 36, 'side_a4_pass': 36, 'parent_mesh_binding': 36}.
Eigenvalue range: 7.780450573848691..9.632297944364172. Support sequence margin range: 0.13530431975652046..0.4024165414434937.

Earlier receipt updates change only dependent hash metadata: the shared test-file reference in Phase9, and the test/Phase9/current-test references in Phase10. Restoring the recorded old references reconstructs both original receipts byte for byte; every prior measurement, gate decision and report is unchanged. The verifier requires those reconstructions.

Arithmetic identity checks use relative1e-14 solely for derived load/peak ratios; neither eigenvalue>=1 nor seq_a4>=seq_cert receives any comparison tolerance. Native solves are not replayed by the portable audit.

Validation: `python3 ops/ci/g16_06_rsv2_08/audit_phase11.py` and `python3 -m unittest ops/ci/tests/test_portable_phase7.py -v`. The assignment explicitly forbids committing or pushing; neither is attempted.

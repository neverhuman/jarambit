# Phase-20 cross-wave Stage-2 identity deduplication audit

**PASS for saved provenance and exact digest/byte deduplication.** 23 assessed events reference 11 distinct certificate identities, leaving 12 repeat assessments beyond the first occurrence. They bind 11 distinct genome CSV byte hashes. One separate unverified attempt is indexed and excluded from all assessed/novelty totals.

| Certificate identity | Assessed events | Repeats beyond first | First-seen wave |
| --- | ---: | ---: | --- |
| 234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70 | 2 | 1 | w316 |
| 3eb40f1b9d74669c3a9afd22833888e26542a7c52ae512bbfa3f77fe5525b2db | 1 | 0 | w318 |
| 44ac7f99fd172d52780482c32dffe9e012282355e941ecc8ebbf0c40ebc35a43 | 1 | 0 | w318 |
| 47d60b04044f32fc84f17d915af58c44e0896e1f25ab97dda6587950f44b2850 | 1 | 0 | w319 |
| 6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac | 6 | 5 | w315 |
| 74ed6ed4ab8bf8a042352e73ca6a51561821d515acc09f21ed83bbaa880ad94e | 1 | 0 | w315 |
| 89363bf867d1e07b9f91a4f72c181024f8823b4268987517fdc3d00a638d0d3c | 1 | 0 | w319 |
| b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc | 6 | 5 | w315 |
| b77c59c158cf89b22a5493bfe380f31397eeb3a77c05a56fb9db46a584c7c0cb | 1 | 0 | w315 |
| d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f | 2 | 1 | w316 |
| dea35f5fb19c5f11bad56e636be1f88434aed9a0fa2547428c2e58619c128599 | 1 | 0 | w320 |

The b2cf2b33 and 6af25de9 groups each appear in all six waves. Each contains six assessment events and five repeats beyond its first occurrence. Their ten repeat events are part of the overall repeat total; other repeated identities are listed above. A repeat here means the same saved certificate identity and identical genome CSV bytes, not byte-identical assessment output files. Reused identity fields are also checked against the SHA-256 of the actual genome CSV bytes. Equal genome bytes with different certificate identities remain distinct identity groups; both indexes are retained.

| Wave | Assessed events | Identities first seen in this wave | Repeat events |
| --- | ---: | ---: | ---: |
| w315 | 4 | 4 | 0 |
| w316 | 4 | 2 | 2 |
| w317 | 4 | 0 | 4 |
| w318 | 4 | 2 | 2 |
| w319 | 4 | 2 | 2 |
| w320 | 3 | 1 | 2 |

Novelty means first occurrence in the listed w315–w320 wave order, not a claim about execution time, physical novelty or performance. Within-wave event rows retain the saved leaderboard order. Root/birth/screen digest occurrences never count as additional assessments. The w320_r02_p14_c93 unverified attempt remains separate and does not become an assessed event just because intermediate files exist.

The audit scans all exact certificate_identity and genome_sha256 fields recursively in 497 selected source JSON files, including nested attempt JSON. It records 2088 certificate-identity and 186 genome-hash field occurrences, plus byte bindings for 24 genome CSV files. Field locations are arrays of JSON keys/indices, not filesystem paths. No other numerical data is copied into the index.

| Wave | Actual saved root source |
| --- | --- |
| w315 | evidence/evolution_waves/2026-10-17/xbabe2/out/w315 |
| w316 | evidence/evolution_waves/2026-10-17/xbabe2/out/w316 |
| w317 | evidence/evolution_waves/2026-10-17/xbabe2/out/w317 |
| w318 | out/w318 |
| w319 | out/w319 |
| w320 | out/w320 |

The short evidence/evolution_waves/2026-10-17/w315 through w318 paths named in the assignment are absent in this checkout. The existing xbabe2/out archive supplies w315–w317 root documents; the reviewed out/w318 copy supplies w318. All candidate attempt JSON and genome CSV files are under the corresponding live out/w315–out/w320 directories. Archive root and live attempt sources are reported separately; absent archive attempt copies are not claimed as verified.

Each root leaderboard and two birth/screen pairs must match the existing reviewed pins. Required schemas/types, exact same-round birth positions, assessed and error identity references, zero stored qualified counts and null champions are checked. JSON parsing rejects duplicate keys/nonfinite numbers. All indexed digest values require lowercase hexadecimal SHA-256 syntax. Attempt certificate-identity fields must match their directory/event, and every stored genome_sha256 field must match the actual associated CSV file hash. One certificate identity cannot bind conflicting genome bytes. Metadata errors fail closed without inferring physical qualification.

The source JSON inventory and all source byte hashes are checked again after scanning. Read-only runs require the saved file inventory, SHA-256 values, sizes and complete recomputed receipt to agree. --write explicitly creates/refreshes the receipt and report; new source bytes are not trusted automatically during verification. Source artifact bytes are never modified. Non-JSON solver/geometry files and Markdown scorecard bodies are outside this digest-field scan; their saved references do not imply a fresh validation of their claims.

All emitted filesystem paths are canonical repository-relative POSIX paths. --artifact-root selects a relocated saved repository without Git discovery; --repo-root selects the receipt/output checkout. Defaults use the common repository for artifacts. Missing source JSON/CSV files fail rather than being counted as verified. No simulations, native certificate generation, geometry changes, physical gate checks, structural comparisons or performance rankings are performed.

```sh
python3 experiments/rsv2_07/audit_phase20_stage2_identity_dedup.py
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-17/18/19 receipt bindings and the existing study/test-source hashes are synchronized after adding the Phase-20 tests.

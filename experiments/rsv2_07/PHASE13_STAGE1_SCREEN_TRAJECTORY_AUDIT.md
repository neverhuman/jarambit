# Phase-13 Stage-1 saved artifact audit

**PASS for artifact integrity and identifier linkage only. Phase-13 is partially completed.** The 20 available files across w315–w319 match their pinned SHA-256 digests and byte sizes. Every saved screen label links to the exact same-round birth-array position, family and certificate identity. All ten completed birth/screen pairs were checked without writing to any wave directory.

The requested structural performance statistics and interpretation of Q7 split-hand geometry are outside this audit. Blade-strength and twist analysis that evaluates weapon designs was not performed. The three requested fields (`lateral_sf`, `tip_sf`, `twist_deg_per_nm`) are also absent from every saved screen record. Their field-presence counts are zero; no substitute metrics are used and no mechanical statistics are inferred.

## Saved round inventory

| Wave | Zero-based saved round | Birth records | Pending screen records | Screen labels among c06, c07, c08 |
| --- | ---: | ---: | ---: | --- |
| w315 | 00 | 100 | 83 | c06, c07, c08 |
| w315 | 01 | 96 | 93 | c06, c07, c08 |
| w316 | 00 | 104 | 20 | none |
| w316 | 01 | 96 | 14 | c08 |
| w317 | 00 | 104 | 20 | none |
| w317 | 01 | 96 | 13 | c07, c08 |
| w318 | 00 | 104 | 49 | none |
| w318 | 01 | 96 | 41 | c08 |
| w319 | 00 | 104 | 56 | none |
| w319 | 01 | 96 | 48 | c07, c08 |

All three positions c06, c07 and c08 exist in each saved birth array. This confirms identifier-position representation only. Birth records have no child-slot label or Q7 role metadata: the checker uses the saved batch position and the producer's label format `{wave}_r{round+1:02}_{family}_c{position:02}`. These are global batch positions, not per-family indices. It does not identify which entries implement a geometry operation or interpret an absent screen row as a physical failure. Births can repeat opaque identities; a screen must still reference the correct position. The verifier does not recompute native certificate identities.

There are 996 birth records and 437 pending screen records, counted as events rather than distinct identities. `round_02_births.json` and `round_02_screen.json` are absent in all five directories. They are reported as unavailable, not validated. Round 01 labels use r02; that does not establish the existence of saved round 02. No round is generated or resumed.

## Integrity and portability

The default input root is `out/` in the common Git repository, currently `/home/ubuntu/jarambit/out`. `--out-root` accepts a relocated folder holding the saved wave directories and bypasses Git discovery. w315–w318 are required; w319 is optional only when its directory is absent. A present directory must match its pinned artifact set. The known w315–w318 hashes reuse the Phase-12 pins; w319 hashes pin the saved bytes observed for this task.

The read-only verifier checks strict JSON parsing, required identifier fields, SHA-256 syntax and file size, round numbers, birth provenance, CSV header/row width, repeated-identity bindings, round contiguity, screen-label wave/round/family, exact birth-position linkage, duplicate screen labels/slots, and pending Stage-1 status. File inventory and all file bytes are checked again after validation. A changed file, missing pair, mismatching identifier, or newly appearing unpinned round fails closed. If a new round completes later, its pins must be explicitly reviewed before it can pass. This is a fixed saved-artifact audit, not a monitor that trusts new bytes automatically. Other directory files and Stage-2 qualification are outside scope.

## Pinned saved bytes

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| w315/round_00_births.json | 183491 | bc9441f863812257b119520370563da8cfe606fe56d76c8649ce3077838ce0d2 |
| w315/round_00_screen.json | 34524 | 4ed6def0fb861038ddc7c8184c08c824325531f2830de41c3772abe0046e1689 |
| w315/round_01_births.json | 175784 | e149fae3512408544de0205c562d995838aa166f0c4f64386a00dd40b961863c |
| w315/round_01_screen.json | 38662 | a45a5f4559b64702c9cf77e71e5f734e14af996ce1b20e34b4afde1523137698 |
| w316/round_00_births.json | 180331 | c1ce1969c2cee585db9cde609acd957635398acfc227f57c595bf08c0415db14 |
| w316/round_00_screen.json | 8172 | 1b5cb4d4ff4504bcd073b0fd1276a5a3887ddbb06447a91a2ad2c478921ed56e |
| w316/round_01_births.json | 167965 | a4fc583720d23a8fc0876ea398dde7d862d261a8199bfe496476573acec4ffe8 |
| w316/round_01_screen.json | 5647 | 2a7a0f86ac92ea7e983179761b639ef7e7bf585274d96cee8d81cccfb589b872 |
| w317/round_00_births.json | 179197 | aa4f9c4c0a888a9fb723ccd1245bb0a8c10d33be7743767ab53c71edbe73290c |
| w317/round_00_screen.json | 8172 | 1895c91af815e83e87e5d820705f84537327a50a493b37a7bfdf98b41b8de240 |
| w317/round_01_births.json | 166685 | b5df4627717dc2b76fc0b2f7ad18f92307b9ba82c268ed792d78ea7730efdda8 |
| w317/round_01_screen.json | 5244 | afd1e87a79c21f6996df5b0774df4b9653a413dafe2a7d83a2bb9b5c65822663 |
| w318/round_00_births.json | 176486 | 294c9d71e64d0963c24c71fcd8e76f9ec6b8e08bd5089bb8c78c44dfc4e3d75f |
| w318/round_00_screen.json | 20298 | 2416936cd05b93217ed8912fc27d5cc8b9308ce0fe04e69fd35278daacded1d8 |
| w318/round_01_births.json | 164034 | f384d9fc60eaad7d2880e47c8ae467e9c42a13688050d0ffa80331b3736f2f21 |
| w318/round_01_screen.json | 16949 | 243404dc45c5903b03a2ea3ddf476a5b0f6eeee16e38a7e8b24832ac81e53b28 |
| w319/round_00_births.json | 174410 | 8ac6de8f7f19cc83c4c9d7f871006e3455300d5ffe71c3f51bedfeb66cd0167c |
| w319/round_00_screen.json | 23223 | 067756640f297d949b7f8f0aa898e3100ce8f61f8070ad2307e9542c7ecf575d |
| w319/round_01_births.json | 162120 | a8098af848a7f65cf6f107e732bc888ace0b06bed7433af0749ed39776f12fee |
| w319/round_01_screen.json | 19893 | af58989acf8b6f9f9c27dcac34f5476eaec73d0cc5adb1630818f1ba8bd34a3a |

## Validation

The 11 new synthetic unit tests cover all three round pairs, missing rounds, unscreened positions, field-presence-only reporting, byte tampering, malformed pins, unpinned files, an identity present at the wrong birth position, wrong wave/round/family labels, duplicate labels, incomplete/gapped rounds, conflicting bindings, malformed schema, pending status, relocated directories without Git, absent optional waves, missing required waves, and inventory/byte mutation during verification. No real wave data is required by the new tests. The complete suite contains 51 tests.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase13_screen_rounds.py
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

The launcher already supplies niceness 10 and affinity 14,15; commands preserve these settings (adding another `nice -n 10` would increase niceness again). No builds, simulations or geometry changes are performed. Phase-10's test-file binding and Phase-7/8 study manifests are refreshed for the changed/new audit sources and report. Original numerical receipts and wave artifacts remain unchanged. No commit, push or PR is made.

Validation evidence: `target/rsv2-07/phase13-stage1-artifact-integrity/20261018T121524Z/summary.json` and adjacent command logs.

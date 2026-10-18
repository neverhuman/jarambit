# Phase-17 portable schema and SHA-256 registry audit

**PASS for schema and registry integrity.** The confirmed inputs are the existing Phase-10 JSON manifest and Phase-11–16 Python pin registries. No seven historical JSON receipts are invented or renamed.

| Phase | Profile | Source | References |
| --- | --- | --- | ---: |
| 10 | json_manifest | experiments/rsv2_07/results/phase10_portable_receipt_manifest.json | 204 |
| 11 | python_pin_registry | experiments/rsv2_07/audit_phase11_wave_receipts.py | 5 |
| 12 | python_pin_registry | experiments/rsv2_07/audit_phase12_cross_wave.py | 20 |
| 13 | python_pin_registry | experiments/rsv2_07/audit_phase13_screen_rounds.py | 20 |
| 14 | python_pin_registry | experiments/rsv2_07/audit_phase14_w319_receipts.py | 10 |
| 15 | python_pin_registry | experiments/rsv2_07/audit_phase15_gate_s_and_margins.py | 25 |
| 16 | python_pin_registry | experiments/rsv2_07/audit_phase16_xbabe3_waves.py | 63 |

The 347 references resolve to 328 unique repository-relative paths, with no digest or known-size conflicts. This run verified 328 paths on disk and recorded 0 as manifest-only. Every available checkout/supplemental copy must match; an absent copy never overrides a mismatching present file.

The JSON profile requires the existing manifest identity, map fields, nonnegative integer counts and raw-artifact digest/size records. Registry profiles require nonempty wave maps and digest/size pairs. SHA-256 values must be 64 lowercase hexadecimal characters. Paths must be canonical relative POSIX paths without traversal, drive prefixes, backslashes or control characters. Boolean sizes/counts, duplicate paths within one phase, conflicting hashes/sizes, duplicate JSON keys and nonfinite JSON numbers fail closed.

Phase-12 w315–w317 registry paths retain their xbabe2 archive locations; Phase-16 paths retain the xbabe3 archive location. Other wave pins use their saved out paths. Source hashes bind the seven manifest/module files. Referenced study inputs require current checkout bytes; wave and target artifacts may be manifest-only when absent. Supplemental roots are supplied at execution and never persisted as absolute paths.

The JSON receipt records the original availability observations. Read-only verification requires its source hashes, profiles and merged registry to match current inputs and validates the recorded observation schema/counts. It then independently checks current file availability and bytes; clean checkouts can have fewer available artifacts. Historical on-disk observations do not establish that absent bytes were freshly verified. Source hashes are integrity bindings, not authenticity signatures.

Earlier phase scope limits remain in force. No simulations, native evaluations, geometry changes, physical gate checks, structural margin calculations or qualification are performed. Python modules are imported only to read their existing registry objects; their verifier entry points are not called.

```sh
python3 experiments/rsv2_07/audit_phase17_portable_schema.py
python3 experiments/rsv2_07/audit_phase17_portable_schema.py --artifact-root saved_repository
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
```

The command is read-only unless --write is supplied. --repo-root and --artifact-root support relocation without Git discovery. The only persistent paths are repository-relative. A write explicitly refreshes this report and the Phase-17 JSON receipt; it does not modify source receipts or artifacts.

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-17 synthetic tests cover schema/type failures, path portability, conflicts, tampering, missing evidence, recorded receipt integrity and a relocated CLI without Git or original host paths. The full discovery suite and git diff --check are required before publication.

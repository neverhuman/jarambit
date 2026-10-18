# Phase-10 portable receipt integrity audit

**Validation: PASS.** Three receipts, 812 digest references, 8 tracked input files and 196 on-disk raw artifacts passed verification. All 20 regression tests passed on CPUs14,15 at nice10, including clean-checkout simulation.

This audit checks existing receipt bytes, JSON structure, digest syntax and report transcription. It does not run simulations, recalculate engineering acceptance criteria, or establish physical qualification.

| Receipt | Schema | SHA-256 | Raw artifact entries |
| --- | --- | --- | ---: |
| phase6_unified_scorecard.json | jarambit.rsv2_07.phase6_unified_scorecard.v1 | 5d301a3b33afdd8470441c0094f79e47915f8f9e1161af035b90ac37223f7dd7 | 164 |
| solid_tip_3d_validation.json | jarambit.rsv2_07.solid_tip_3d_validation.v2 | 0b98204a73501e3c11683b6b362972ea162d693a0a63bdcb01596e3b5cae1cfc | 32 |
| validation.json | legacy.validation.v1 (structural) | d7d0c4f55aaf89a8650055606bc4c7cf13412b59357aff246521eeb88570f508 | 0 |

`validation.json` is a legacy receipt without a schema identifier. Its documented structural profile checks required keys and types; no schema field has been added to the historical file. The other two receipts retain their versioned schema identifiers. JSON parsing rejects duplicate keys, NaN, infinities and overflowing numeric literals. SHA-256 references must contain exactly 64 lowercase hexadecimal characters. The historical boolean `production_surface_matches_3d_source_sha256` is checked as a flag rather than a digest.

The Phase-6 report's overview labels/counts and both stored gate grids (26 rows) must match the JSON transcription exactly. This does not recompute gate decisions or physical margins. Its remaining tables and prose are protected by the original report digest. The inventory above must match all three receipt files and their root artifact-map entry counts.

The companion `results/phase10_portable_receipt_manifest.json` binds the three receipts, both reports, verifier, tests and shared portability helper. Its 196 raw artifact records retain their original receipt digests and measured byte sizes, recorded only after verifying the existing bytes. The manifest is controlling metadata, not a circular self-hash or an authenticity signature. Phase-7/8 study manifests also cover the new tracked files and updated tests.

Each present raw file must match its original digest and recorded byte size. In a clean checkout, absent `target/rsv2-07/**` files are explicitly counted as **manifest-only** and require valid digest/size metadata. This does not verify absent bytes. Tracked receipt and report files always require exact on-disk hashes; no historical-source fallback applies to them. Nested source and external-input digests are checked for format, not claimed to be fresh validation of external files.

The default command is read-only, uses only the Python standard library, and imposes no CPU affinity restriction. The test suite covers clean-checkout simulation, receipt and table tampering, required keys/types, malformed digests/JSON, incomplete raw manifests and existing Phase-7/8 portability cases. It does not rename or remove saved evidence.

From a normal-priority shell:

```sh
nice -n 10 taskset -c 14,15 python3 experiments/rsv2_07/audit_phase6.py
nice -n 10 taskset -c 14,15 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
```

This session already inherits nice10 and CPU affinity14,15; verification preserves those settings and CARGO_BUILD_JOBS=2. No builds are needed. The three original JSON receipts and Phase-6 report remain byte-for-byte unchanged.

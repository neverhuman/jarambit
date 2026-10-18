# Phase-4 unified prototype checkpoint

This archive preserves the completed `G16-12-RSV2-04-p4` evaluation. It creates no new design and changes no production source or engineering threshold. See `UNIFIED_PROTOTYPE_REPORT.md` for all results and limits.

- Three combined bevel variants completed all four native FEM areas: 2.0, 0.25, 0.125 and 0.0625 mm².
- Five complete witness-control evaluations reproduce the source receipts within 1.3e-13 relative scalar error.
- Four complete native actual-export measurements include the primary 0.025 mm grid.
- No combined candidate passes all assignment targets. Best two-mesh human-max twist/spine factors are 0.386575096 / 0.565685833. Each variant misses the requested sequence margin 1.25 on one fine mesh. All four exports fail the 25-degree sharpness limit at 28.5434571059 degrees; point profile, bore radius and watertightness pass.

## Verification and provenance

Run `python3 experiments/G16-12-RSV2-04/phase4/verify_archive.py` from the repository, or execute the verifier from any directory. It checks the package seal and each preserved original artifact against its original seal. The report is byte-identical to the original report, SHA-256 `7db2bf57581528e6e87f5d400e2bebcdc992e50bfa654fb90d09b8dd69204432`.

`unified_study/ORIGINAL_SHA256SUMS` is the original 95-artifact seal. Ninety-four original artifacts are archived byte for byte. The generated `study` executable stays in the original worktree's `target/` and is omitted from git; its digest remains in the original seal, `ARCHIVE_PROVENANCE.json` and `unified_study/run_provenance.json`. No compiler output is committed. `SHA256SUMS` is a separate complete seal of this tracked package.

The scientific runs used baseline source commit `21425fe2948ab3941eedbe9710ebc3d29427faae`, CPUs 10,11, `CARGO_BUILD_JOBS=2` and explicit faer. The checkpoint is packaged on the lane's current parent `d046093`, without replaying the study or changing its results. Original absolute paths and source/commit provenance are preserved in the sealed bytes. The copied neck config's skyline/retained-faer discrepancy and the stopped partial skyline control are documented explicitly.

## Reproduction scope

The archived harness retains the original worktree paths and expected production-source digests. Its original reproduction entrypoint is `unified_study/reproduce.sh`, executed in `/home/ubuntu/jarambit_wt_diag` against the matching baseline source and existing lane release dependencies. The generator fails if production sources differ from its snapshot. Archive verification does not rebuild, launch solvers, create a worktree, switch branches or alter current main.

`HANDOFF_SNAPSHOT.md` and `CHANGELOG_SNAPSHOT.md` preserve the local tracking records at checkpoint preparation. Further scientific work remains on hold for the captain. This checkpoint opens no PR, pushes no branch, deletes nothing and asserts no all-hard qualification.

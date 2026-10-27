# WITHDRAWN: the v4.0.0 G4 prototype (not a result)

v4.0.0 was withdrawn in v4.0.1 by owner decision (2026-10-27). The files under this folder stay unchanged, as the record of what
v4.0.0 contained. They are an experimental prototype, not a design or result of record. The current design of record is the
G3 v3.0.0 cohort's best individual, `hz4i013_g00995_d206e3c9` (`evidence/g3/v3.0.0/release.json`).

Why it was withdrawn (details in `docs/RELEASES.md`, v4.0.1):

- **Not an evolved design.** Every initial population, including the "Sobol" baseline, was seeded with the hand-authored
  defaults of `genome/G4.0.toml`, and a hand-coded rule set pushes named genes toward chosen values. The winner
  `g4v400_s042_62763aa2` lies within a median of 0.031 (normalised units) of those defaults, after 254 evaluations per run.
- **Not under the owner's loads.** It uses a 0.80 N·m tip twist where RS-karambit-v3 has 1.82 N·m. It judges the 3,427 N
  spine case on ultimate strength rather than first yield, and assumes HIP material. Its "certified" status refers to these
  changed requirements (`requirements/RS-karambit-v4.toml`, `loads-v4.toml`, `materials-v4.toml`, `frame-v4.toml`,
  `PROVENANCE.md`, which belong to this prototype).
- **Against the owner's rules.** It uses hard gates (a feasibility-first comparator) and a Lamarckian repair emitter, where the
  owner requires a continuous objective with no gates and a design produced by evolution alone.

The genuine G4 is planned as v4.1.0: Sobol-only starts, no hand design, every RS-karambit-v3 load value kept, and a continuous
objective. Parts of this prototype may be reused there only after review.

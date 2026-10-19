# Engineered Intelligence — Admiralty phase and task ledger

**Authority:** latest direct human instruction, 19 October 2026. Admiralty owns execution as the primary Jarambit push; Codex reviews/advises. **All CI is deferred until P8.**

[Detailed engineering specification](ENGINEERED_INTELLIGENCE_PUBLICATION_AND_RUN_HISTORY_PLAN.md) · [Machine-readable ledger](ENGINEERED_INTELLIGENCE_PHASE_TASKS.json)

This ledger has **9 phases and 60 leaf tasks**. IDs below are logical import keys, not claims that Admiralty objects already exist. Admiral must return actual workplan/task IDs, assigned workers, path leases and first dispatch proof. The JSON registry is the canonical task/dependency contract.

## Execution rules

- P0–P7: implement and author tests; skip all CI, suite execution, CI cleanup and CI-triggering development pushes/PRs. Artifact-production compilation/rendering/hashing may proceed. Use local/internal commits and bundles.
- P8: freeze, run all deferred validation, fix findings, run required CI, independently review the exact head, then merge. Preserve workflows/protections.
- One actual worker per write lease; one integration steward. Disjoint work may run in parallel.
- `implemented_unvalidated` is not `validated` or `done`. Deferred checks never count as passed.
- Preserve original evidence, active/unmerged/untracked work, and unrelated jobs during session cleanup.
- Recovery/controller readiness does not mean a live evolution campaign was restarted.

## Phase overview

| Phase | Purpose | Depends on | Leaf tasks |
|---|---|---|---:|
| P0 | Admiralty takeover, CI hold and obsolete-session cleanup | — | 6 |
| P1 | Truth admission and retained-corpus inventory | P0 | 7 |
| P2 | Public identity, selection and full-population projections | P1 | 8 |
| P3 | Paper argument, methods and supplement | P0, P2 | 6 |
| P4 | Scientific visual master, crops and Blender hero | P2 | 7 |
| P5 | Compact journals, storage policy and archive publisher | P1 | 6 |
| P6 | Generic production recovery and four-worker readiness | P5 | 7 |
| P7 | Shared README, viewer and integrated showcase artifacts | P2, P3, P4 | 6 |
| P8 | Final validation, independent review and integration | P7, P6 | 7 |

## P0 — Admiralty takeover, CI hold and obsolete-session cleanup

Adopt the primary push in the actual system, transfer ownership, preserve prior work and dispatch real workers.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P0-T01 | **Create/adopt the Admiralty primary workplan**<br>Admiral | — | Actual workplan ID and task import mapping<br>All logical IDs map to actual Admiralty objects; no fabricated dispatched/done claim. |
| JR-EI-P0-T02 | **Acknowledge CI-last policy and assign integration steward**<br>Admiral | JR-EI-P0-T01 | Named steward, CI hold and final-phase trigger<br>P0–P7 have no CI dispatch/triggering PR or push; final validation remains P8. |
| JR-EI-P0-T03 | **Ingest source, local foundation and immutable review inputs**<br>Integration steward | JR-EI-P0-T01 | Verified base, bundle 9738824 and input hashes<br>Preserve current main and unrelated work; record any conflicting newer source. |
| JR-EI-P0-T04 | **Inventory old screens/sessions and their task ownership**<br>Admiral operations | JR-EI-P0-T01 | KEEP/REASSIGN/RETIRE inventory with session/window/process/cwd/worktree/lease<br>Capture transcripts, git dirty/untracked/unmerged state and active child writers; protect live coordinators/unrelated jobs. |
| JR-EI-P0-T05 | **Safely retire released obsolete sessions and supersede old assignments**<br>Admiral operations | JR-EI-P0-T04<br>JR-EI-P0-T03 | Per-session retirement result and retained work pointers<br>Only obsolete/released sessions retired; active writers/pending work preserved; unclear ownership routed, never blanket-killed. |
| JR-EI-P0-T06 | **Dispatch the first ready tasks through Admiralty**<br>Admiral | JR-EI-P0-T02<br>JR-EI-P0-T03 | Actual worker claims, worktree/path leases and first work receipts<br>Admiralty task/claim IDs and observable progress returned; Screen pointer alone does not count. |

## P1 — Truth admission and retained-corpus inventory

Admit an explicit finite snapshot, including interrupted histories, without inventing missing data.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P1-T01 | **Verify V2 and additive erratum linkage**<br>Archive owner | JR-EI-P0-T06 | Verified raw handoff/erratum SHA and first observation<br>Frozen V1/V2 preserved; each correction checked against source bytes. |
| JR-EI-P1-T02 | **Inventory actual roots on all four intended workers**<br>Archive owner | JR-EI-P1-T01 | Per-host root/file membership and explicit unavailable/excluded states<br>Do not report all-four readiness/completeness without verified fourth-host sources. |
| JR-EI-P1-T03 | **Capture compact aborted/interrupted histories as separate attempts**<br>Archive owner | JR-EI-P1-T02 | Aborted attempt genes/scores/journals or explicit missingness<br>Incomplete/unfavorable attempts are retained; no destructive cleanup of evidence. |
| JR-EI-P1-T04 | **Reconcile historical frames, JSON and genome CSV**<br>Reporting data worker | JR-EI-P1-T01 | Exact historical record reconciliation<br>1984 births; occurrence multiplicity, missing elite scores and pre-migration stage preserved. |
| JR-EI-P1-T05 | **Join newer births and screens using exact identity**<br>Reporting data worker | JR-EI-P1-T01 | All retained joins and no-screen records<br>One-based index plus family/content identity; missing screen is not failed FEM. |
| JR-EI-P1-T06 | **Admit native summary files with real producer/config scope**<br>Reporting data worker | JR-EI-P1-T01 | Headerless row-zero retained, gaps/NaN/source bindings recorded<br>Summaries are not births or complete genomes; missing process/binary/start state stays unknown. |
| JR-EI-P1-T07 | **Seal finite snapshot inventory and durable accession**<br>Archive owner | JR-EI-P1-T02<br>JR-EI-P1-T03<br>JR-EI-P1-T04<br>JR-EI-P1-T05<br>JR-EI-P1-T06 | Snapshot membership, source mapping and durable raw-package accession<br>Exact hashes/counts/exclusions; target working copy is not sole long-term custody. |

## P2 — Public identity, selection and full-population projections

Show all admitted records with correct score/stage and event semantics.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P2-T01 | **Allocate persistent integer public IDs**<br>Identity worker | JR-EI-P1-T07 | Append-only individual/run/island registry<br>No renumbering by rank/filter; distinguish birth/genome/appearance/artifact. |
| JR-EI-P2-T02 | **Audit selection authority and all current/best claims**<br>Identity worker | JR-EI-P2-T01 | Creation/ranking/assessment/editorial roles<br>Unknown actors stay unknown; cover remains an assessed illustration unless stronger proof exists. |
| JR-EI-P2-T03 | **Implement versioned historical population adapter**<br>Population worker | JR-EI-P2-T01 | Birth/occurrence/parent/migration/GenEnd tables<br>Duplicate slot occurrences retained; no inferred current elite fitness or migration replacement. |
| JR-EI-P2-T04 | **Generate all-historical-birth and membership figures**<br>Population worker | JR-EI-P2-T03 | Every historical birth and recorded membership visible/downloadable<br>Birth distributions, current memberships and generation maxima remain distinct. |
| JR-EI-P2-T05 | **Generate complete newer-event and latest-snapshot panels**<br>Population worker | JR-EI-P1-T07<br>JR-EI-P2-T01 | All admitted newer events and all retained native generations<br>Unknown data band, snapshot bounds and distinct stages/scorers; no historical-only substitute. |
| JR-EI-P2-T06 | **Declare score profiles, objective priorities and display loss**<br>Reporting worker | JR-EI-P1-T07 | Source-bound profile/priority tables and per-regime progress series<br>No mixed-stage curve or retrospective redefinition of historical objective. |
| JR-EI-P2-T07 | **Generate actual inheritance and genome-difference panels**<br>Population worker | JR-EI-P2-T03 | Real family graph and recorded representation differences<br>No invented parents, causal inheritance, or geometry identities. |
| JR-EI-P2-T08 | **Seal shared counts, captions and claim/source tables**<br>Integration steward | JR-EI-P2-T02<br>JR-EI-P2-T04<br>JR-EI-P2-T05<br>JR-EI-P2-T06<br>JR-EI-P2-T07 | One admitted projection set for all consumers<br>README/paper/viewer consume same IDs, counts, roles and snapshot refs. |

## P3 — Paper argument, methods and supplement

Write the strong simple Engineered Intelligence story with technical detail and truthful results.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P3-T01 | **Rewrite title, abstract, introduction and contributions**<br>Editorial worker | JR-EI-P0-T06 | Process-first EI narrative and metadata<br>Proposed features not claimed complete; generated numbers finalized from P2. |
| JR-EI-P3-T02 | **Expand genome/decoder and actual mating/mutation explanation**<br>Editorial worker | JR-EI-P2-T07 | Genome ribbon, schema/dependencies and inheritance explanation<br>Describes implemented representation, not new physical design or operator tuning. |
| JR-EI-P3-T03 | **Write profile-specific fitness and numerical methods**<br>Editorial worker | JR-EI-P2-T06 | Weight/priority/method/observable/source subsections<br>Native conditional/contextual behavior accurately retained; no new objective. |
| JR-EI-P3-T04 | **Move exhaustive historical matrix and taxonomy to supplement**<br>Editorial worker | JR-EI-P0-T06 | Readable main methods plus 44-entry atlas crosswalk<br>All original outcomes, anchors and missing-field disclosures remain accessible. |
| JR-EI-P3-T05 | **Write full-population results and strong evidence-grounded conclusion**<br>Editorial worker | JR-EI-P2-T08 | Memorable actual findings and expansive future implications<br>No unsupported qualified/lightest/beyond-human/tolerance/scaling claim. |
| JR-EI-P3-T06 | **Reconcile agents, reproducibility, source dates and references**<br>Editorial worker | JR-EI-P2-T02<br>JR-EI-P2-T08 | Recorded agent/selection roles, availability and versioned references<br>Remove broad unavailable/current-version overclaims; separate manuscript/data/scorer/renderer revisions. |

## P4 — Scientific visual master, crops and Blender hero

Produce the requested white horizontal hero from exact retained geometry and eligible fields.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P4-T01 | **Expose one admitted read-only plate representation**<br>Field worker | JR-EI-P2-T02 | Single source of field arrays/masks/domain identity<br>No duplicate mask reconstruction or mutable evidence exposure. |
| JR-EI-P4-T02 | **Implement percentile composite and contributor coverage**<br>Field worker | JR-EI-P4-T01 | All-map mean, primitive-only sensitivity, flat reference and coverage<br>Area-weighted ties/NaN/masks handled; envelope overlap disclosed; no all-tests claim. |
| JR-EI-P4-T03 | **Bind analysis fields to exact CAD display surface**<br>Field worker | JR-EI-P4-T01 | Validated domain-aware correspondence<br>No cross-hole/sidewall/other-individual painting; unmapped areas neutral. |
| JR-EI-P4-T04 | **Implement saliency crops and exact locators**<br>Field worker | JR-EI-P4-T01 | 90%-measure crops, extrema and PIP bounds<br>Declared area/visibility measure, zero/uniform/disconnected fallbacks and real achieved coverage. |
| JR-EI-P4-T05 | **Version bounded v3 render route and active-cover admission**<br>Render worker | JR-EI-P4-T02<br>JR-EI-P4-T03 | Permitted 5400px white/field outputs with original mesh checks<br>Atomic v3 migration; preserve v2 hashes and bounded paths/resources. |
| JR-EI-P4-T06 | **Produce registered reflective, anodized and 50/50 previews**<br>Render worker | JR-EI-P4-T05 | Same-camera comparison sheet<br>Proper rigid pose, white, full silhouette, linear/alpha-correct mixture; appearance distinct from data. |
| JR-EI-P4-T07 | **Render final shared hero and localized lossless masters**<br>Render worker | JR-EI-P4-T04<br>JR-EI-P4-T06 | Final paper/web derivatives and flat reference<br>Technical source binds, locators, legends and geometry unchanged; artifact creation allowed, final suite P8. |

## P5 — Compact journals, storage policy and archive publisher

Keep genes/scores/events reproducible without routine full CAD/FEM data per individual.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P5-T01 | **Specify reusable scientific identity and compact schemas**<br>Evidence worker | JR-EI-P1-T07 | Versioned event/genome/segment/snapshot contract<br>Reuse SubjectRef/ScientificIdentity/ContentRef; separate transport/source/scientific hashes. |
| JR-EI-P5-T02 | **Implement lossless genomes and all-event records**<br>Evidence worker | JR-EI-P5-T01 | Birth-before-evaluation and exact scalar/genome recording<br>Preserve failed/invalid/unknown births, parent/selection/membership context and seed bytes. |
| JR-EI-P5-T03 | **Implement immutable bounded compression and atomic seals**<br>Evidence worker | JR-EI-P5-T02 | Hashed bounded zstd segments and index<br>No growing rewritten blob, decompression bomb, silent truncation or lost birth deduplication. |
| JR-EI-P5-T04 | **Import compact retained histories additively**<br>Archive worker | JR-EI-P5-T03<br>JR-EI-P1-T07 | Versioned score/genome/event records with explicit missing genes<br>Existing originals preserved; no guessed gene or precision recovery. |
| JR-EI-P5-T05 | **Implement one-steward durable Git publication queue**<br>Evidence worker | JR-EI-P5-T03 | Idempotent local-sealed/transferred/branch/merged acknowledgments<br>During P0–P7 use local/internal Git only; final external publication at P8; keep spool on rejection. |
| JR-EI-P5-T06 | **Measure storage growth and finalize evidence exceptions**<br>Archive worker | JR-EI-P5-T04<br>JR-EI-P5-T05 | Measured bytes/ratios/lag and selected full-evidence accession<br>Routine full CAD/FEM excluded prospectively; existing/qualification/publication evidence preserved; CI authored not executed yet. |

## P6 — Generic production recovery and four-worker readiness

Implement exact continuation where supported, distinguish warm starts, and protect prior outputs.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P6-T01 | **Extract complete native production state and phase contract**<br>Runtime owner | JR-EI-P5-T01 | Ordered full state incl RNG/novelty/fresh_ids/all/history<br>No operator/geometry/score behavior change; distinguish legacy and test-only checkpoint paths. |
| JR-EI-P6-T02 | **Implement atomic checkpoint/journal/pointer recovery**<br>Runtime owner | JR-EI-P6-T01<br>JR-EI-P5-T03 | Hash-bound durable pointer and retained previous/orphan states<br>Sync pointer file and parent directory; refuse incompatible/torn state; preserve orphan bytes. |
| JR-EI-P6-T03 | **Integrate native CLI drain and create-new/resume guards**<br>Runtime owner | JR-EI-P6-T02 | Production native safe output lifecycle<br>No truncation/NullSink silent loss for recorded runs; existing nonempty path refused; request != stop. |
| JR-EI-P6-T04 | **Implement automatic-round phase/attempt recovery**<br>Runtime owner | JR-EI-P6-T02 | RNG/elites/assessed_all/seed_parent_ids/errors/stage maxima restored<br>No respawn/overwrite of committed rounds or assessment attempts; policies unchanged. |
| JR-EI-P6-T05 | **Author production neutral replay and crash controls**<br>Runtime owner | JR-EI-P6-T03<br>JR-EI-P6-T04 | Uninterrupted/stop/resume/crash fixtures<br>Compare complete production outputs, not only final score; suite execution deferred P8. |
| JR-EI-P6-T06 | **Implement four-worker generic controller and fencing**<br>Controller owner | JR-EI-P6-T03 | Idempotent targeted lifecycle and per-worker receipts<br>Partial network/worker failures explicit; stale epochs rejected at archive admission; no broad process kill. |
| JR-EI-P6-T07 | **Author neutral controller scenarios and readiness runbook**<br>Controller owner | JR-EI-P6-T06<br>JR-EI-P6-T05 | Four-neutral-worker partition/lease/PID/retry cases<br>No live evolution restart or capability overclaim; final scenario execution P8. |

## P7 — Shared README, viewer and integrated showcase artifacts

Use one source set to make the public overview, paper and full-population companion coherent.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P7-T01 | **Build immutable full-population viewer**<br>Viewer worker | JR-EI-P2-T08 | Read-only run timeline/inspector/download/accessibility<br>All events available; no browser score/ID recomputation, silent downsampling or search launch. |
| JR-EI-P7-T02 | **Rewrite README from shared overview/asset data**<br>Publication owner | JR-EI-P2-T08<br>JR-EI-P4-T07 | EI hero/population/objective/navigation overview<br>Current/historical/leader claims audited; same assets/scope as paper. |
| JR-EI-P7-T03 | **Integrate manuscript, captions, supplement and visual index**<br>Publication owner | JR-EI-P3-T01<br>JR-EI-P3-T02<br>JR-EI-P3-T03<br>JR-EI-P3-T04<br>JR-EI-P3-T05<br>JR-EI-P3-T06<br>JR-EI-P4-T07 | Integrated showcase and 44-entry source/visual crosswalk<br>Missing fields remain explicit; no stale active-cover bindings or conflicting public titles. |
| JR-EI-P7-T04 | **Produce the 15–20-page paper and inspect all pages**<br>Publication owner | JR-EI-P7-T03 | Full built paper and visual inspection receipt<br>Build/render only; no CI suite before P8. Fix visual hierarchy rather than tiny fonts. |
| JR-EI-P7-T05 | **Join viewer, archive links and shared figure accessions**<br>Integration steward | JR-EI-P7-T01<br>JR-EI-P7-T02<br>JR-EI-P7-T04<br>JR-EI-P5-T06 | Same current artifacts and immutable data links<br>No duplicate manually maintained score table or mismatched subject/data revision. |
| JR-EI-P7-T06 | **Freeze implementation candidate and assemble review bundle**<br>Integration steward | JR-EI-P7-T05<br>JR-EI-P6-T07 | Exact final candidate head, artifact manifest and outstanding-failure list<br>All implementation tasks implemented_unvalidated; CI still deferred; release/fleet-readiness claims not yet done. |

## P8 — Final validation, independent review and integration

Run all deferred validation at the end, repair actual failures and merge the reviewed exact head.

| Task | Work and owner role | Dependencies | Deliverable / acceptance |
|---|---|---|---|
| JR-EI-P8-T01 | **Declare final validation freeze and release CI hold**<br>Admiral | JR-EI-P7-T06<br>JR-EI-P0-T05<br>JR-EI-P0-T06 | Recorded frozen head and CI-start event<br>No further silent source/artifact mutation; real final head drives validation. |
| JR-EI-P8-T02 | **Run focused reporting/archive/neutral recovery tests**<br>Validation worker | JR-EI-P8-T01 | Executed meaningful negative/positive contracts<br>All deferred suites run now; exact source, commands, results and scope recorded. |
| JR-EI-P8-T03 | **Run broad repository gates, web and publication checks**<br>Validation worker | JR-EI-P8-T02 | Required complete local gates, final PDF/links/manifest reconciliation<br>Do not remove failing tests/limits; route/fix failure and revalidate changed final head. |
| JR-EI-P8-T04 | **Open final integration PR and run required CI**<br>Integration steward | JR-EI-P8-T03 | Actual PR URL/head and required workflows<br>No skip-CI marker or disabled branch protection; no inherited previous-head green. |
| JR-EI-P8-T05 | **Obtain independent exact-head review and repair findings**<br>Independent reviewer | JR-EI-P8-T04 | Actual independent approval on final head<br>Never self-approve; new commit requires appropriate re-review/revalidation. |
| JR-EI-P8-T06 | **Merge and verify published artifacts/data**<br>Designated merger | JR-EI-P8-T05 | Merge SHA/tree, principals and artifact/Git data checks<br>Required CI green, exact-head match and true completion scope; main-push state separate. |
| JR-EI-P8-T07 | **Record milestone completion and retire obsolete monitors**<br>Admiral | JR-EI-P8-T06 | Final paper/archive/recovery readiness receipt<br>Do not call a real fleet restart done unless separate actual receipts prove it; retire timer only when agreed scope complete. |

## Adoption receipt required from the Admiral

1. Actual Admiralty workplan ID and logical-to-system task mapping.
2. Named integration steward and worker claims/write leases; current root/base/head.
3. CI hold accepted and P8 final validation trigger.
4. Screen/session inventory and concrete KEEP/REASSIGN/RETIRE outcomes, with preserved-work pointers.
5. First actual ready-task dispatch and progress artifact; do not report dispatched from a message alone.
6. A durable progress location for Codex read-only review and the next bounded task.

If the Admiralty runtime cannot yet ingest or claim this work, name the exact missing capability and its owner. Do not silently substitute loose Screen work and call it Admiralty-managed. Continue source/ownership preservation and clear the runtime blocker.

**Scope mapping:** detailed plan sections 2/15/18 → P0; 3/4 → P1; 7/8/9 → P2; 5/20 → P3; 10/11 → P4; 12 → P5; 13 → P6; 6/14 → P7; 16/17/18/19 → P8 and cross-cutting acceptance. Every legacy packet EI-00/EI-01..06/ARC-01..02/REC-01..02 remains described in the master spec and is decomposed here.

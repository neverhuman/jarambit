# Publication coverage and provenance audit

Audit baseline: committed main
`253944829cb58e262a16b13f4a72ff9adc745425` (17 October 2026, after #91/#92).
This is a document-packaging checklist. Section and asset presence does not
establish scientific validity, certification, or visual quality.

| Requested publication component | Coverage at the audit baseline | Remaining gap |
| --- | --- | --- |
| Individual entries grouped by tested property | [White paper §IV-B](WHITEPAPER.md#iv-b-engineering-gates-implementation-derived-atlas): six groups, 16 numbered individual gate subsubsections (`1`–`16`), and the 10-group `RS-karambit-v2` hard-gate table (§IV-B.7); the [companion §II–§III](ENGINEERING_GATES_AND_EVOLUTION.md) contains the full derivations. | This inventory does not reassess the scientific content. |
| Historical observations | [White paper §IV-D](WHITEPAPER.md#iv-d-evolution-mechanics-and-measured-population-history) links seven population-history figures, the certifiable-pipeline figure, the plateau-history figure, and the completed automatic-wave history figure (`fig_wave_history.png`), all with committed input ledgers. | Some historical population telemetry was not retained (`w301`–`w308` and completed `w300`). The paper discloses those omissions; runtime-only observations are outside this audit. |
| Per-entry image coverage | Sixteen existing Blender plates have committed manifests and are embedded inline in both papers. | These depict the labeled assessed example, not a certified winner. |
| Final-design views | Three existing illustrative views (`hero_oblique.webp`, `hero_reverse.webp`, `hero_section.webp`) have committed manifests and are embedded inline in both papers. | The [certified-winner slot](WHITEPAPER.md#iv-e-certified-winner-re-render-slot-and-assessed-candidate-studio-views) is empty. No certified winner is present in the baseline evidence. |
| PDF publication | [Build command and provenance controls](../ops/render/paper/build_pdf.py) are available; PDF outputs belong in the lane's `target/paper/`, rather than Git. | Regenerate PDFs for each publication head and retain their build manifests. A previous-head PDF does not validate a later head. |
| Catalog denominators | README and paper both identify 23 subjects × 18 evaluators = 414 cells, with 0/23 RS-v2 all-hard passes. | Keep the catalog denominator separate from later automatic assessments. |

## Reproduce the committed asset inventory

The generic [inventory command](../ops/ci/publication_inventory.py) resolves its
revision once, then reads documents, assets, manifests, generating scripts and
manifest inputs from that commit's Git objects. Uncommitted runtime files and
dirty working-tree copies cannot supply missing publication inputs.

```sh
mkdir -p target/paper
python3 -B ops/ci/publication_inventory.py \
  --ref HEAD \
  README.md docs/WHITEPAPER.md docs/ENGINEERING_GATES_AND_EVOLUTION.md \
  > target/paper/publication-inventory.json
```

The report contains commit and document hashes, ATX heading counts, inline image
counts, and each available image manifest's identity and input verification.
Existing manifest hashes must match their committed inputs and outputs. Images
without adjacent manifests are listed explicitly; remote images are counted
without fetching them. Fenced examples are excluded. Reference-style and HTML
images are outside this command's scope.

With the completed automatic-wave history figure and inline studio views, the
white paper references 51 unique local inline images (34 with adjacent
manifests), and the companion references 31 (all 31 with adjacent manifests).
Seventeen older white-paper images and the README's market-landscape image lack
adjacent figure manifests. This is a coverage disclosure, not a new manifest
requirement: the repository's existing figure gate checks its required set, and
all 51 committed figure manifests pass that gate. This inventory does not create
retroactive provenance for legacy assets.

Use the normal generated-document and figure checks alongside this inventory:

```sh
python3 -B ops/ci/gen_readme.py --check
python3 -B ops/ci/gen_whitepaper.py --check
python3 -B ops/ci/verify_figures.py
python3 -B -m unittest ops.ci.tests.test_publication_inventory -v
```

The [neutral regression controls](../ops/ci/tests/test_publication_inventory.py)
exercise dirty working copies, uncommitted assets, stale input and image hashes,
revision pinning, missing sidecars and repository-path containment. They execute
no physical models, solvers or renderers.

## Source-file identities before publication

The generic [source ledger](../ops/ci/source_ledger.py) records each named file's
repository-relative path, byte length and SHA-256. It reads Git blobs from one
resolved commit, with no fallback to uncommitted files:

```sh
python3 -B ops/ci/source_ledger.py --ref origin/main \
  docs/VERSION docs/PUBLICATION_READINESS.md > target/paper/source-ledger.json
```

For a local inventory, use `--runtime-root /path/to/local/run` instead of `--ref`
and pass paths relative to that root. The report explicitly labels those bytes
`local_files_unchecked_against_git` and sets `source_commit` to null. It records
each file as read; it does not establish an atomic snapshot across changing files.
A hash does not preserve the source bytes or make unavailable results reproducible.
Commit the original sources before citing them as committed paper inputs.

Neither mode interprets result fields, computes statistics, or validates scientific
claims. The [neutral controls](../ops/ci/tests/test_source_ledger.py) exercise exact
byte identities, dirty copies, missing sources, moving refs, mid-read substitution,
path containment and CLI failure behavior.

# Engineered Intelligence: Tracing Computational Design from Search to Evidence

[Main PDF](main.pdf) · [Standalone supplement PDF](supplement.pdf) · [Source/evidence crosswalk](CROSSWALK_SUPPLEMENT.md) · [Publication contract](publication.v2.json)

This research draft presents a retrospective account of representation, search context, retained coverage, and selected numerical evidence. Author approval remains pending. Physical observations are not reported in this study.

The opening introduces uneven evidence coverage; ancestor reuse and a historical later-assessment disagreement; and the distinct conclusions supported by shared-mesh solid agreement and refused plate extrapolation. The original twelve section files remain separate, with study design and findings introduced earlier.

## Figures and identities

The white hero is an exact CAD projection of **Individual 2000**, colored by the mean of available, pooled area-weighted ranks from four retained response channels. The [roster](data/test-field-roster.v1.json) accounts for missing, scalar-only and incompatible observations; 52 metadata rows do not mean 52 independent tests. The [display contract](data/display-composite-contract.v1.json) separates this dimensionless index, contributor support and color transfer from physical-unit fields and **Individual 2002's** weighted reporting vector. Original fields, geometry, weights, thresholds, and evidence stay unchanged.

Individual 2000 is a stable reporting alias for the retained export, not a generation or score rank. Its registry entry has no linked source birth record. The selection receipt records an editorial illustration role because the assessed bundle is retained; it does not establish a global champion or identify a particular human selector.

The [evidence-path diagram](figures/evidence-architecture.svg) separates retained search observations from candidate-specific assessed exports; it asserts no population-wide assessment join. The [compact ancestry view](figures/inheritance.svg) has a separate [detailed coordinate-change view](figures/inheritance-details.svg) in the Supplement.

The main population view processes every retained scored record through a point operation. Native, historical, and screening regimes retain separate axes; unknown proposals have a nonnumeric coverage note. Points can overlap. [Source/ID/extrema/point receipt](generated/population/point-rendering.v1.json).

The [complete row-level inventory](generated/test-field-roster.v1.tsv) preserves all 44 catalog predicates and eight channel metadata rows, including missing and incompatible spatial observations. The [support view](generated/hero-support.svg) reports contributor coverage. The selected hero uses the full [0,1] display index with mesh off; the [contrast preview](generated/hero-contrast-preview.svg) and [mesh comparison](generated/hero-mesh-on.svg) are separate alternatives. The [display receipt](generated/display-composite.v1.json) records raw ranges, normalization, per-cell contributors, palette and tail accounting. The navy-to-red palette has no monotone-luminance or universal color-vision accessibility claim.

The [score-axis receipt](generated/population/score-axis.v1.json) records each separate scale. The Supplement includes the [additional quantile detail](generated/population-detail.svg) and all five paired native-run views. They reuse the original observations. The [leader receipt](generated/population/retained-leaders.v1.json) retains exact source identities and ties within compatible regimes; it establishes no global comparable champion.

The solid comparison is scikit-fem/PyAMG CG versus DOLFINx/PETSc CG/GAMG on a shared saved mesh with cut-arc clamping and patch loads. Executed scikit-fem/PyAMG versions are unavailable; the retained sidecar identifies DOLFINx 0.9.0 and PETSc 3.22.0. [Implementation details](supplement/numerical-methods.tex).

## Reconstruct the reports

From the repository root:

~~~sh
rtk proxy sh docs/paper/reproduce.sh build
~~~

This reads retained evidence and writes reporting derivatives, PNG figures, main/supplement PDFs, and manifests under the lane's existing target/. It does not run a new search, CAD export, FEM solve, or physical experiment. The complete native input expands to approximately 3.81 GB and is streamed under the allocated resource cap.

Prerequisites: Rust/Cargo with locked dependencies, complete archives, LuaLaTeX/IEEEtran, TeX Gyre fonts, librsvg, Fontconfig, and PDF tools. The script defaults to two Cargo build jobs; the caller supplies its machine's resource allocation. Rebuild documents from existing generated figures with:

~~~sh
rtk proxy sh docs/paper/reproduce.sh documents
~~~

Display-only parameters are versioned in [display-composite-contract.v1.json](data/display-composite-contract.v1.json) and [score-axis-contract.v1.json](data/score-axis-contract.v1.json): source membership/masks, area-midrank and mean rules, palette anchors, optional contrast bounds, and axis/quantile rules. The renderer checks these admitted definitions; changing a display definition requires a matching reviewed source/contract revision. The build retains release-mode overflow checks and debug assertions.

Final validation uses the check subcommand after the recorded freeze, together with all required repository gates and independent exact-head review. An implementation build does not establish that those checks have passed.

## Source closure and accessions

The standalone supplement driver includes detailed representation, score, numerical-method, cohort and literature fragments plus an active assembly copy of the historical supplement. The active assembly corrects heading hierarchy, navigation and captions; the original historical source remains unchanged.

The build records actual LuaLaTeX inputs, active SVG/crop/hero sources, reporting source/contracts, TeX and Fontconfig font files, and toolchain under target/ieee/paper/ in INPUT_FILES, SOURCE_SHA256SUMS, OUTPUT_SHA256SUMS, and toolchain.txt. SOURCE_DATE_EPOCH defaults to the retained evidence baseline's commit timestamp for deterministic PDF metadata; actual build UTC belongs to the execution receipt. Original producer identities remain in their contracts. The final publication receipt must bind matching manuscript/build/delivery accessions; prior evidence snapshots are not relabeled as this revision.

The [crosswalk](CROSSWALK_SUPPLEMENT.md) provides artifact navigation, masks/support definitions, historical battery details, and producer/report/manuscript/PDF identity distinctions. Main/supplement page inspection and working immutable access links remain final acceptance requirements.

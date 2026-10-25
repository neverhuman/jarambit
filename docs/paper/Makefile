ROOT := ../..
OUT := $(abspath $(ROOT)/target/ieee/paper)
export CARGO_TARGET_DIR := $(abspath $(ROOT)/target)
export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS := true
export CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS := true
CARGO ?= cargo
SOURCE_DATE_EPOCH ?= 1792460432
GEN_FIGURES := population fitness plate-fields verification assessment battery-matrix weighted-runs reference hero-support hero-contrast-preview population-detail native-runs-2-3 native-runs-4-5 native-runs-6-7 native-runs-8-9 native-runs-10-11 hero-appearance hero-appearance-unlit hero-full-range plate-smart-crops objective-priorities.v1
STORY_RUNS := 02 03 04 05 06 07 08 09 10 11
STORY_VIEWS := asinh positive-affine positive-gap
STORY_NAMES := $(foreach run,$(STORY_RUNS),$(foreach view,$(STORY_VIEWS),run-$(run)-$(view)))
STORY_PREFIXES := $(addprefix generated/population/story/,$(STORY_NAMES))
STORY_SVGS := $(addsuffix .svg,$(STORY_PREFIXES))
STORY_METADATA := $(addsuffix .v1.json,$(STORY_PREFIXES)) generated/population/story/index.v1.json
POSITIVE_SCOPES := $(addprefix run-,$(STORY_RUNS)) historical screening
POSITIVE_NAMES := $(foreach scope,$(POSITIVE_SCOPES),$(foreach view,affine gap,$(scope)-positive-points-$(view)))
POSITIVE_PREFIXES := $(addprefix generated/population/positive/,$(POSITIVE_NAMES))
POSITIVE_SVGS := $(addsuffix .svg,$(POSITIVE_PREFIXES))
POSITIVE_METADATA := $(addsuffix .v1.json,$(POSITIVE_PREFIXES)) generated/population/positive/index.v1.json
POP_FIGURES := population-latest inheritance evidence-architecture inheritance-details
HERO_SVG := drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.svg
CROP_SVG := drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.svg
HERO_PNG := drafts_p3_p5/JR_EI_P4_T01_blender_exact_cad_hero_white.png
README_HERO_PNG := figures/hero-appearance.png
CROP_PNG := drafts_p3_p5/JR_EI_P4_T04_fem_saliency_crops_and_locators.png
REPORT_CONTRACTS := $(sort $(wildcard data/*.json))
REPORT_METADATA := generated/display-composite.v1.json generated/test-field-roster.v1.tsv generated/population/score-axis.v1.json generated/population/retained-leaders.v1.json generated/population/point-rendering.v1.json generated/reporting-interpretation.v2.json generated/hero-appearance.v1.json generated/plate-smart-crops.v1.json generated/objective-priorities.v1.json $(STORY_METADATA) $(POSITIVE_METADATA)
DISPLAY_ALTERNATIVES := generated/hero-full-range.svg generated/hero-mesh-on.svg
REPAIR_HISTORY := history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE102C_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE510713_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_D3BA882D_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_2A3B1A6_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_2A2CD68_20261020.json history/ultralight_weighted_fitness_009f5ac5.source.txt history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_DD92248_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_7A51C1B2_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_73FFEF00_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_DBC4034_20261020.json history/hero-appearance-source-recipe.v1.BASE_6C49E1C_20261020.json history/hero-appearance-preflight.v1.BASE_6C49E1C_20261020.json history/hero-appearance-pin-review.v1.BASE_6C49E1C_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_6C49E1C_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_BFD49EC_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_26AEDA0_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_4213DA0_20261020.json history/hero-appearance-source-recipe.v1.BASE_4FF1CE1_20261020.json history/hero-appearance-preflight.v1.BASE_4FF1CE1_20261020.json history/hero-appearance-pin-review.v1.BASE_4FF1CE1_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_4FF1CE1_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_6AE781B_20261020.json history/hero-appearance-source-recipe.v1.BASE_4C71F76_20261021.json history/hero-appearance-preflight.v1.BASE_4C71F76_20261021.json history/hero-appearance-pin-review.v1.BASE_4C71F76_20261021.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_4C71F76_20261021.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_2E46025_20261021.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_E07606A_20261022.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_7AAF6D4_20261022.json
GEN_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(GEN_FIGURES)))
# Main-page figure of each variant: false-colour render + 2.5x isogrid-pocket inset + MPa key + badge.
COVER_PNGS := figures/cover/cover-unsmoothed.png figures/cover/cover-smoothed.png
# Main-page figure of variant A: the surface-tension-smoothed design with every retained plate
# response stacked (pocket-figures writes hero.svg and hero-stack.png).
POCKET_PNGS := figures/pocket/hero.png
POP_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(POP_FIGURES)))
STORY_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(STORY_NAMES)))
POSITIVE_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(POSITIVE_NAMES)))
# Pinned evidence-baseline timestamp; actual build UTC belongs to the receipt.
TEX_ENV = TEXINPUTS="$(OUT):" SOURCE_DATE_EPOCH=$(SOURCE_DATE_EPOCH) FORCE_SOURCE_DATE=1 TZ=UTC
.PHONY: all artifacts figures pdfs main supplement audit audit-quick audit-full review-origin-controls check clean smoothing-ladder ladder-data ladder-figures ladder-pdf pocket-figures
# A report must finish generating before its figure/document consumers start.
.NOTPARALLEL:
all: artifacts pdfs
# Reporting reconstruction only; no CAD, FEM, search, tests or CI.
artifacts:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example score_decomposition
figures: $(GEN_PNGS) $(POP_PNGS) $(STORY_PNGS) $(POSITIVE_PNGS) $(README_HERO_PNG) figures/fig1_hero_blender_v2.png $(CROP_PNG) $(COVER_PNGS) $(POCKET_PNGS)
$(COVER_PNGS): figures/cover/cover-%.png: figures/cover/cover-%.svg figures/cover/render-%.png figures/cover/inset-%.png
	rsvg-convert -w 2000 -h 850 -o $@ $<
$(POCKET_PNGS): figures/pocket/%.png: figures/pocket/%.svg figures/pocket/hero-stack.png
	rsvg-convert -w 2000 -o $@ $<
$(OUT):
	mkdir -p $(OUT)
$(GEN_PNGS): $(OUT)/%.png: generated/%.svg | $(OUT)
	rsvg-convert -w 3600 -o $@ $<
$(POP_PNGS): $(OUT)/%.png: figures/%.svg | $(OUT)
	rsvg-convert -w 3600 -o $@ $<
$(STORY_PNGS): $(OUT)/%.png: generated/population/story/%.svg | $(OUT)
	rsvg-convert -w 3600 -o $@ $<
$(POSITIVE_PNGS): $(OUT)/%.png: generated/population/positive/%.svg | $(OUT)
	rsvg-convert -w 3600 -o $@ $<
$(README_HERO_PNG): $(OUT)/hero-appearance.png
	cp $< $@
$(HERO_PNG): $(HERO_SVG)
	rsvg-convert -w 3200 -h 1360 -o $@ $<
figures/fig1_hero_blender_v2.png: $(HERO_PNG)
	cp $< $@
$(CROP_PNG): $(CROP_SVG)
	rsvg-convert -w 3600 -o $@ $<
# Main and supplement share cross-document labels.
pdfs: figures
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/main-pass1.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/main-pass2.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) supplement-main.tex > $(OUT)/supplement-pass1.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) supplement-main.tex > $(OUT)/supplement-pass2.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/main-pass3.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/main-pass4.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) supplement-main.tex > $(OUT)/supplement-pass3.log
	cp $(OUT)/main.pdf main.pdf
	cp $(OUT)/supplement-main.pdf supplement.pdf
	# Portable companion name matches the main PDF GoToR target in the build bundle.
	cp $(OUT)/supplement-main.pdf $(OUT)/supplement.pdf
	{ sed -n 's/^INPUT //p' $(OUT)/main.fls $(OUT)/supplement-main.fls; printf '%s\n' Makefile reproduce.sh REVIEW_ORIGIN_TRANSPORT.md README.md $(ROOT)/README.md CORPUS_READER_AND_SVG_REPAIR_RECEIPT_20261019.json $(REPAIR_HISTORY) $(ROOT)/crates/paper-artifacts/examples/fixtures/verify-review-origin-neutral.v1.json $(HERO_SVG) $(CROP_SVG) publication.v2.json figure-semantics.v1.json claim-evidence.v1.json generated/manifest.json $(REPORT_METADATA) $(DISPLAY_ALTERNATIVES) $(STORY_SVGS) $(POSITIVE_SVGS); for name in $(GEN_FIGURES); do printf 'generated/%s.svg\n' "$$name"; done; for name in $(POP_FIGURES); do printf 'figures/%s.svg\n' "$$name"; done; printf '%s\n' *-inputs.v1.json $(REPORT_CONTRACTS); find $(ROOT)/crates/paper-artifacts -maxdepth 2 -name '*.rs' -type f; printf '%s\n' $(ROOT)/crates/paper-artifacts/Cargo.toml $(ROOT)/crates/paper-artifacts/Cargo.lock $(ROOT)/rust-toolchain.toml; for family in Inter 'Inter:weight=bold' sans-serif 'sans-serif:weight=bold'; do fc-match -f '%{file}\n' "$$family" || exit; done; } | LC_ALL=C sort -u > $(OUT)/INPUT_FILES
	while IFS= read -r input; do test -f "$$input" || exit 1; sha256sum "$$input" || exit; done < $(OUT)/INPUT_FILES > $(OUT)/SOURCE_SHA256SUMS
	sha256sum main.pdf supplement.pdf $(OUT)/supplement.pdf $(GEN_PNGS) $(POP_PNGS) $(STORY_PNGS) $(POSITIVE_PNGS) $(README_HERO_PNG) figures/fig1_hero_blender_v2.png $(HERO_PNG) $(CROP_PNG) $(REPORT_METADATA) $(DISPLAY_ALTERNATIVES) > $(OUT)/OUTPUT_SHA256SUMS
	lualatex --version > $(OUT)/toolchain.txt
	printf 'SOURCE_DATE_EPOCH=%s\n' "$(SOURCE_DATE_EPOCH)" >> $(OUT)/toolchain.txt
	rsvg-convert --version >> $(OUT)/toolchain.txt
	kpsewhich IEEEtran.cls >> $(OUT)/toolchain.txt
	pdfinfo main.pdf > $(OUT)/pdfinfo.txt
	pdfinfo supplement.pdf > $(OUT)/supplement-pdfinfo.txt
main supplement: pdfs
# Variant B: same source with the smoothing subsection switched on (sections/smoothing-variant.tex).
pdf-smoothed: figures
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main-smoothed.tex > $(OUT)/main-smoothed-pass1.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main-smoothed.tex > $(OUT)/main-smoothed-pass2.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main-smoothed.tex > $(OUT)/main-smoothed-pass3.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main-smoothed.tex > $(OUT)/main-smoothed-pass4.log
	cp $(OUT)/main-smoothed.pdf main-smoothed.pdf
	sha256sum main-smoothed.pdf > $(OUT)/main-smoothed.sha256
	pdfinfo main-smoothed.pdf > $(OUT)/main-smoothed-pdfinfo.txt
# Smoothing ladder of the paper's winner (smoothing-ladder.pdf): ten levels are run (L0-L9) and L0-L6 are
# published (L7-L9 fail closed on face reversal); each
# re-scored and rendered for 3D relief. `ladder-data` smooths the exported mesh and re-solves the
# solid tip at every level (about 30 minutes; its inputs live outside git in WINNER_DIR).
# `ladder-figures` and `ladder-pdf` rebuild from the committed receipt in ladder/.
# The label and inputs default to the committed receipt's, so this target re-runs the paper's
# current winner; set both to ladder a new one. Each is read once (ladder-data replaces ladder/).
ifndef LADDER_LABEL
LADDER_LABEL := $(shell python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["label"])' $(CURDIR)/ladder/ladder.json)
endif
ifndef WINNER_DIR
WINNER_DIR := $(shell python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["winner_dir"])' $(CURDIR)/ladder/ladder.json)
endif
SOLVER_PYTHON ?= $(HOME)/.venvs/jarambit-fem-pinned/bin/python
ifndef LADDER_RUN
LADDER_RUN := $(HOME)/jarambit_runs/smoothing/$(LADDER_LABEL)-ladder-$(shell date -u +%Y%m%dT%H%M%SZ)
endif
LADDER_JOBS ?= 5
smoothing-ladder: ladder-data ladder-figures ladder-pdf
ladder-data:
	@test -n "$(LADDER_LABEL)" && test -d "$(WINNER_DIR)" || { echo "ladder-data: no winner inputs (LADDER_LABEL='$(LADDER_LABEL)', WINNER_DIR='$(WINNER_DIR)')" >&2; exit 1; }
	cd $(ROOT)/experiments/geometry-smoothing-benchmark && $(CARGO) build --release --locked
	cd $(ROOT) && $(CARGO) build --release --locked -p jarambit
	cd $(ROOT) && $(CARGO_TARGET_DIR)/release/jarambit smoothing-ladder --winner $(WINNER_DIR) --label $(LADDER_LABEL) \
	  --smoother $(CARGO_TARGET_DIR)/release/geometry-smoothing-benchmark --solver-python $(SOLVER_PYTHON) \
	  --out $(LADDER_RUN) --jobs $(LADDER_JOBS) --code-commit $$(git rev-parse HEAD)
	rm -rf ladder && mkdir ladder
	cp $(LADDER_RUN)/ladder.json $(LADDER_RUN)/ladder-table.tex $(LADDER_RUN)/ladder-levels.tex $(LADDER_RUN)/ladder-scores.dat ladder/
ladder-figures:
	rm -rf $(OUT)/ladder-raw ladder/*.png ladder/*.png.manifest.json
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example smoothing_ladder_figure -- \
	  --ladder docs/paper/ladder/ladder.json --out $(OUT)/ladder-raw --code-commit $$(git rev-parse HEAD)
	cp $(OUT)/ladder-raw/smoothing-ladder.render.v1.json $(OUT)/ladder-raw/ladder-meta.tex ladder/
	for svg in $(OUT)/ladder-raw/*.svg; do name=$$(basename $$svg .svg); \
	  (cd $(OUT)/ladder-raw && rsvg-convert -o $(abspath ladder)/$$name.png $$name.svg) || exit 1; \
	  printf '{\n  "figure": "docs/paper/ladder/%s.png",\n  "figure_sha256": "%s",\n  "script": "crates/paper-artifacts/examples/smoothing_ladder_figure.rs",\n  "inputs": {\n    "docs/paper/ladder/ladder.json": "%s",\n    "docs/paper/ladder/smoothing-ladder.render.v1.json": "%s"\n  },\n  "postprocess": "stored-deflate producer PNG re-encoded losslessly by rsvg-convert at its own size"\n}\n' \
	    $$name $$(sha256sum < ladder/$$name.png | cut -c1-64) $$(sha256sum < ladder/ladder.json | cut -c1-64) \
	    $$(sha256sum < ladder/smoothing-ladder.render.v1.json | cut -c1-64) > ladder/$$name.png.manifest.json; done
ladder-pdf:
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) smoothing-ladder.tex > $(OUT)/smoothing-ladder-pass1.log
	$(TEX_ENV) lualatex -recorder -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) smoothing-ladder.tex > $(OUT)/smoothing-ladder-pass2.log
	cp $(OUT)/smoothing-ladder.pdf smoothing-ladder.pdf
	pdfinfo smoothing-ladder.pdf > $(OUT)/smoothing-ladder-pdfinfo.txt
# Final RR5 validation only; implementation builds use artifacts/pdf targets.
audit: audit-full
audit-quick:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit
audit-full:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts -- --check
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example score_decomposition -- --check
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit -- --full-zstd
review-origin-controls:
	cd $(ROOT) && $(CARGO) test --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example verify_review_origin -- --nocapture
# Both papers (A: main.pdf, B: main-smoothed.pdf) must build cleanly and stay within 15-20 pages.
check: audit pdfs pdf-smoothed
	! rg 'Overfull|LaTeX Warning: (Reference|Citation).*undefined' $(OUT)/main.log $(OUT)/supplement-main.log $(OUT)/main-smoothed.log
	for f in $(OUT)/pdfinfo.txt $(OUT)/main-smoothed-pdfinfo.txt; do \
	  awk '/^Pages:/ { if ($$2 < 15 || $$2 > 20) exit 1; found=1 } END { if (!found) exit 1 }' $$f || { echo "$$f: page count outside 15-20"; exit 1; }; done
clean:
	@echo 'Build artifacts remain under target/ieee/paper for inspection; no automatic deletion.'
# Surface-tension pocket smoothing (variant B: sections/pocket-smoothing.tex and its main page).
# Renders the Stage-2 run bound by the receipt below; the STLs and D1 fields stay outside git,
# pinned by sha256 in the receipt and checked against it here. Geometry and retained fields only.
# The subject's receipt names its STLs and D1 fields; set POCKET_SUBJECT to render another subject.
POCKET_SUBJECT ?= ec3i042_r01_p10_c56
POCKET_RECEIPT_REL := experiments/pocket-surface-tension-smoothing/results/$(POCKET_SUBJECT)/RECEIPT.json
POCKET_RECEIPT := ../../$(POCKET_RECEIPT_REL)
POCKET_FILE = $(shell python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print(next(r for r in d["runs"] if r["name"]=="f075")["files"][sys.argv[2]]["path"])' $(POCKET_RECEIPT) $(1))
POCKET_STL := $(call POCKET_FILE,stl)
POCKET_D1 := $(call POCKET_FILE,d1)
POCKET_RAW_STL := $(shell python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["raw_export_stl"]["path"])' $(POCKET_RECEIPT))
POCKET_FIGS := $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example pocket_smoothing_figures --
pocket-figures:
	for f in $(POCKET_STL) $(POCKET_D1) $(POCKET_RAW_STL); do \
	  grep -q $$(sha256sum < $$f | cut -c1-64) $(POCKET_RECEIPT) || { echo "$$f does not match $(POCKET_RECEIPT)"; exit 1; }; done
	rm -rf $(OUT)/pocket-raw figures/pocket && mkdir -p figures/pocket
	cd $(ROOT) && $(POCKET_FIGS) hero --stl $(POCKET_STL) --d1 $(POCKET_D1) \
	  --score $$(sed -n 's/.*"hero_score_100": *\([0-9.]*\).*/\1/p' $(abspath $(POCKET_RECEIPT)) | head -1) --out $(OUT)/pocket-raw/hero
	cd $(ROOT) && $(POCKET_FIGS) bw --raw $(POCKET_RAW_STL) --ours $(POCKET_STL) \
	  --ladder docs/paper/ladder/smoothing-ladder.render.v1.json --out $(OUT)/pocket-raw/bw
	cd $(ROOT) && $(POCKET_FIGS) morph --stl $(POCKET_STL) \
	  --ladder docs/paper/ladder/smoothing-ladder.render.v1.json --out $(OUT)/pocket-raw/morph
	for d in hero bw morph; do cp $(OUT)/pocket-raw/$$d/$$d.render.v1.json figures/pocket/; done
	cp $(OUT)/pocket-raw/hero/hero.svg figures/pocket/hero.svg
	for svg in $(OUT)/pocket-raw/hero/hero-stack.svg $(OUT)/pocket-raw/bw/bw-*.svg $(OUT)/pocket-raw/morph/morph-*.svg; do \
	  name=$$(basename $$svg .svg); dir=$$(dirname $$svg); \
	  (cd $$dir && rsvg-convert -o $(abspath figures/pocket)/$$name.png $$name.svg) || exit 1; \
	  json=$$(case $$name in hero*) echo hero;; bw*) echo bw;; *) echo morph;; esac).render.v1.json; \
	  printf '{\n  "figure": "docs/paper/figures/pocket/%s.png",\n  "figure_sha256": "%s",\n  "script": "crates/paper-artifacts/examples/pocket_smoothing_figures.rs",\n  "inputs": {\n    "$(POCKET_RECEIPT_REL)": "%s",\n    "docs/paper/figures/pocket/%s": "%s"\n  },\n  "postprocess": "stored-deflate producer PNG re-encoded losslessly by rsvg-convert at its own size"\n}\n' \
	    $$name $$(sha256sum < figures/pocket/$$name.png | cut -c1-64) $$(sha256sum < $(POCKET_RECEIPT) | cut -c1-64) \
	    $$json $$(sha256sum < figures/pocket/$$json | cut -c1-64) > figures/pocket/$$name.png.manifest.json; done
	rsvg-convert -w 2000 -o figures/pocket/hero.png figures/pocket/hero.svg
	printf '{\n  "figure": "docs/paper/figures/pocket/hero.png",\n  "figure_sha256": "%s",\n  "script": "docs/paper/Makefile",\n  "inputs": {\n    "docs/paper/figures/pocket/hero.svg": "%s",\n    "docs/paper/figures/pocket/hero-stack.png": "%s",\n    "$(POCKET_RECEIPT_REL)": "%s"\n  },\n  "note": "make figures: rsvg-convert -w 2000 of the SVG (stacked fields + labels + colour key + W_ULTRALIGHT_v1 badge from the receipt)."\n}\n' \
	  $$(sha256sum < figures/pocket/hero.png | cut -c1-64) $$(sha256sum < figures/pocket/hero.svg | cut -c1-64) \
	  $$(sha256sum < figures/pocket/hero-stack.png | cut -c1-64) $$(sha256sum < $(POCKET_RECEIPT) | cut -c1-64) > figures/pocket/hero.png.manifest.json
	# Main page of each paper: the as-exported (A) and smoothed (B) export, each painted with its own
	# stored multi-case von Mises envelope on one MPa scale, under one camera, light and inset.
	cd $(ROOT) && $(POCKET_FIGS) cover --receipt $(POCKET_RECEIPT_REL) --out $(OUT)/pocket-raw/cover
	rm -f figures/cover/smoothing-figure.render.v1.json
	cp $(OUT)/pocket-raw/cover/cover.render.v1.json $(OUT)/pocket-raw/cover/cover-unsmoothed.svg $(OUT)/pocket-raw/cover/cover-smoothed.svg figures/cover/
	for v in unsmoothed smoothed; do for part in render inset; do \
	  (cd $(OUT)/pocket-raw/cover && rsvg-convert -o $(abspath figures/cover)/$$part-$$v.png $$part-$$v.svg) || exit 1; \
	  printf '{\n  "figure": "docs/paper/figures/cover/%s-%s.png",\n  "figure_sha256": "%s",\n  "script": "crates/paper-artifacts/examples/pocket_smoothing_figures.rs",\n  "inputs": {\n    "docs/paper/figures/cover/cover.render.v1.json": "%s",\n    "$(POCKET_RECEIPT_REL)": "%s"\n  },\n  "external_inputs_note": "STLs and D1 reports are outside git because of their size; cover.render.v1.json and RECEIPT.json bind them by sha256.",\n  "postprocess": "stored-deflate producer PNG re-encoded losslessly by rsvg-convert at its own size"\n}\n' \
	    $$part $$v $$(sha256sum < figures/cover/$$part-$$v.png | cut -c1-64) $$(sha256sum < figures/cover/cover.render.v1.json | cut -c1-64) \
	    $$(sha256sum < $(POCKET_RECEIPT) | cut -c1-64) > figures/cover/$$part-$$v.png.manifest.json; done; \
	  rsvg-convert -w 2000 -h 850 -o figures/cover/cover-$$v.png figures/cover/cover-$$v.svg || exit 1; \
	  printf '{\n  "figure": "docs/paper/figures/cover/cover-%s.png",\n  "figure_sha256": "%s",\n  "script": "docs/paper/Makefile",\n  "inputs": {\n    "docs/paper/figures/cover/cover-%s.svg": "%s",\n    "docs/paper/figures/cover/render-%s.png": "%s",\n    "docs/paper/figures/cover/inset-%s.png": "%s",\n    "$(POCKET_RECEIPT_REL)": "%s"\n  },\n  "note": "make figures: rsvg-convert -w 2000 -h 850 of the SVG written by pocket_smoothing_figures cover (render, 2.5x isogrid-pocket inset, MPa colour key, tests without a stored field and the W_ULTRALIGHT_v1 badge, all from the receipt)."\n}\n' \
	    $$v $$(sha256sum < figures/cover/cover-$$v.png | cut -c1-64) $$v $$(sha256sum < figures/cover/cover-$$v.svg | cut -c1-64) \
	    $$v $$(sha256sum < figures/cover/render-$$v.png | cut -c1-64) $$v $$(sha256sum < figures/cover/inset-$$v.png | cut -c1-64) \
	    $$(sha256sum < $(POCKET_RECEIPT) | cut -c1-64) > figures/cover/cover-$$v.png.manifest.json; done

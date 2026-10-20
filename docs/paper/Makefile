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
REPAIR_HISTORY := history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE102C_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE510713_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_D3BA882D_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_2A3B1A6_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_2A2CD68_20261020.json history/ultralight_weighted_fitness_009f5ac5.source.txt history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_DD92248_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_7A51C1B2_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_73FFEF00_20261020.json history/CORPUS_READER_AND_SVG_REPAIR_RECEIPT_BASE_DBC4034_20261020.json
GEN_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(GEN_FIGURES)))
POP_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(POP_FIGURES)))
STORY_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(STORY_NAMES)))
POSITIVE_PNGS := $(addprefix $(OUT)/,$(addsuffix .png,$(POSITIVE_NAMES)))
# Pinned evidence-baseline timestamp; actual build UTC belongs to the receipt.
TEX_ENV = TEXINPUTS="$(OUT):" SOURCE_DATE_EPOCH=$(SOURCE_DATE_EPOCH) FORCE_SOURCE_DATE=1 TZ=UTC
.PHONY: all artifacts figures pdfs main supplement audit audit-quick audit-full review-origin-controls check clean
# A report must finish generating before its figure/document consumers start.
.NOTPARALLEL:
all: artifacts pdfs
# Reporting reconstruction only; no CAD, FEM, search, tests or CI.
artifacts:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts
figures: $(GEN_PNGS) $(POP_PNGS) $(STORY_PNGS) $(POSITIVE_PNGS) $(README_HERO_PNG) figures/fig1_hero_blender_v2.png $(CROP_PNG)
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
# Final RR5 validation only; implementation builds use artifacts/pdf targets.
audit: audit-full
audit-quick:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit
audit-full:
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts -- --check
	cd $(ROOT) && $(CARGO) run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit -- --full-zstd
review-origin-controls:
	cd $(ROOT) && $(CARGO) test --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example verify_review_origin -- --nocapture
check: audit pdfs
	! rg 'Overfull|LaTeX Warning: (Reference|Citation).*undefined' $(OUT)/main.log $(OUT)/supplement-main.log
	awk '/^Pages:/ { if ($$2 < 15 || $$2 > 20) exit 1; found=1 } END { if (!found) exit 1 }' $(OUT)/pdfinfo.txt
clean:
	@echo 'Build artifacts remain under target/ieee/paper for inspection; no automatic deletion.'

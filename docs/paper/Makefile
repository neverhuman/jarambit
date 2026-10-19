ROOT := ../..
OUT := $(abspath $(ROOT)/target/ieee/paper)
export CARGO_TARGET_DIR := $(abspath $(ROOT)/target)
.PHONY: all artifacts check clean
all: artifacts
	mkdir -p $(OUT)
	rsvg-convert -w 3600 -o $(OUT)/fitness.png generated/fitness.svg
	rsvg-convert -w 3600 -o $(OUT)/population.png generated/population.svg
	rsvg-convert -w 2400 -o $(OUT)/cover-evolution.png generated/cover-evolution.svg
	rsvg-convert -w 3600 -o $(OUT)/plate-fields.png generated/plate-fields.svg
	rsvg-convert -w 3600 -o $(OUT)/verification.png generated/verification.svg
	rsvg-convert -w 3600 -o $(OUT)/assessment.png generated/assessment.svg
	rsvg-convert -w 3600 -o $(OUT)/battery-matrix.png generated/battery-matrix.svg
	rsvg-convert -w 3600 -o $(OUT)/lineage.png generated/lineage.svg
	rsvg-convert -w 3600 -o $(OUT)/export-identity.png generated/export-identity.svg
	rsvg-convert -w 3600 -o $(OUT)/weighted-runs.png generated/weighted-runs.svg
	rsvg-convert -w 3600 -o $(OUT)/reference.png generated/reference.svg
	rsvg-convert -w 3600 -o $(OUT)/population-births.png figures/population-births.svg
	rsvg-convert -w 3600 -o $(OUT)/population-membership.png figures/population-membership.svg
	rsvg-convert -w 3600 -o $(OUT)/population-newer.png figures/population-newer.svg
	rsvg-convert -w 3600 -o $(OUT)/population-latest.png figures/population-latest.svg
	rsvg-convert -w 3600 -o $(OUT)/fitness-progress.png figures/fitness-progress.svg
	rsvg-convert -w 3600 -o $(OUT)/inheritance.png figures/inheritance.svg
	TEXINPUTS="$(OUT):$${TEXINPUTS:-}" SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass1.log
	TEXINPUTS="$(OUT):$${TEXINPUTS:-}" SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass2.log
	cp -f $(OUT)/main.pdf main.pdf
	shasum -a 256 main.tex references.tex sections/*.tex generated/*.json generated/*.svg generated/*.tex generated/*.md generated/population/*.json *-inputs.v1.json figures/*.svg figures/*.png > $(OUT)/SOURCE_SHA256SUMS
	find figures/v2 -maxdepth 1 -type f -print0 | sort -z | xargs -0 shasum -a 256 >> $(OUT)/SOURCE_SHA256SUMS
	shasum -a 256 $(OUT)/main.pdf $(OUT)/fitness.png $(OUT)/population.png $(OUT)/cover-evolution.png $(OUT)/plate-fields.png $(OUT)/verification.png $(OUT)/assessment.png $(OUT)/battery-matrix.png $(OUT)/lineage.png $(OUT)/export-identity.png $(OUT)/weighted-runs.png $(OUT)/reference.png $(OUT)/population-births.png $(OUT)/population-membership.png $(OUT)/population-newer.png $(OUT)/population-latest.png $(OUT)/fitness-progress.png $(OUT)/inheritance.png > $(OUT)/OUTPUT_SHA256SUMS
	lualatex --version > $(OUT)/toolchain.txt
	rsvg-convert --version >> $(OUT)/toolchain.txt
	kpsewhich IEEEtran.cls >> $(OUT)/toolchain.txt
	pdfinfo $(OUT)/main.pdf > $(OUT)/pdfinfo.txt
artifacts:
	cd $(ROOT) && cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts -- --check
	cd $(ROOT) && cargo run --release --locked --manifest-path crates/paper-artifacts/Cargo.toml --example publication_v2_audit
check: all
	! rg 'Overfull|LaTeX Warning: (Reference|Citation).*undefined' $(OUT)/main.log
	awk '/^Pages:/ { if ($$2 < 15 || $$2 > 26) exit 1; found=1 } END { if (!found) exit 1 }' $(OUT)/pdfinfo.txt
clean:
	@echo 'Build artifacts remain under target/ieee/paper for inspection; no automatic deletion.'

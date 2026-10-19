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
	SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass1.log
	SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass2.log
	shasum -a 256 main.tex references.tex sections/*.tex generated/* *-inputs.v1.json figures/v2/* > $(OUT)/SOURCE_SHA256SUMS
	shasum -a 256 $(OUT)/main.pdf $(OUT)/fitness.png $(OUT)/population.png $(OUT)/cover-evolution.png $(OUT)/plate-fields.png $(OUT)/verification.png $(OUT)/assessment.png $(OUT)/battery-matrix.png $(OUT)/lineage.png $(OUT)/export-identity.png > $(OUT)/OUTPUT_SHA256SUMS
	lualatex --version > $(OUT)/toolchain.txt
	rsvg-convert --version >> $(OUT)/toolchain.txt
	kpsewhich IEEEtran.cls >> $(OUT)/toolchain.txt
	pdfinfo $(OUT)/main.pdf > $(OUT)/pdfinfo.txt
artifacts:
	cd $(ROOT) && cargo run --locked --manifest-path crates/paper-artifacts/Cargo.toml --example paper_artifacts -- --check
check: all
	! rg 'Overfull|LaTeX Warning: (Reference|Citation).*undefined' $(OUT)/main.log
	awk '/^Pages:/ { if ($$2 < 15 || $$2 > 20) exit 1; found=1 } END { if (!found) exit 1 }' $(OUT)/pdfinfo.txt
clean:
	@echo 'Build artifacts remain under target/ieee/paper for inspection; no automatic deletion.'

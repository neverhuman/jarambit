ROOT := ../..
OUT := $(abspath $(ROOT)/target/ieee/paper)
export CARGO_TARGET_DIR := $(abspath $(ROOT)/target)
.PHONY: all artifacts check clean
all: artifacts
	mkdir -p $(OUT)
	rsvg-convert -f pdf -o $(OUT)/fitness.pdf generated/fitness.svg
	rsvg-convert -f pdf -o $(OUT)/population.pdf generated/population.svg
	rsvg-convert -f pdf -o $(OUT)/cover-evolution.pdf generated/cover-evolution.svg
	SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass1.log
	SOURCE_DATE_EPOCH=1792365255 FORCE_SOURCE_DATE=1 TZ=UTC lualatex -halt-on-error -interaction=nonstopmode -output-directory=$(OUT) main.tex > $(OUT)/build-pass2.log
	shasum -a 256 main.tex references.tex sections/*.tex generated/* figure-inputs.v1.json figures/v2/* > $(OUT)/SOURCE_SHA256SUMS
	shasum -a 256 $(OUT)/main.pdf > $(OUT)/OUTPUT_SHA256SUMS
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

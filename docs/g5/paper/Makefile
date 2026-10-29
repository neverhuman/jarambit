# G5 release paper. Every number comes from the release receipt via g5-release-tex.
#   make -C docs/g5/paper          generated/*.tex from evidence/g5/v$(RECORD)/release.json, then the PDF
#   make -C docs/g5/paper check    fail if generated/*.tex is stale against the receipt
# VERSION is the release; RECORD (evidence/g5/RECORD) the cohort of record whose receipt the paper reports.
RAW_VERSION := $(shell cat ../../VERSION)
ROOT        := ../../..
RECORD      := $(shell cat $(ROOT)/evidence/g5/RECORD)
VERSION     := $(if $(filter 5.% 6.% 7.%,$(RAW_VERSION)),$(RAW_VERSION),$(RECORD))
IMG         := ../v$(RECORD)/img
FIGURES     := $(patsubst %.svg,%.pdf,$(wildcard $(IMG)/*.svg $(IMG)/fem/*.svg))
RECEIPT     := $(ROOT)/evidence/g5/v$(RECORD)/release.json
PDF         := g5-v$(VERSION).pdf
TEXGEN       = cargo run --quiet --release --locked --manifest-path $(ROOT)/crates/g5/Cargo.toml --bin g5-release-tex --

all: $(PDF)

generated/numbers.tex: $(RECEIPT) ../../VERSION $(ROOT)/crates/g5/src/bin/g5_release_tex.rs $(wildcard $(ROOT)/evidence/g5/v$(RECORD)/vv/*.json) $(wildcard $(ROOT)/evidence/g5/v$(RECORD)/spike/*.json $(ROOT)/evidence/g5/v$(RECORD)/fleet/*.json) $(IMG)/manifest.json $(wildcard ../v$(RECORD)/cad/*.json)
	$(TEXGEN) $(RECEIPT) generated $(VERSION)

# every SVG figure of the record (evolution, fitness budget and terms, FEM renderings) as PDF
$(IMG)/%.pdf: $(IMG)/%.svg
	rsvg-convert -f pdf -o $@ $<

$(PDF): main.tex $(wildcard sections/*.tex) generated/numbers.tex $(FIGURES) ../../paper/ei-ieee-skin.tex
	latexmk -lualatex -interaction=nonstopmode -halt-on-error -outdir=build main.tex
	cp build/main.pdf $(PDF)

check:
	$(TEXGEN) $(RECEIPT) build/generated-check $(VERSION)
	diff -r generated build/generated-check

clean:
	rm -rf build

.PHONY: all check clean

# G3 release paper. Every number comes from the release receipt via g3-release-tex.
#   make -C docs/g3/paper          generated/*.tex from evidence/g3/v$(RECORD)/release.json, then the PDF
#   make -C docs/g3/paper check    fail if generated/*.tex is stale against the receipt
# VERSION is the release; RECORD (evidence/g3/RECORD) the cohort of record whose receipt the paper reports.
VERSION := $(shell cat ../../VERSION)
ROOT    := ../../..
RECORD  := $(shell cat $(ROOT)/evidence/g3/RECORD)
RECEIPT := $(ROOT)/evidence/g3/v$(RECORD)/release.json
PDF     := g3-v$(VERSION).pdf
TEXGEN   = cargo run --quiet --release --locked --manifest-path $(ROOT)/crates/g3/Cargo.toml --bin g3-release-tex --

all: $(PDF)

generated/numbers.tex: $(RECEIPT) ../../VERSION
	$(TEXGEN) $(RECEIPT) generated $(VERSION)

$(PDF): main.tex $(wildcard sections/*.tex) generated/numbers.tex ../../paper/ei-ieee-skin.tex
	latexmk -lualatex -interaction=nonstopmode -halt-on-error -outdir=build main.tex
	cp build/main.pdf $(PDF)

check:
	$(TEXGEN) $(RECEIPT) build/generated-check $(VERSION)
	diff -r generated build/generated-check

clean:
	rm -rf build

.PHONY: all check clean

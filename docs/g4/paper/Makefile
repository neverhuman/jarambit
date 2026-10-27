# G4 release paper (v4.0.0). Every headline number comes from evidence/g4/v4.0.0/release.json via g4-release-tex.
VERSION := 4.0.0
ROOT    := ../../..
RECEIPT := $(ROOT)/evidence/g4/v4.0.0/release.json
PDF     := g4-v$(VERSION).pdf

all: $(PDF)

$(PDF): main.tex generated/numbers.tex ../../paper/ei-ieee-skin.tex
	latexmk -lualatex -interaction=nonstopmode -halt-on-error -outdir=build main.tex
	cp build/main.pdf $(PDF)

check:
	test -s generated/numbers.tex
	test -s $(PDF)

clean:
	rm -rf build

.PHONY: all check clean

//! Build script for `jarambit-g4` (`G4-GEN-1`, `H-1`, `C-42`, `C-50`):
//! Parses `../../genome/G4.0.toml` at compile time and generates:
//! 1. `$OUT_DIR/schema_gen.rs` (`G4GeneSpec`, `G4_GENES`, and `#[repr(C)] pub struct G4DecodedParams`)
//! 2. `../../docs/g4/generated/genome_table.md`
//! 3. `../../docs/g4/paper/generated/genome_table.tex`

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[4 * i],
                chunk[4 * i + 1],
                chunk[4 * i + 2],
                chunk[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (hi, vi) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *hi = hi.wrapping_add(vi);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

#[derive(Debug, Clone)]
struct ParsedGene {
    name: String,
    module: String,
    kind: String,
    scale: String,
    lo: f64,
    hi: f64,
    default: f64,
    unit: String,
    description: String,
}

fn parse_genes_toml(content: &str) -> Vec<ParsedGene> {
    let mut genes = Vec::new();
    let mut current: BTreeMap<String, String> = BTreeMap::new();
    let mut in_gene = false;

    let flush = |cur: &mut BTreeMap<String, String>, out: &mut Vec<ParsedGene>| {
        if cur.is_empty() {
            return;
        }
        let name = cur.get("name").expect("missing gene name").clone();
        let module = cur.get("module").expect("missing gene module").clone();
        let kind = cur.get("kind").expect("missing gene kind").clone();
        let scale = cur.get("scale").expect("missing gene scale").clone();
        let lo: f64 = cur.get("lo").expect("missing gene lo").parse().expect("parse lo");
        let hi: f64 = cur.get("hi").expect("missing gene hi").parse().expect("parse hi");
        let default: f64 = cur.get("default").expect("missing gene default").parse().expect("parse default");
        let unit = cur.get("unit").expect("missing gene unit").clone();
        let description = cur.get("description").expect("missing gene description").clone();
        assert!(lo < hi, "gene {name}: lo ({lo}) must be < hi ({hi})");
        assert!(
            default >= lo && default <= hi,
            "gene {name}: default ({default}) outside [{lo}, {hi}]"
        );
        out.push(ParsedGene {
            name,
            module,
            kind,
            scale,
            lo,
            hi,
            default,
            unit,
            description,
        });
        cur.clear();
    };

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[[genes]]" {
            if in_gene {
                flush(&mut current, &mut genes);
            }
            in_gene = true;
            continue;
        }
        if !in_gene {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim().to_string();
            let val = v.trim().trim_matches('"').to_string();
            current.insert(key, val);
        }
    }
    if in_gene {
        flush(&mut current, &mut genes);
    }
    genes
}


fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let repo_root = Path::new(&manifest_dir).join("../..");
    let toml_path = repo_root.join("genome/G4.0.toml");
    println!("cargo:rerun-if-changed={}", toml_path.display());

    let toml_bytes = fs::read(&toml_path).expect("read genome/G4.0.toml");
    let toml_sha256 = sha256_hex(&toml_bytes);
    let toml_str = String::from_utf8(toml_bytes).expect("utf8 G4.0.toml");
    let genes = parse_genes_toml(&toml_str);
    assert!(!genes.is_empty(), "G4.0.toml must define at least 1 gene");

    let mut code = String::new();
    code.push_str("// AUTO-GENERATED BY crates/g4/build.rs FROM genome/G4.0.toml — DO NOT EDIT.\n\n");
    code.push_str("pub const G4_GENOME_SCHEMA: &str = \"G4.0\";\n");
    code.push_str(&format!("pub const G4_GENE_COUNT: usize = {};\n", genes.len()));
    code.push_str(&format!(
        "pub const G4_GENOME_TOML_SHA256: &str = \"{}\";\n\n",
        toml_sha256
    ));

    code.push_str(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]\n\
         pub enum G4Module { H, G, P, Z, X, I, E, T, A }\n\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]\n\
         pub enum G4GeneKind { F64, Int }\n\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]\n\
         pub enum G4GeneScale { Lin, Log }\n\n\
         #[derive(Debug, Clone, Copy)]\n\
         pub struct G4GeneSpec {\n\
             pub index: usize,\n\
             pub name: &'static str,\n\
             pub module: G4Module,\n\
             pub kind: G4GeneKind,\n\
             pub scale: G4GeneScale,\n\
             pub lo: f64,\n\
             pub hi: f64,\n\
             pub default: f64,\n\
             pub unit: &'static str,\n\
             pub description: &'static str,\n\
         }\n\n",
    );

    code.push_str("pub const G4_GENES: [G4GeneSpec; G4_GENE_COUNT] = [\n");
    for (idx, g) in genes.iter().enumerate() {
        let mod_variant = match g.module.as_str() {
            "H" => "G4Module::H",
            "G" => "G4Module::G",
            "P" => "G4Module::P",
            "Z" => "G4Module::Z",
            "X" => "G4Module::X",
            "I" => "G4Module::I",
            "E" => "G4Module::E",
            "T" => "G4Module::T",
            "A" => "G4Module::A",
            other => panic!("Unknown module {other}"),
        };
        let kind_variant = match g.kind.as_str() {
            "f64" => "G4GeneKind::F64",
            "int" => "G4GeneKind::Int",
            other => panic!("Unknown kind {other}"),
        };
        let scale_variant = match g.scale.as_str() {
            "lin" => "G4GeneScale::Lin",
            "log" => "G4GeneScale::Log",
            other => panic!("Unknown scale {other}"),
        };
        code.push_str(&format!(
            "    G4GeneSpec {{ index: {idx}, name: {:?}, module: {mod_variant}, kind: {kind_variant}, scale: {scale_variant}, lo: {:.6}, hi: {:.6}, default: {:.6}, unit: {:?}, description: {:?} }},\n",
            g.name, g.lo, g.hi, g.default, g.unit, g.description
        ));
    }
    code.push_str("];\n\n");

    // Typed #[repr(C)] G4DecodedParams struct (`H-1` / `C-42`)
    code.push_str("#[repr(C)]\n#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct G4DecodedParams {\n");
    for g in &genes {
        code.push_str(&format!("    pub {}: f64,\n", g.name));
    }
    // Derived anthropometric / frame datums cached directly in the struct for O(1) access
    code.push_str("    pub bore_d_mm: f64,\n");
    code.push_str("    pub lip_w_mm: f64,\n");
    code.push_str("    pub guard_ref_x_mm: f64,\n");
    code.push_str("    pub l_grip_mm: f64,\n");
    code.push_str("}\n\n");

    code.push_str("impl G4DecodedParams {\n");
    code.push_str("    pub fn from_physical_slice(vals: &[f64; G4_GENE_COUNT]) -> Self {\n");
    for (idx, g) in genes.iter().enumerate() {
        if g.kind == "int" {
            code.push_str(&format!(
                "        let {} = vals[{idx}].round().clamp({:.6}, {:.6});\n",
                g.name, g.lo, g.hi
            ));
        } else {
            code.push_str(&format!(
                "        let {} = vals[{idx}].clamp({:.6}, {:.6});\n",
                g.name, g.lo, g.hi
            ));
        }
    }
    code.push_str("        let bore_d_mm = 24.9;\n");
    code.push_str("        let lip_w_mm = 8.0;\n");
    code.push_str("        let guard_ref_x_mm = 75.6;\n");
    code.push_str("        let l_grip_mm = (19.5 + 21.0 + 22.0) * hand_scale + grip_dl;\n");
    code.push_str("        Self {\n");
    for g in &genes {
        code.push_str(&format!("            {},\n", g.name));
    }
    code.push_str("            bore_d_mm,\n            lip_w_mm,\n            guard_ref_x_mm,\n            l_grip_mm,\n        }\n    }\n\n");

    code.push_str("    pub fn get_by_name(&self, name: &str) -> Option<f64> {\n        match name {\n");
    for g in &genes {
        code.push_str(&format!("            {:?} => Some(self.{}),\n", g.name, g.name));
    }
    code.push_str("            \"bore_d_mm\" => Some(self.bore_d_mm),\n");
    code.push_str("            \"lip_w_mm\" => Some(self.lip_w_mm),\n");
    code.push_str("            \"guard_ref_x_mm\" => Some(self.guard_ref_x_mm),\n");
    code.push_str("            \"l_grip_mm\" => Some(self.l_grip_mm),\n");
    code.push_str("            _ => None,\n        }\n    }\n\n");

    code.push_str("    pub fn to_btreemap(&self) -> std::collections::BTreeMap<&'static str, f64> {\n        let mut m = std::collections::BTreeMap::new();\n");
    for g in &genes {
        code.push_str(&format!("        m.insert({:?}, self.{});\n", g.name, g.name));
    }
    code.push_str("        m.insert(\"bore_d_mm\", self.bore_d_mm);\n");
    code.push_str("        m.insert(\"lip_w_mm\", self.lip_w_mm);\n");
    code.push_str("        m.insert(\"guard_ref_x_mm\", self.guard_ref_x_mm);\n");
    code.push_str("        m.insert(\"l_grip_mm\", self.l_grip_mm);\n");
    code.push_str("        m\n    }\n}\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("schema_gen.rs"), code).expect("write schema_gen.rs");

    // Emit Markdown and LaTeX gene tables (`G4-GEN-1`)
    let md_dir = repo_root.join("docs/g4/generated");
    let _ = fs::create_dir_all(&md_dir);
    let mut md = String::new();
    md.push_str(&format!(
        "# Jarambit `G4.0` Declarative Genome Table ({} Genes, SHA-256 `{}`)\n\n",
        genes.len(),
        &toml_sha256[..16]
    ));
    md.push_str("| # | Gene | Mod | Kind | Bounds | Default | Unit | Description |\n");
    md.push_str("|---:|---|---|---|---|---:|---|---|\n");
    for (idx, g) in genes.iter().enumerate() {
        md.push_str(&format!(
            "| {} | `{}` | `{}` | `{}` | `[{}, {}]` | `{}` | `{}` | {} |\n",
            idx, g.name, g.module, g.kind, g.lo, g.hi, g.default, g.unit, g.description
        ));
    }
    let _ = fs::write(md_dir.join("genome_table.md"), md);

    let tex_dir = repo_root.join("docs/g4/paper/generated");
    let _ = fs::create_dir_all(&tex_dir);
    let mut tex = String::new();
    tex.push_str("% Auto-generated from genome/G4.0.toml by crates/g4/build.rs\n");
    tex.push_str(&format!("\\newcommand{{\\gFourGeneCount}}{{{}}}\n", genes.len()));
    tex.push_str(&format!("\\newcommand{{\\gFourSchemaShaShort}}{{{}}}\n", &toml_sha256[..12]));
    let _ = fs::write(tex_dir.join("genome_table.tex"), tex);
}

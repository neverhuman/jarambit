//! Typed bridge from source-bound retained surface/colors to appearance only.
//! Geometry/field registration is owned by the current publication caller. This
//! module never decodes a phenotype, queries a nearest cell, computes fitness,
//! changes the palette, creates a mask, or assigns another candidate's fields.
use crate::appearance::{self, Camera, NormalSettings, RawMesh, RenderSettings};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
type Result<T> = std::result::Result<T, String>;

pub struct RetainedSurface {
    mesh: RawMesh,
    subject: String,
    source_sha256: String,
    geometry_sha256: String,
}
impl RetainedSurface {
    pub fn from_stl(
        subject: &str,
        bytes: &[u8],
        expected_source: &str,
        expected_geometry: &str,
    ) -> Result<Self> {
        if subject.is_empty() || appearance::digest(bytes) != expected_source {
            return Err("retained surface subject/source binding differs".into());
        }
        let mesh = appearance::decode_stl(bytes)?;
        let geometry_sha256 = mesh.geometry_digest();
        if geometry_sha256 != expected_geometry {
            return Err("ordered retained geometry differs".into());
        }
        Ok(Self {
            mesh,
            subject: subject.into(),
            source_sha256: expected_source.into(),
            geometry_sha256,
        })
    }
    pub fn mesh(&self) -> &RawMesh {
        &self.mesh
    }
}

#[derive(Clone, Debug)]
pub struct CellColor {
    /// Original dimensionless four-response index, or a typed missing value.
    pub index: Option<f64>,
    pub missing_reason: Option<String>,
    /// Original source-channel order; true is explicitly producer-masked.
    pub source_masks: Vec<bool>,
    /// Existing original contributor order, not invented from scalar magnitude.
    pub contributors: Vec<String>,
    /// Existing unlit Palette::rgb output (including explicit missing color).
    pub base_rgb: [u8; 3],
}
#[derive(Clone, Debug)]
pub struct RegistrationInput {
    pub subject: String,
    pub d1_sha256: String,
    pub channel_ids: Vec<String>,
    /// Indexed by original D1 triangle ID, never sorted by response.
    pub cells: Vec<CellColor>,
    /// One original D1 cell ID for EVERY original ordered STL face.
    pub face_to_cell: Vec<usize>,
}
fn string(h: &mut Sha256, s: &str) {
    h.update((s.len() as u64).to_le_bytes());
    h.update(s.as_bytes());
}
impl RegistrationInput {
    fn validate(&self) -> Result<()> {
        let unique: BTreeSet<_> = self.channel_ids.iter().collect();
        if self.subject.is_empty()
            || self.channel_ids.len() != 4
            || unique.len() != 4
            || self.channel_ids.iter().any(String::is_empty)
            || self.cells.is_empty()
            || self.face_to_cell.is_empty()
            || self.face_to_cell.iter().any(|i| *i >= self.cells.len())
        {
            return Err("invalid four-channel source registration".into());
        }
        for cell in &self.cells {
            if cell.source_masks.len() != 4 {
                return Err("source mask/channel order differs".into());
            }
            let mut seen = BTreeSet::new();
            for id in &cell.contributors {
                let i = self
                    .channel_ids
                    .iter()
                    .position(|c| c == id)
                    .ok_or("foreign contributor")?;
                if !seen.insert(id) || cell.source_masks[i] {
                    return Err("duplicate or masked contributor".into());
                }
            }
            match cell.index {
                Some(v)
                    if v.is_finite()
                        && (0. ..=1.).contains(&v)
                        && !cell.contributors.is_empty()
                        && cell.missing_reason.is_none() => {}
                None if cell.contributors.is_empty()
                    && cell.missing_reason.as_ref().is_some_and(|s| !s.is_empty()) => {}
                _ => return Err("corrupt index or untyped missing/contributor state".into()),
            }
        }
        Ok(())
    }
    /// Bit-preserving original cell/face order digest. The expected digest must
    /// be independently source-bound by Captain, never treated as self-attested.
    pub fn digest(&self) -> Result<String> {
        self.validate()?;
        let mut h = Sha256::new();
        h.update(b"hero.registered_cell_colors.v1\0");
        string(&mut h, &self.subject);
        string(&mut h, &self.d1_sha256);
        for id in &self.channel_ids {
            string(&mut h, id);
        }
        h.update((self.cells.len() as u64).to_le_bytes());
        for cell in &self.cells {
            match cell.index {
                Some(v) => {
                    h.update([1_u8]);
                    h.update(v.to_le_bytes());
                }
                None => h.update([0_u8]),
            }
            match &cell.missing_reason {
                Some(s) => {
                    h.update([1_u8]);
                    string(&mut h, s);
                }
                None => h.update([0_u8]),
            }
            for &mask in &cell.source_masks {
                h.update([u8::from(mask)]);
            }
            h.update((cell.contributors.len() as u64).to_le_bytes());
            for id in &cell.contributors {
                string(&mut h, id);
            }
            h.update(cell.base_rgb);
        }
        h.update((self.face_to_cell.len() as u64).to_le_bytes());
        for &i in &self.face_to_cell {
            h.update((i as u64).to_le_bytes());
        }
        Ok(format!("{:x}", h.finalize()))
    }
    fn component_digests(&self) -> Value {
        let (mut scalar, mut mask, mut contributors, mut colors, mut mapping) = (
            Sha256::new(),
            Sha256::new(),
            Sha256::new(),
            Sha256::new(),
            Sha256::new(),
        );
        for cell in &self.cells {
            match cell.index {
                Some(v) => {
                    scalar.update([1_u8]);
                    scalar.update(v.to_le_bytes());
                }
                None => scalar.update([0_u8]),
            }
            for &v in &cell.source_masks {
                mask.update([u8::from(v)]);
            }
            contributors.update((cell.contributors.len() as u64).to_le_bytes());
            for id in &cell.contributors {
                string(&mut contributors, id);
            }
            colors.update(cell.base_rgb);
        }
        for &i in &self.face_to_cell {
            mapping.update((i as u64).to_le_bytes());
        }
        json!({"registered_index_bits":format!("{:x}",scalar.finalize()),"original_source_mask_bits":format!("{:x}",mask.finalize()),
            "contributor_ids_and_counts":format!("{:x}",contributors.finalize()),"unlit_cell_palette_rgb":format!("{:x}",colors.finalize()),"ordered_face_to_original_cell_ids":format!("{:x}",mapping.finalize())})
    }
}
pub struct RegisteredCellColors {
    input: RegistrationInput,
    digest: String,
}
impl RegisteredCellColors {
    pub fn bind(
        input: RegistrationInput,
        d1_bytes: &[u8],
        expected_assignment_digest: &str,
    ) -> Result<Self> {
        if appearance::digest(d1_bytes) != input.d1_sha256 {
            return Err("original D1 bytes differ".into());
        }
        let digest = input.digest()?;
        if digest != expected_assignment_digest {
            return Err("source-bound assignment digest differs".into());
        }
        Ok(Self { input, digest })
    }
}

#[derive(Clone, Debug)]
pub struct AppearanceRecipe {
    pub normals: NormalSettings,
    /// None means inferred exact-edge adjacency only, not trusted CAD semantics.
    pub regions: Option<Vec<u32>>,
    pub render: RenderSettings,
    /// Frozen caller camera digest; changing normal/light settings cannot refit.
    pub expected_camera_sha256: String,
}
pub struct AppearanceOutput {
    pub image: appearance::Rendered,
    pub receipt: Value,
    /// MUST appear visibly in the publication caption/key, outside clean imagery.
    pub required_visible_disclosure: &'static str,
}
fn bytes_of_usize(values: &[usize]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|v| (*v as u64).to_le_bytes())
        .collect()
}
fn normal_digest(values: &[[f64; 3]]) -> String {
    let mut h = Sha256::new();
    h.update(b"appearance.corner_normals.f64le.v1\0");
    for v in values.iter().flatten() {
        h.update(v.to_le_bytes());
    }
    format!("{:x}", h.finalize())
}
pub fn render(
    surface: &RetainedSurface,
    registration: &RegisteredCellColors,
    recipe: &AppearanceRecipe,
) -> Result<AppearanceOutput> {
    if recipe.render.camera.digest() != recipe.expected_camera_sha256 {
        return Err("fixed original camera binding differs".into());
    }
    let input = &registration.input;
    if input.subject != surface.subject
        || input.face_to_cell.len() != surface.mesh.len()
        || input.digest()? != registration.digest
    {
        return Err("subject or complete source-face registration differs".into());
    }
    let geometry_before = surface.mesh.geometry_digest();
    let components_before = input.component_digests();
    let base: Vec<_> = input
        .face_to_cell
        .iter()
        .map(|&i| input.cells[i].base_rgb)
        .collect();
    let (normals, stats) =
        appearance::derive_normals(&surface.mesh, recipe.normals, recipe.regions.as_deref())?;
    let image = appearance::render(&surface.mesh, &normals, &base, recipe.render)?;
    if image.submitted_triangles != surface.mesh.len()
        || geometry_before != surface.mesh.geometry_digest()
        || input.digest()? != registration.digest
        || components_before != input.component_digests()
    {
        return Err("appearance changed immutable geometry/field assignment".into());
    }
    let Camera {
        rotation,
        scale,
        offset,
    } = recipe.render.camera;
    let light = recipe.render.lighting;
    let receipt = json!({"schema":"jarambit.retained_hero_appearance.v1","subject":surface.subject,
        "sources":{"stl_sha256":surface.source_sha256,"d1_sha256":input.d1_sha256},
        "geometry":{"ordered_decoded_corners_before":geometry_before,"ordered_decoded_corners_after":surface.geometry_sha256,
            "face_order":"original STL triangle/corner order; no indexed-buffer rewrite, welding, decimation or displacement", "source_faces":surface.mesh.len(),"submitted_faces":image.submitted_triangles,"projected_edge_on_faces":image.edge_on_triangles},
        "registered_assignment":{"sha256":registration.digest,"component_hashes_before":components_before,"component_hashes_after":input.component_digests(),
            "query":"Caller supplies the CURRENT exact nearest-original-D1-centroid map and original source-index ties; adapter does no new registration or interpolation", "source_channels":input.channel_ids,"eligible_channels":4},
        "normals":{"sha256":normal_digest(&normals),"crease_degrees":recipe.normals.crease_degrees,"blend":recipe.normals.blend,"max_deviation_degrees":recipe.normals.max_deviation_degrees,
            "joined_edges":stats.joined_edges,"blocked_edges":stats.blocked_edges,"ambiguous_edges":stats.ambiguous_edges,"duplicate_faces":stats.duplicate_faces,"capped_corners":stats.capped_corners,"changed_corners":stats.changed_corners,"observed_max_deviation_degrees":stats.max_deviation_degrees,
            "region_authority":if recipe.regions.is_some(){"caller-provided labels; bind authority separately"}else{"none; inferred exact-edge fans, not semantic feature preservation"}},
        "camera":{"sha256":recipe.expected_camera_sha256,"rotation":rotation,"scale":scale,"offset":offset,"refit":false,"depth":"maximum posed z per geometric sample; exact ties later original face wins; no normal-based culling"},
        "appearance":{"renderer":"deterministic Rust CPU depth raster; NOT Blender/Cycles","ambient":light.ambient,"diffuse":light.diffuse,"specular":light.specular,"shininess":light.shininess,"direction_camera":light.direction,
            "width":recipe.render.width,"height":recipe.render.height,"supersample":recipe.render.supersample,"outline_pixels":recipe.render.outline_pixels,"outline":"black inward L1 boundary of actual sampled coverage, including holes", "background":"literal sRGB255/255/255 outside actual coverage; no exterior effects", "field_filtering":false,
            "lit_RGB_equals_base_palette":false,"quantitative_companion":"unchanged existing unlit four-channel full-range source figure MUST remain; image.unlit_rgb adds a matched visibility/AA comparison, not a replacement for original input evidence"},
        "buffers":{"illuminated_rgb":appearance::digest(&image.rgb),"matched_unlit_rgb":appearance::digest(&image.unlit_rgb),"coverage":appearance::digest(&image.coverage),"inward_outline":appearance::digest(&image.outline),"winning_original_face_ids":appearance::digest(&bytes_of_usize(&image.winning_face))},
        "fresh_fitness_or_solver_execution":false,"fitness_delta":null,"retained_fitness":"Reuse the same independently bound original observation for both appearances; no newly scored geometry or measured zero-loss delta", "source_attribution":{"commit":"562e9c1e8f776c37a4a242e7ba545f15d5974639","appearance_rs_sha256":"ac88ebfe5b179a566be629b14b327701b1bfe300f0d67d101edd7366572fcb37","port":"generic normals/strict STL/depth principles only; current camera/colors/white-outline adapter newly authored"}});
    Ok(AppearanceOutput {image,receipt,required_visible_disclosure:
        "Illuminated appearance of the retained response index: RGB differs from base palette colors. Use the unchanged unlit four-channel companion for quantitative colors. Retained geometry and fitness are reused, not newly evaluated."})
}

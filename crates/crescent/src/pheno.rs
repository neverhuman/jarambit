//! Crescent genotype -> silhouette (one C2 spline + ring union + tangent neck fillets, C.4
//! serrations + thumb-seat jimping) + lightening holes in C.4 zones + 0.25 mm thickness raster.
use crate::c6_ref as c4; // Rev C.6 anchor (C.5 + two-sided ring-face racetrack groove; C.5 = C.4 + R0.8/0.45 jimping)
use crate::genome::*;
use crate::geom::*;
use jarambit_genotype::TiGrade;
use std::f64::consts::PI;

pub const EDGE_T: f64 = 0.5;
/// Removed-volume slots: relief, lattice, fuller, flute, texture, ISOGRID pockets, RING flute.
pub const NREM: usize = 7;
/// Machining rules for the internal layer: tool radius (Ø2 mm end mill) for pocket corners,
/// minimum through-window width (Ø3 mm tool), window rim to the grind line, window gap.
pub const POCKET_RC: f64 = 1.0;
pub const WIN_MIN_W: f64 = 3.0;
pub const WIN_RIM: f64 = 1.5;
pub const WIN_GAP: f64 = 3.4;
pub const GRID: f64 = 0.25;
pub const CAP_MM: f64 = 4.0 * 25.4;   // 101.6 sharpened-edge cap (each edge)
pub const FLOOR_MM: f64 = 3.375 * 25.4 + 0.5; // 86.225 design floor (all three)

#[derive(Clone, Debug, Default)]
pub struct Blade { pub tip_to_heel: f64, pub horizontal_reach: f64, pub arc_in: f64, pub arc_sp: f64, pub sharpened_arc: f64, pub spine_offset_used: f64, pub hook_drop: f64 }

#[derive(Clone, Debug)]
pub struct Field { pub x0: f64, pub y0: f64, pub g: f64, pub nx: usize, pub ny: usize, pub th: Vec<f32>, pub base: Vec<f32> }
impl Field {
    pub fn at(&self, q: P2) -> f64 {
        let i = ((q.0 - self.x0) / self.g).floor() as i64; let j = ((q.1 - self.y0) / self.g).floor() as i64;
        for r in 0..4i64 { let mut best: Option<(f64, f64)> = None;
            for a in i - r..=i + r { for b in j - r..=j + r {
                if a < 0 || b < 0 || a >= self.nx as i64 || b >= self.ny as i64 { continue; }
                let v = self.th[b as usize * self.nx + a as usize]; if v.is_nan() { continue; }
                let c = (self.x0 + (a as f64 + 0.5) * self.g, self.y0 + (b as f64 + 0.5) * self.g); let d = dist(c, q);
                if best.map_or(true, |x| d < x.0) { best = Some((d, v as f64)); } } }
            if let Some(b) = best { return b.1; } }
        EDGE_T
    }
    /// Plate and export consumers share the actual sharpened field. `base` is machining metadata.
    pub fn at_plate(&self, q: P2, _rs_v2: bool) -> f64 { self.at(q) }
}

#[derive(Clone, Debug)]
pub struct Pheno {
    /// Exact input genes for the continuous exported surface (no catalog or design-label lookup).
    pub genome: Genome,
    pub surface_x_max: f64,
    pub cut_in: Segments, pub cut_sp: Segments,
    pub cp: Vec<P2>,
    /// Full exterior: scallops + jimping + ring + fillets (clockwise like C.4).
    pub outline: Vec<P2>,
    /// Same silhouette without scallops/jimping (aesthetic curvature metrics).
    pub macro_outline: Vec<P2>,
    /// Smooth spline samples tip->tip (before the ring union) and control-point sample indices.
    pub curve: Vec<P2>, pub cidx: Vec<usize>,
    /// [ring, comet, eye?, cairn]
    pub holes: Vec<Vec<P2>>, pub hole_names: Vec<&'static str>,
    pub t: f64, pub mat: TiGrade, pub tip: P2, pub heel: P2, pub dx: f64,
    pub edge_in: Vec<P2>, pub edge_sp: Vec<P2>, pub blade: Blade,
    pub grip_len: f64, pub ring_od: f64, pub ring_id: f64, pub pad: [P2; 4],
    pub teeth_in: usize, pub teeth_sp: usize, pub ripple_in: f64, pub ripple_sp: f64,
    pub field: Field,
    pub mass_g: f64, pub blank_g: f64, pub area_mm2: f64,
    /// Volume removed (mm^3): relief, lattice, fuller, flute, texture; and web-floor clamped cells.
    pub removed: [f64; NREM], pub web_clamped: usize, pub min_web: f64,
    /// Pierced-fuller through-windows: count and planform area (mm^2).
    pub n_win: usize, pub win_area: f64,
    /// Perforation-lattice through-holes: count and planform area (mm^2).
    pub n_perf: usize, pub perf_area: f64,
    pub land_full_mm2: f64, pub pad_flat: bool,
    /// Rev C.6 ring-face groove: radial width (mm, 0 = absent) and planform area per face (mm^2).
    pub groove_w: f64, pub groove_area: f64,
    /// D1 P1: thinnest centreline knot (joint thin+hot factor uses it; = t without P1), shallow-pocket
    /// planform area (mm^2, per face) and min rail width actually kept full-thickness (mm).
    pub t_joint: f64, pub pocket_area: f64, pub p1: bool, pub rs_v2: bool,
    pub fails: Vec<String>,
    /// RESEARCH, default None: surface-tension pocket smoothing (JARAMBIT_POCKET_SMOOTH, rs_v2 only).
    pub pocket_smooth: Option<std::sync::Arc<crate::pocket_smooth::SmoothField>>,
}

/// D1 P1 parent safety-factor map (JARAMBIT_P1_SFMAP): raster of the signed distance (mm) INTO the region
/// where the parent's certified 100 C envelope SF > 2 (<= 0 outside). Text: `x0 y0 g nx ny` then nx*ny values,
/// row-major (j outer). Produced by ops/d1/make_sfmap.py from the parent's D1 plate JSON.
pub struct SfMap { pub x0: f64, pub y0: f64, pub g: f64, pub nx: usize, pub ny: usize, pub d: Vec<f32> }
impl SfMap {
    pub fn at(&self, q: P2) -> f64 {
        let i = ((q.0 - self.x0) / self.g).floor(); let j = ((q.1 - self.y0) / self.g).floor();
        if i < 0.0 || j < 0.0 || i as usize >= self.nx || j as usize >= self.ny { return -1.0; }
        self.d[j as usize * self.nx + i as usize] as f64
    }
}
pub fn sfmap() -> Option<&'static SfMap> {
    static M: std::sync::OnceLock<Option<SfMap>> = std::sync::OnceLock::new();
    M.get_or_init(|| {
        let path = std::env::var("JARAMBIT_P1_SFMAP").ok()?; let txt = std::fs::read_to_string(&path).ok()?;
        let v: Vec<f64> = txt.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        if v.len() < 5 { return None; }
        let (nx, ny) = (v[3] as usize, v[4] as usize); if v.len() < 5 + nx * ny { return None; }
        Some(SfMap { x0: v[0], y0: v[1], g: v[2], nx, ny, d: v[5..5 + nx * ny].iter().map(|x| *x as f32).collect() })
    }).as_ref()
}

fn scallop_depth(table: &[(f64, f64, f64)], s: f64, dscale: &dyn Fn(f64, f64) -> f64) -> f64 {
    for &(a, p, d0) in table { if s >= a && s <= a + p {
        let d = dscale(p, d0); if d <= 0.0 { return 0.0; }
        let r = (p * p / 4.0 + d * d) / (2.0 * d); let x = s - a - p / 2.0;
        return ((r * r - x * x).max(0.0).sqrt() - (r - d)).max(0.0); } }
    0.0
}

pub fn control_points(g: &Genome) -> Vec<P2> {
    let base: Vec<P2> = c4::SPLINE_P.to_vec();
    let orient = signed_area(&base[..39]).signum();
    let mut p = base.clone();
    for i in 0..N_CP { let d = g.v[i]; if d == 0.0 { continue; }
        let prev = if i == 0 { base[38] } else { base[i - 1] }; let next = if i == 39 { base[1] } else { base[i + 1] };
        let t = norm(sub(next, prev)); let n_out = if orient > 0.0 { (t.1, -t.0) } else { (-t.1, t.0) };
        p[i] = add(p[i], mul(n_out, d)); }
    let dx = g.v[GRIP_DX];
    for i in (0..=13).chain(28..=39) { p[i].0 += dx; }
    let a = -g.v[DROP].to_radians(); let (ca, sa) = (a.cos(), a.sin()); let pv = p[8];
    for i in (0..=7).chain(31..=39) { let q = sub(p[i], pv); p[i] = add(pv, (ca * q.0 - sa * q.1, sa * q.0 + ca * q.1)); }
    p
}

/// Replace the run of `c` inside the ring disk (r_od about the origin) by tangent fillets + ring arc.
fn ring_union(c: &[P2], n_out: &[P2], r_od: f64, rf: f64) -> Option<(Vec<P2>, usize, usize)> {
    let inside: Vec<bool> = c.iter().map(|q| q.0.hypot(q.1) < r_od).collect();
    let a = inside.iter().position(|&b| b)?; let b = inside.iter().rposition(|&b| b)?;
    if inside[a..=b].iter().any(|&x| !x) { return None; }
    let f = |j: usize| { let o = add(c[j], mul(n_out[j], rf)); o.0.hypot(o.1) - (r_od + rf) };
    let mut ja = None; for j in (0..a).rev() { if f(j) >= 0.0 { ja = Some(j); break; } }
    let mut jb = None; for j in b + 1..c.len() { if f(j) >= 0.0 { jb = Some(j); break; } }
    let (ja, jb) = (ja?, jb?);
    let fillet = |j: usize, to_ring: bool| -> (Vec<P2>, f64) {
        let o = add(c[j], mul(n_out[j], rf)); let lo = o.0.hypot(o.1); let tr = mul(o, r_od / lo);
        let (p1, p2) = if to_ring { (c[j], tr) } else { (tr, c[j]) };
        let a1 = (p1.1 - o.1).atan2(p1.0 - o.0); let mut a2 = (p2.1 - o.1).atan2(p2.0 - o.0);
        while a2 - a1 > PI { a2 -= 2.0 * PI; } while a1 - a2 > PI { a2 += 2.0 * PI; }
        let k = (((a2 - a1).abs() * rf) / 0.3).ceil().max(4.0) as usize;
        ((1..k).map(|i| { let th = a1 + (a2 - a1) * i as f64 / k as f64; (o.0 + rf * th.cos(), o.1 + rf * th.sin()) }).collect(), tr.1.atan2(tr.0))
    };
    let (fa, tha) = fillet(ja, true); let (fb, thb) = fillet(jb, false);
    // ring arc from tha to thb passing the rear (angle pi)
    let mut d = thb - tha; while d > 0.0 { d -= 2.0 * PI; } while d <= -2.0 * PI { d += 2.0 * PI; }
    if ((tha + d / 2.0).cos()) > 0.0 { d += 2.0 * PI; }
    let k = ((d.abs() * r_od) / 0.3).ceil() as usize;
    let mut out: Vec<P2> = c[..=ja].to_vec();
    out.extend(fa);
    for i in 0..=k { let th = tha + d * i as f64 / k as f64; out.push((r_od * th.cos(), r_od * th.sin())); }
    out.extend(fb);
    out.extend_from_slice(&c[jb..c.len() - 1]); // last sample == tip == first sample
    Some((out, ja, jb))
}

/// Blade medial line (fuller centreline): midpoints between the inside edge (tip..heel samples) and
/// the nearest spine sample, over inside-edge fraction [s0, s1] measured from the heel (0) to the tip (1).
pub fn medial_line(edge_in: &[P2], spine: &[P2], s0: f64, s1: f64) -> Vec<P2> {
    let m = edge_in.len(); let spd = densify(spine, 0.5, false);
    let mut pts = Vec::new();
    for k in 0..200 { let u = s0 + (s1 - s0) * k as f64 / 199.0; let q = edge_in[((1.0 - u) * (m - 1) as f64) as usize];
        let sp = spd.iter().copied().min_by(|a, b| dist(*a, q).partial_cmp(&dist(*b, q)).unwrap()).unwrap();
        pts.push(((q.0 + sp.0) / 2.0, (q.1 + sp.1) / 2.0)); }
    densify(&pts, 0.2, false)
}

pub fn build(g: &Genome) -> Pheno {
    let mut fails: Vec<String> = Vec::new();
    let cp = control_points(g);
    let (c, cidx) = natural_spline(&cp, 0.1);
    let n = c.len();
    let seg = cumlen(&c);
    let orient = signed_area(&c[..n - 1]).signum();
    let tan: Vec<P2> = (0..n).map(|i| norm(sub(c[(i + 1).min(n - 1)], c[i.saturating_sub(1)]))).collect();
    // inward normal: interior is left of travel for CCW, right for CW
    let n_in: Vec<P2> = tan.iter().map(|t| if orient > 0.0 { (-t.1, t.0) } else { (t.1, -t.0) }).collect();
    let n_out: Vec<P2> = n_in.iter().map(|q| (-q.0, -q.1)).collect();
    let dx = g.v[GRIP_DX];
    let tip = c[0]; let i_heel = cidx[8]; let heel = c[i_heel];
    let in_len = seg[i_heel];
    let sp_full = seg[n - 1] - seg[cidx[30]];
    let off = g.v[SPINE_OFF].max(sp_full - (CAP_MM - 0.3)).max(0.0);
    let i_sp0 = (cidx[30]..n).find(|&i| seg[i] - seg[cidx[30]] >= off).unwrap_or(n - 2);
    let arc_sp = seg[n - 1] - seg[i_sp0];
    let blade = Blade { tip_to_heel: dist(tip, heel), horizontal_reach: tip.0 - heel.0, arc_in: in_len, arc_sp,
        sharpened_arc: in_len.max(arc_sp), spine_offset_used: off, hook_drop: heel.1 - tip.1 };
    // ---- serrations (C.4 tables, depth genes) + thumb-seat jimping ----
    let mut depth = vec![0.0; n];
    let (dl, ds, df, js) = (g.v[SER_L], g.v[SER_S], g.v[SER_F], g.v[JIMP]);
    let din = |p: f64, d0: f64| if p > 4.0 { dl } else { ds * d0 / 0.7 };
    let dsp = |_p: f64, d0: f64| df * d0 / 0.75;
    for i in 0..=i_heel { depth[i] = scallop_depth(&c4::SC_IN, in_len - seg[i], &din); }
    for i in i_sp0..n { depth[i] = scallop_depth(&c4::SC_SP, seg[i] - seg[i_sp0], &dsp); }
    // Rev C.5 jimping: 7 notches, 2.0 mm pitch, R0.8, UNIFORM depth 0.45 mm x jimp_scale (C.4 was R0.55 graduated 0.45-0.8)
    let s_j0 = seg[cidx[28]] - 7.0; let rj = c4::JIMP_R;
    for j in 0..c4::JIMP_N { let sc = s_j0 + c4::JIMP_PITCH * j as f64; let dj = c4::JIMP_D * js;
        for i in 0..n { let x = seg[i] - sc; if x.abs() <= rj { let d = ((dj - rj) + (rj * rj - x * x).max(0.0).sqrt()).max(0.0); depth[i] = depth[i].max(d); } } }
    let cs: Vec<P2> = (0..n).map(|i| add(c[i], mul(n_in[i], depth[i]))).collect();
    let teeth_in = c4::SC_IN.len(); let teeth_sp = c4::SC_SP.len();
    let rip = |r: std::ops::Range<usize>| { let v: Vec<f64> = depth[r].to_vec(); let m = v.iter().sum::<f64>() / v.len().max(1) as f64; (v.iter().map(|d| (d - m).powi(2)).sum::<f64>() / v.len().max(1) as f64).sqrt() };
    let (ripple_in, ripple_sp) = (rip(0..i_heel), rip(i_sp0..n));
    // ---- ring union + neck fillets ----
    let ring_id = g.v[RING_ID]; let ring_od = ring_id + 2.0 * g.v[RING_WALL];
    let rf = g.v[NECK_FILLET];
    let (outline, macro_outline) = match (ring_union(&cs, &n_out, ring_od / 2.0, rf), ring_union(&c, &n_out, ring_od / 2.0, rf)) {
        (Some(a), Some(b)) => (a.0, b.0),
        _ => { fails.push("Disconnected: ring does not join the crescent".into()); (cs[..n - 1].to_vec(), c[..n - 1].to_vec()) }
    };
    // ---- lightening holes, C.4 zones only ----
    let mut holes = vec![circle((0.0, 0.0), ring_id / 2.0, 256, true)]; let mut hole_names = vec!["ring"];
    let (x0, x1, y0) = (g.v[COMET_X0], g.v[COMET_X1], g.v[COMET_Y]); let l = x1 - x0; let bow = g.v[COMET_BOW];
    let comet_c = bezier([(x0, y0), (x0 + 0.325 * l, y0 + bow), (x0 + 0.675 * l, y0 + 0.83 * bow), (x1, y0 - 0.1)], 80);
    holes.push(taper_slot(&comet_c, g.v[COMET_W0], g.v[COMET_W1])); hole_names.push("comet");
    if g.v[EYE] >= 0.5 { let s = g.v[EYE]; let ctr = (90.0, 1.7);
        let e: Vec<P2> = [(84.0, 2.6), (88.0, 2.4), (92.0, 1.6), (96.0, 0.2)].iter().map(|q| (ctr.0 + s * (q.0 - ctr.0) + dx, ctr.1 + s * (q.1 - ctr.1))).collect();
        holes.push(taper_slot(&bezier([e[0], e[1], e[2], e[3]], 40), 5.0 * s, 2.2 * s)); hole_names.push("eye"); }
    holes.push(circle((85.0 + dx, -15.0), g.v[CAIRN] / 2.0, 64, true)); hole_names.push("cairn");
    // ---- pierced fuller: through-windows on the blade medial line (C.4 fuller band) ----
    let (mut n_win, mut win_area) = (0usize, 0.0f64);
    let nw = g.v[WIN_N].floor().max(0.0) as usize;
    if nw > 0 {
        let maxx = outline.iter().map(|q| q.0).fold(f64::MIN, f64::max);
        let med = medial_line(&c[..=i_heel], &c[cidx[29]..], g.v[WIN_S0], g.v[WIN_S1]);
        let ms = cumlen(&med); let lm = *ms.last().unwrap_or(&0.0);
        let nn_in = Nn::new(&densify(&c[..=i_heel], 0.25, false), 2.0);
        let nn_sp = Nn::new(&densify(&c[cidx[30]..], 0.25, false), 2.0);
        let (t, t_tip, tx0) = (g.v[T], g.v[T_TIP], g.v[TAPER_X0] + dx);
        let (tan_in, tan_sp, et) = (g.v[EA_IN].to_radians().tan(), g.v[EA_SP].to_radians().tan(), g.v[EDGE_TH]);
        let clear = |q: P2| { let b = t - (t - t_tip) * ((q.0 - tx0) / (maxx - tx0)).clamp(0.0, 1.0);
            (nn_in.nearest(q, 40.0) - (b - et) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (b - et) / (2.0 * tan_sp)) - WIN_RIM };
        let pitch = lm / nw as f64;
        if pitch > WIN_MIN_W + WIN_GAP {
            for k in 0..nw {
                let sc = (k as f64 + 0.5) * pitch;
                let ls = (g.v[WIN_FILL] * pitch).min(pitch - WIN_GAP);
                let wk = g.v[WIN_W] * (1.0 + (g.v[WIN_DECAY] - 1.0) * k as f64 / (nw.max(2) - 1) as f64);
                let seg: Vec<P2> = (0..med.len()).filter(|&i| (ms[i] - sc).abs() <= ls / 2.0).map(|i| med[i]).collect();
                if seg.len() < 2 { continue; }
                let cl = seg.iter().map(|&q| clear(q)).fold(f64::INFINITY, f64::min);
                let w = wk.min(2.0 * cl).min(ls);
                if !(w >= WIN_MIN_W) { continue; }
                let core: Vec<P2> = (0..med.len()).filter(|&i| (ms[i] - sc).abs() <= ((ls - w) / 2.0).max(0.0)).map(|i| med[i]).collect();
                let h = if core.len() < 3 || cumlen(&core).last().copied().unwrap_or(0.0) < 0.4 {
                    let ci = (0..med.len()).min_by(|&a, &b| (ms[a] - sc).abs().partial_cmp(&(ms[b] - sc).abs()).unwrap()).unwrap();
                    circle(med[ci], w / 2.0, 48, true)
                } else { taper_slot(&densify(&core, 0.5, false), w, 0.9 * w) };
                win_area += signed_area(&h).abs(); n_win += 1;
                holes.push(h); hole_names.push("window");
            }
        }
    }
    let (pw, pl) = (g.v[PAD_W], g.v[PAD_L]); let pc = (g.v[PAD_X] + dx, g.v[PAD_Y]);
    // ---- perforation lattice (through-holes, guard/land band and/or handle) ----
    let (mut n_perf, mut perf_area) = (0usize, 0.0f64);
    let pd = g.v[PERF_D];
    if pd >= 3.0 && (g.v[PERF_ZL] > 0.5 || g.v[PERF_ZH] > 0.5) {
        let (gap, pa, pe) = (g.v[PERF_GAP], g.v[PERF_A].to_radians(), g.v[PERF_E]);
        let pitch = pd + gap; let (ca, sa) = (pa.cos(), pa.sin());
        let r_od = (g.v[RING_ID] + 2.0 * g.v[RING_WALL]) / 2.0;
        let h0 = (r_od + 3.0).max(18.0); let hx1 = 74.0 + dx; let hx0 = h0 + g.v[PERF_XH] * (hx1 - h0);
        let mut bpts = densify(&outline, 0.25, true); for h in &holes { bpts.extend(densify(h, 0.25, true)); }
        let nn = Nn::new(&bpts, 2.0);
        let padbox = |q: P2, m: f64| q.0 >= pc.0 - pw / 2.0 - m && q.0 <= pc.0 + pw / 2.0 + m && q.1 >= pc.1 - pl / 2.0 - m && q.1 <= pc.1 + pl / 2.0 + m;
        let (a_maj, b_min) = (pe * pd / 2.0, pd / 2.0);
        let cen = (60.0 + dx, 0.0);
        for j in -12i64..=12 { for i in -16i64..=16 {
            let u = (i as f64 + if j.rem_euclid(2) == 1 { 0.5 } else { 0.0 }) * pitch * pe; let v = j as f64 * pitch * 0.866;
            let q = (cen.0 + ca * u - sa * v, cen.1 + sa * u + ca * v);
            let inl = g.v[PERF_ZL] > 0.5 && q.0 >= 78.0 + dx && q.0 <= 100.0 + dx;
            let inh = g.v[PERF_ZH] > 0.5 && q.0 >= hx0 && q.0 <= hx1;
            if !(inl || inh) { continue; }
            if !point_in_poly(q, &outline) || holes.iter().any(|h| point_in_poly(q, h)) { continue; }
            if nn.nearest(q, 40.0) < a_maj + 3.2 || padbox(q, a_maj + 1.2) { continue; }
            let e: Vec<P2> = (0..40).map(|k| { let th = -2.0 * PI * k as f64 / 40.0; let (x, y) = (a_maj * th.cos(), b_min * th.sin()); (q.0 + ca * x - sa * y, q.1 + sa * x + ca * y) }).collect();
            if e.iter().any(|&z| !point_in_poly(z, &outline)) { continue; }
            perf_area += signed_area(&e).abs(); n_perf += 1; holes.push(e); hole_names.push("perf");
        } }
    }
    let pad = [(pc.0 - pw / 2.0, pc.1 - pl / 2.0), (pc.0 + pw / 2.0, pc.1 - pl / 2.0), (pc.0 + pw / 2.0, pc.1 + pl / 2.0), (pc.0 - pw / 2.0, pc.1 + pl / 2.0)];
    let neck_x = ring_od / 2.0 - 3.0;
    let grip_len = cp[13].0 - neck_x;
    let cutting = if g.rs_v2() { &cs } else { &c };
    let edge_in: Vec<P2> = cutting[..=i_heel].to_vec(); let edge_sp: Vec<P2> = cutting[i_sp0..].to_vec();
    let mut p = Pheno { genome: g.clone(), surface_x_max: outline.iter().map(|q|q.0).fold(f64::NEG_INFINITY,f64::max),
        cut_in: Segments::new(&edge_in), cut_sp: Segments::new(&edge_sp), cp, outline, macro_outline, curve: c, cidx, holes, hole_names, t: g.v[T], mat: g.mat, tip, heel, dx,
        edge_in, edge_sp, blade, grip_len, ring_od, ring_id, pad, teeth_in, teeth_sp, ripple_in, ripple_sp,
        field: Field { x0: 0.0, y0: 0.0, g: GRID, nx: 0, ny: 0, th: vec![], base: vec![] }, mass_g: 0.0, blank_g: 0.0, area_mm2: 0.0,
        removed: [0.0; NREM], n_win, win_area, n_perf, perf_area, web_clamped: 0, min_web: 0.0, land_full_mm2: 0.0, pad_flat: false, groove_w: 0.0, groove_area: 0.0,
        t_joint: g.t_joint(), pocket_area: 0.0, p1: g.p1(), rs_v2: g.rs_v2(), fails, pocket_smooth: None };
    thickness_field(g, &mut p);
    p
}

fn density(m: TiGrade) -> f64 { jarambit_physics::material(m).density }

/// 0.25 mm raster: distal taper, flat grinds from the SMOOTH edges (edge angle genes), then the
/// 3-D feature layer (medial-axis section relief, hex lattice dimples, blade fuller, comet-ligament
/// flutes, machined grip texture). Brand pad and the holder friction land are protected (full t).
fn thickness_field(g: &Genome, p: &mut Pheno) {
    let gsz = GRID;
    let (mut lo, mut hi) = ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN));
    for q in &p.outline { lo.0 = lo.0.min(q.0); lo.1 = lo.1.min(q.1); hi.0 = hi.0.max(q.0); hi.1 = hi.1.max(q.1); }
    let (x0, y0) = (lo.0 - gsz, lo.1 - gsz);
    let nx = ((hi.0 - x0) / gsz).ceil() as usize + 2; let ny = ((hi.1 - y0) / gsz).ceil() as usize + 2;
    let mut th = vec![f32::NAN; nx * ny]; let mut base_v = vec![f32::NAN; nx * ny];
    let rings: Vec<&Vec<P2>> = std::iter::once(&p.outline).chain(p.holes.iter()).collect();
    let t = p.t; let t_tip = g.v[T_TIP]; let tx0 = g.v[TAPER_X0] + p.dx; let maxx = hi.0;
    let (tan_in, tan_sp) = (g.v[EA_IN].to_radians().tan(), g.v[EA_SP].to_radians().tan());
    let et = g.v[EDGE_TH];
    let rs_v2 = p.rs_v2;
    let et_std = et;
    let nn_in = Nn::new(&densify(&p.edge_in, 0.25, false), 2.0);
    let nn_sp = Nn::new(&densify(&p.edge_sp, 0.25, false), 2.0);
    let mut bpts: Vec<P2> = densify(&p.outline, 0.25, true);
    for h in &p.holes { bpts.extend(densify(h, 0.25, true)); }
    let nn_b = Nn::new(&bpts, 2.0);
    let pad = p.pad; let in_pad = |q: P2, m: f64| q.0 >= pad[0].0 - m && q.0 <= pad[1].0 + m && q.1 >= pad[0].1 - m && q.1 <= pad[2].1 + m;
    let land = |q: P2| q.0 >= 78.0 + p.dx && q.0 <= 100.0 + p.dx;          // holder friction land / ricasso band
    let handle = |q: P2| q.0 < 78.0 + p.dx;
    let r_od = p.ring_od / 2.0;
    // lattice centres (hex, rotated), with rim checks
    let (ld, lp, lf, la) = (g.v[LAT_D], g.v[LAT_P], g.v[LAT_F], g.v[LAT_A].to_radians());
    let lr = lf * lp / 2.0;
    let mut dimples: Vec<P2> = Vec::new();
    if ld > 0.05 && (g.v[LAT_H] > 0.5 || g.v[LAT_B] > 0.5) {
        let (ca, sa) = (la.cos(), la.sin()); let rr = 0.5 * ((hi.0 - lo.0).hypot(hi.1 - lo.1)) + lp;
        let cen = ((lo.0 + hi.0) / 2.0, (lo.1 + hi.1) / 2.0);
        let nj = (rr / (lp * 0.866)) as i64 + 1; let ni = (rr / lp) as i64 + 1;
        for j in -nj..=nj { for i in -ni..=ni {
            let u = (i as f64 + if j % 2 != 0 { 0.5 } else { 0.0 }) * lp; let v = j as f64 * lp * 0.866;
            let q = (cen.0 + ca * u - sa * v, cen.1 + sa * u + ca * v);
            if q.0 < lo.0 || q.0 > hi.0 || q.1 < lo.1 || q.1 > hi.1 { continue; }
            if !crate::geom::point_in_poly(q, &p.outline) || p.holes.iter().any(|h| crate::geom::point_in_poly(q, h)) { continue; }
            if nn_b.nearest(q, 30.0) < lr + 1.5 || in_pad(q, lr + 1.0) || land(q) || q.0.hypot(q.1) < r_od + lr + 1.0 { continue; }
            let hz = g.v[LAT_H] > 0.5 && q.0 >= 18.0 && q.0 <= 74.0 + p.dx;
            let bz = g.v[LAT_B] > 0.5 && q.0 >= 102.0 + p.dx && {
                let b = t - (t - t_tip) * ((q.0 - tx0) / (maxx - tx0)).clamp(0.0, 1.0);
                nn_in.nearest(q, 40.0) >= (b - et_std) / (2.0 * tan_in) + lr + 1.0 && nn_sp.nearest(q, 40.0) >= (b - et_std) / (2.0 * tan_sp) + lr + 1.0 };
            if hz || bz { dimples.push(q); } } }
    }
    let nn_dim = if dimples.is_empty() { None } else { Some(Nn::new(&dimples, lp.max(2.0))) };
    // fuller centreline: midpoints between inside edge and spine over [s0, s1] of the inside edge
    let (fd, fw) = (g.v[FUL_D], g.v[FUL_W]);
    let nn_ful = if fd > 0.05 {
        let ei = &p.edge_in; let m = ei.len(); let spd = densify(&p.curve[p.cidx[29]..], 0.5, false);
        let mut pts = Vec::new();
        let (s0, s1) = (g.v[FUL_S0], g.v[FUL_S1]);
        for k in 0..200 { let u = s0 + (s1 - s0) * k as f64 / 199.0; let q = ei[((1.0 - u) * (m - 1) as f64) as usize];
            let sp = spd.iter().copied().min_by(|a, b| dist(*a, q).partial_cmp(&dist(*b, q)).unwrap()).unwrap();
            pts.push(((q.0 + sp.0) / 2.0, (q.1 + sp.1) / 2.0)); }
        Some(Nn::new(&densify(&pts, 0.2, false), 2.0)) } else { None };
    let (rd, rw, rq) = (g.v[RELIEF_D], g.v[RELIEF_W], g.v[RELIEF_Q]);
    let (tm, tdp, tpi, tang) = (g.v[TEX_MODE] >= 0.5, g.v[TEX_D], g.v[TEX_P], g.v[TEX_A].to_radians());
    let (wx0, wx1) = (g.v[WRAP_X0], g.v[WRAP_X0] + g.v[WRAP_L]);
    let web_min = (0.3 * t).max(1.5);
    let mut removed = [0.0f64; NREM]; let mut clamped = 0usize; let mut min_web = f64::MAX;
    // ---- isogrid (two-sided triangular pockets) ----
    let (iso_d, iso_p, iso_rib, iso_a, iso_rim, iso_gr) = (g.v[ISO_D], g.v[ISO_P], g.v[ISO_RIB], g.v[ISO_A].to_radians(), g.v[ISO_RIM], g.v[ISO_GR]);
    let iso_on = iso_d > 0.05 && (g.v[ISO_ZB] > 0.5 || g.v[ISO_ZH] > 0.5 || g.v[ISO_ZL] > 0.5);
    let iso_n: [P2; 3] = [0usize, 1, 2].map(|k| { let ph = iso_a + std::f64::consts::FRAC_PI_2 + k as f64 * PI / 3.0; (ph.cos(), ph.sin()) });
    let iso_wh = iso_rib / 2.0 + POCKET_RC;
    let bx0 = 102.0 + p.dx; let bx1 = bx0 + g.v[ISO_XB] * (maxx - bx0); let hx1 = 74.0 + p.dx; let hx0 = 18.0 + g.v[ISO_XH] * (hx1 - 18.0); let (lx0, lx1) = (78.0 + p.dx, 100.0 + p.dx);
    let iso_depth = |q: P2| -> f64 {
        let (inb, inh, inl) = (g.v[ISO_ZB] > 0.5 && q.0 >= bx0 && q.0 <= bx1, g.v[ISO_ZH] > 0.5 && q.0 >= hx0 && q.0 <= hx1, g.v[ISO_ZL] > 0.5 && q.0 >= lx0 && q.0 <= lx1);
        if !(inb || inh || inl) { return 0.0; }
        let mut s2 = 0.0;
        for nk in &iso_n { let z = q.0 * nk.0 + q.1 * nk.1; let dk = (z - iso_p * (z / iso_p).round()).abs(); let e = dk - iso_wh; if e < 0.0 { s2 += e * e; } }
        if s2.sqrt() > POCKET_RC { return 0.0; }
        let u = if inb { (q.0 - bx0) / (bx1 - bx0).max(1.0) } else if inh { (q.0 - hx0) / (hx1 - hx0).max(1.0) } else { 0.5 };
        iso_d * (1.0 + iso_gr * (1.0 - 2.0 * u.clamp(0.0, 1.0))).clamp(0.0, 1.5)
    };
    // ---- ring-face flute (ball-nose groove, both faces) ----
    let (rfd, rfw, rfa) = (g.v[RFL_D], g.v[RFL_W], g.v[RFL_ARC].to_radians() / 2.0);
    let r_id = p.ring_id / 2.0; let rmid = (r_id + r_od) / 2.0; let rhw = rfw * (r_od - r_id) / 2.0;
    let ring_flute = |q: P2| -> f64 {
        if rfd <= 0.05 { return 0.0; }
        let dr = q.0.hypot(q.1) - rmid; if dr.abs() >= rhw { return 0.0; }
        let da = PI - q.1.atan2(q.0).abs(); if da >= rfa { return 0.0; }
        let run = ((rfa - da) / 8f64.to_radians()).clamp(0.0, 1.0);
        rfd * run * (1.0 - (dr / rhw).powi(2))
    };
    // ---- Rev C.6 ring-face racetrack groove (two-sided annulus, rims to bore + OD; part of the anchor) ----
    let (g_r0, g_r1) = (r_id + c4::RING_GROOVE_RIM, r_od - c4::RING_GROOVE_RIM);
    let groove_on = c4::active() && g_r1 - g_r0 >= c4::RING_GROOVE_MIN_W - 1e-9;
    let mut groove_area = 0.0f64;
    let mut land_full = 0.0; let mut pad_flat = true; let mut area = 0.0;
    // ---- D1 P1: continuous rails + full neck + shallow pockets (parent SF > 2 only, >= R3 runouts) ----
    let p1 = g.p1(); let (rail_w, neck_r, pk_d, pk_r) = (g.v[RAIL_W], r_od + g.v[NECK_L], g.v[POCKET_D], g.v[POCKET_R]);
    let sfm = if p1 { sfmap() } else { None };
    let mut pocket_area = 0.0f64;
    let smooth = |u: f64| { let u = u.clamp(0.0, 1.0); u * u * (3.0 - 2.0 * u) };
    for j in 0..ny {
        let y = y0 + (j as f64 + 0.5) * gsz;
        let mut xs: Vec<f64> = Vec::new();
        for r in &rings { let m = r.len(); for i in 0..m { let (a, b) = (r[i], r[(i + 1) % m]);
            if (a.1 > y) != (b.1 > y) { xs.push(a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1)); } } }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for pr in xs.chunks(2) { if pr.len() < 2 { continue; }
            let i0 = (((pr[0] - x0) / gsz) - 0.5).ceil().max(0.0) as usize; let i1 = (((pr[1] - x0) / gsz) - 0.5).floor() as i64;
            if i1 < i0 as i64 { continue; }
            for i in i0..=(i1 as usize).min(nx - 1) {
                let q = (x0 + (i as f64 + 0.5) * gsz, y);
                let base = if p1 { let kt = g.knot_t(q.0.min(tx0), tx0); kt - (kt - t_tip) * ((q.0 - tx0) / (maxx - tx0)).clamp(0.0, 1.0) }
                    else { t - (t - t_tip) * ((q.0 - tx0) / (maxx - tx0)).clamp(0.0, 1.0) };
                let d_in = nn_in.nearest(q, 14.0).min(1e3);
                let d_sp = nn_sp.nearest(q, 14.0).min(1e3);
                let gi = et_std + 2.0 * d_in * tan_in;
                let gs = et_std + 2.0 * d_sp * tan_sp;
                let th_g = base.min(gi).min(gs); let grind = th_g < base - 1e-9;
                let rq_ = q.0.hypot(q.1);
                let in_groove = groove_on && !grind && rq_ >= g_r0 && rq_ <= g_r1;
                let th0 = if in_groove { groove_area += gsz * gsz; th_g - 2.0 * c4::RING_GROOVE_D } else { th_g };
                let prot = in_pad(q, 1.0) || land(q);
                let db = nn_b.nearest(q, 12.0);
                let mut rem = [0.0f64; NREM];
                let mut pk_f = 0.0f64;
                let prot_iso = in_pad(q, 1.0) || (land(q) && g.v[ISO_ZL] <= 0.5);
                if iso_on && !prot_iso && !grind && db >= iso_rim && !in_pad(q, iso_rim) && q.0.hypot(q.1) > r_od + iso_rim
                    && nn_in.nearest(q, 40.0) >= (base - et_std) / (2.0 * tan_in) + iso_rim && nn_sp.nearest(q, 40.0) >= (base - et_std) / (2.0 * tan_sp) + iso_rim {
                    rem[5] = 2.0 * iso_depth(q); }
                if !prot && !grind { rem[6] = 2.0 * ring_flute(q); }
                if p1 && pk_d > 0.01 && !prot && !in_pad(q, pk_r) && !grind {
                    // rails (outline + every hole rim, ring OD -> heel and beyond) and the neck stay full-thickness;
                    // pocket floor ramps in over >= pk_r (R3+ runout) from rails, neck, grind lines and the SF<=2 boundary
                    let d_rail = nn_b.nearest(q, rail_w + pk_r + 1.0) - rail_w;
                    let d_neck = q.0.hypot(q.1) - neck_r;
                    let d_grind = (nn_in.nearest(q, 40.0) - (base - et_std) / (2.0 * tan_in)).min(nn_sp.nearest(q, 40.0) - (base - et_std) / (2.0 * tan_sp));
                    let d_sf = sfm.map_or(-1.0, |m| m.at(q));
                    let d_land = if land(q) { -1.0 } else { f64::INFINITY };
                    let dm = d_rail.min(d_neck).min(d_grind).min(d_sf).min(d_land);
                    if dm > 0.0 { let f = smooth(dm / pk_r); if f > 0.0 { pk_f = f; rem[5] = 2.0 * pk_d * f; pocket_area += gsz * gsz; } }
                }
                if !prot && !grind && handle(q) && rd > 0.0 && db < rw { rem[0] = 2.0 * rd * (1.0 - db / rw).powf(rq); }
                if let Some(nd) = &nn_dim { if !prot && !grind { let d = nd.nearest(q, lr); if d < lr { rem[1] = 2.0 * ld * (1.0 - (d / lr).powi(2)); } } }
                if let Some(nf) = &nn_ful { if !prot && !grind && q.0 > p.heel.0 - 5.0 { let d = nf.nearest(q, fw / 2.0); if d < fw / 2.0 { rem[2] = 2.0 * fd * (1.0 - (2.0 * d / fw).powi(2)); } } }
                if tm && !prot && q.0 >= wx0 && q.0 <= wx1 && db >= 1.0 && q.0.hypot(q.1) > r_od && !grind {
                    let ph = ((q.0 * tang.cos() + q.1 * tang.sin()) / tpi).rem_euclid(1.0);
                    if ph < 0.5 { rem[4] = 2.0 * tdp * (PI * ph / 0.5).sin(); } }
                let tot: f64 = rem.iter().sum();
                let mut thv_std = th0 - tot;
                if !grind { if thv_std < web_min { if tot > 0.0 { clamped += 1; } thv_std = web_min.min(th0); } min_web = min_web.min(thv_std); }
                let removed_here = th0 - thv_std;
                if tot > 0.0 { for k in 0..NREM { removed[k] += removed_here * rem[k] / tot * gsz * gsz; } }
                let mut thv = thv_std;
                if rs_v2 {
                    if in_groove {
                        thv = (thv - 2.0 * 0.58).max(web_min);
                    } else if pk_f > 0.0 {
                        thv = (thv - 2.0 * 0.28 * pk_f).max(web_min);
                    } else if rem[5] > 0.0 {
                        thv = (thv - 2.0 * 0.36).max(web_min);
                    }
                }
                if rs_v2 { thv = bore_collar_thickness(p, q, thv); }
                th[j * nx + i] = thv as f32; base_v[j * nx + i] = (if rs_v2 { thv_std } else { base }) as f32; area += gsz * gsz;
                // P1: 'full thickness' = the local knot-field base (a planar ground face), not the max stock
                let full = if p1 { base } else { t };
                if land(q) && !grind && (thv_std - full).abs() < 1e-6 { land_full += gsz * gsz; }
                if in_pad(q, 0.0) && (thv_std - full).abs() > 1e-6 { pad_flat = false; }
            } }
    }
    // flutes on the two comet ligaments (C.2 P1, x 28..46): column runs, flat-bottom, rim >= 2.3 mm
    let fl = g.v[FLUTE];
    if fl > 0.05 {
        for i in 0..nx { let x = x0 + (i as f64 + 0.5) * gsz; if x < 28.0 || x > 46.0 { continue; }
            let mut runs: Vec<(usize, usize)> = Vec::new(); let mut s: Option<usize> = None;
            for j in 0..ny { let inside = !th[j * nx + i].is_nan(); match (inside, s) { (true, None) => s = Some(j), (false, Some(a)) => { runs.push((a, j - 1)); s = None; } _ => {} } }
            if let Some(a) = s { runs.push((a, ny - 1)); }
            if runs.len() < 2 { continue; }
            for (run, w) in [(runs[runs.len() - 1], 2.2), (runs[0], 2.6)] {
                let len = (run.1 - run.0 + 1) as f64 * gsz; if len < w + 4.6 { continue; }
                let mid = (run.0 + run.1) as f64 / 2.0;
                for j in run.0..=run.1 { if ((j as f64 - mid) * gsz).abs() < w / 2.0 {
                    let k = j * nx + i; let old = th[k] as f64; let new = (old - 2.0 * fl).max(web_min);
                    if new < old { removed[3] += (old - new) * gsz * gsz; th[k] = new as f32; min_web = min_web.min(new); }
                    if rs_v2 && !base_v[k].is_nan() { base_v[k] = (base_v[k] as f64 - 2.0 * fl).max(web_min) as f32; }
                } } } }
    }
    p.field = Field { x0, y0, g: gsz, nx, ny, th, base: base_v };
    if rs_v2 {
        if let Some(prm) = crate::pocket_smooth::params_from_env() {
            p.pocket_smooth = Some(crate::pocket_smooth::cached(p, &prm));
        }
    }
    if rs_v2 {
        let actual: Vec<f32> = p.field.th.iter().enumerate().map(|(k, &v)| {
            if v.is_nan() { return v; }
            let q = (x0 + (k % nx) as f64 * gsz + gsz / 2.0,
                     y0 + (k / nx) as f64 * gsz + gsz / 2.0);
            crate::export::thickness_at(p, q) as f32
        }).collect();
        p.field.th = actual;
    }
    let rho = density(p.mat);
    let vol: f64 = p.field.th.iter().filter(|v| !v.is_nan()).map(|&v| v as f64 * gsz * gsz).sum();
    p.mass_g = vol * rho / 1000.0; p.blank_g = area * t * rho / 1000.0; p.area_mm2 = area;
    p.removed = removed; p.web_clamped = clamped; p.min_web = if min_web == f64::MAX { t } else { min_web };
    p.land_full_mm2 = land_full; p.pad_flat = pad_flat;
    p.groove_w = if groove_on { g_r1 - g_r0 } else { 0.0 }; p.groove_area = groove_area;
    p.pocket_area = pocket_area;
}

/// Integral bore collar dimensions derived exclusively from the existing ring-wall gene.
/// A quarter-circle roll on each face leaves a straight finger-bearing length of twice the wall.
/// These are construction rules, not requirement limits or strength multipliers.
pub fn bore_dimensions(p: &Pheno) -> (f64, f64) {
    let wall = (p.ring_od - p.ring_id) / 2.0;
    (wall / 4.0, 2.0 * wall)
}

pub fn bore_collar_thickness(p: &Pheno, q: P2, substrate: f64) -> f64 {
    let (radius, bearing) = bore_dimensions(p);
    let d = q.0.hypot(q.1) - p.ring_id / 2.0;
    // The vertical tangent amplifies sub-ulp radial noise through sqrt; snap only the CAD boundary.
    let d = if d.abs()<1e-9 {0.0}else{d};
    if d < -1e-8 { return substrate; }
    if d <= radius + 1e-9 {
        let d = d.clamp(0.0, radius);
        return bearing + 2.0 * (radius * radius - (radius - d).powi(2)).max(0.0).sqrt();
    }
    if d <= 1.5 * radius + 1e-9 { return bearing + 2.0 * radius; }
    let wall = (p.ring_od - p.ring_id) / 2.0;
    let u = ((d - 1.5 * radius) / (wall - 1.5 * radius)).clamp(0.0, 1.0);
    let roll_out = (bearing + 2.0 * radius) * (1.0 - u * u * (3.0 - 2.0 * u));
    substrate.max(roll_out)
}

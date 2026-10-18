//! Watertight variable-thickness solid (top z=+t/2, bottom z=-t/2 over a CDT of the planform,
//! side walls on every boundary edge) -> ASCII STL and STEP with analytic toroidal bore rolls.
use crate::geom::*;
use crate::pheno::*;
use jarambit_physics::fine;
use std::collections::{BTreeMap,HashMap};
type V3 = (f64, f64, f64);

#[derive(Clone, Debug)]
pub struct Solid { pub verts: Vec<V3>, pub tris: Vec<[usize; 3]>, pub bore: Option<BorePatch> }
#[derive(Clone, Debug)]
pub struct BorePatch {
    pub radius: f64, pub bearing: f64, pub inner_radius: f64,
    /// [outer-flat top/bottom, outer-roll top/bottom, bore top/bottom], ordered around the bore.
    pub profiles: Vec<[usize;6]>,
    pub faceted_faces: usize,
}

/// The same physical thickness law feeds CAD and every plate solve; never use `Field.base`.
pub fn baseline_thickness_at(p: &Pheno, q: P2) -> f64 {
    let raw = p.field.at(q);
    if !p.rs_v2 { return raw.max(0.3); }
    use crate::genome::*;
    let g = &p.genome;
    let xt = g.v[TAPER_X0] + p.dx;
    let xmax = p.surface_x_max;
    let stock = g.knot_t(q.0.min(xt), xt);
    let base = stock - (stock - g.v[T_TIP]) * ((q.0 - xt) / (xmax - xt)).clamp(0.0, 1.0);
    let ti=g.v[EA_IN].to_radians().tan();let ts=g.v[EA_SP].to_radians().tan();
    let di=p.cut_in.nearest(q,((base-g.v[EDGE_TH])/(2.0*ti)).max(0.0));
    let ds=p.cut_sp.nearest(q,((base-g.v[EDGE_TH])/(2.0*ts)).max(0.0));
    let grind=(g.v[EDGE_TH]+2.0*di*ti).min(g.v[EDGE_TH]+2.0*ds*ts);
    // Pocket/relief operations exclude the grinds. Evaluate their continuous faces exactly,
    // including the scallop boundaries; nearest raster cells cannot define an apex.
    let mut value = if grind < base { grind } else { raw };
    let (radius, _) = bore_dimensions(p);
    if q.0.hypot(q.1) <= p.ring_id / 2.0 + 1.5 * radius + 1e-9 {
        value = bore_collar_thickness(p, q, value);
    }
    value.max(0.05)
}

pub fn thickness_at(p: &Pheno, q: P2) -> f64 { crate::compound::thickness_at(p,q) }

pub fn solid(p: &Pheno, max_area: f64) -> Option<Solid> {
    let outer=dp_closed(&p.outline,crate::compound::config().boundary_tol);
    let (radius,bearing)=bore_dimensions(p);
    let holes:Vec<Vec<P2>>=p.holes.iter().enumerate().map(|(k,h)| {
        if p.rs_v2 && k==0 {circle((0.0,0.0),p.ring_id/2.0+1.5*radius,256,true)}
        else {dp_closed(h,0.02)}
    }).collect();
    // The exported tip mesh itself is the source of the independent solid submodel.
    let m=crate::compound::export_mesh(p,&outer,&holes,if p.rs_v2 {max_area.min(crate::compound::config().export_area)}else{max_area})?;
    let n=m.xy.len();
    let mut verts:Vec<V3>=m.xy.iter().map(|&q|(q.0,q.1,thickness_at(p,q)/2.0)).collect();
    verts.extend(m.xy.iter().map(|&q|(q.0,q.1,-thickness_at(p,q)/2.0)));
    let mut tris=Vec::new();let mut cnt:BTreeMap<(usize,usize),(usize,(usize,usize))>=BTreeMap::new();
    for t in &m.tris {
        tris.push(*t);tris.push([t[0]+n,t[2]+n,t[1]+n]);
        for k in 0..3 {let(a,b)=(t[k],t[(k+1)%3]);let e=cnt.entry((a.min(b),a.max(b))).or_insert((0,(a,b)));e.0+=1;}
    }
    let mut bore_edges=Vec::new();
    for (_, (count,(a,b))) in cnt {if count==1 {
        let on_bore=|i:usize|p.rs_v2 && holes[0].iter().enumerate().any(|(k,&q)|
            seg_dist(m.xy[i],q,holes[0][(k+1)%holes[0].len()])<1e-7);
        if on_bore(a)&&on_bore(b){bore_edges.push((a,b));}
        else {tris.extend([[a,a+n,b+n],[a,b+n,b]]);}
    }}
    let mut bore=None;
    if p.rs_v2 {
        let faceted_faces=tris.len();let mut profiles:BTreeMap<usize,(Vec<usize>,Vec<usize>)>=BTreeMap::new();
        const SEGMENTS:usize=16;
        for &(a,b) in &bore_edges {for i in [a,b] {
            profiles.entry(i).or_insert_with(|| {
                let q=m.xy[i];let normal=mul(q,1.0/q.0.hypot(q.1));
                // A planar annular land joins the true circular roll to the polygonal CDT boundary.
                verts[i].2=bearing/2.0+radius;verts[i+n].2=-bearing/2.0-radius;
                let(mut top,mut bottom)=(Vec::new(),Vec::new());
                for j in 0..=SEGMENTS {
                    let phi=std::f64::consts::FRAC_PI_2*j as f64/SEGMENTS as f64;
                    let r=p.ring_id/2.0+radius*(1.0-phi.cos());let z=bearing/2.0+radius*phi.sin();
                    top.push(verts.len());verts.push((r*normal.0,r*normal.1,z));
                    bottom.push(verts.len());verts.push((r*normal.0,r*normal.1,-z));
                }
                top.push(i);bottom.push(i+n);(top,bottom)
            });
        }}
        for &(a,b) in &bore_edges {
            let(ta,ba)=&profiles[&a];let(tb,bb)=&profiles[&b];
            for j in 0..=SEGMENTS {
                tris.extend([[ta[j+1],ta[j],tb[j]],[ta[j+1],tb[j],tb[j+1]],
                             [ba[j+1],bb[j],ba[j]],[ba[j+1],bb[j+1],bb[j]]]);
            }
            tris.extend([[ta[0],ba[0],bb[0]],[ta[0],bb[0],tb[0]]]);
        }
        let mut rows:Vec<_>=profiles.iter().map(|(&i,(t,b))|
            [i,i+n,t[SEGMENTS],b[SEGMENTS],t[0],b[0]]).collect();
        rows.sort_by(|a,b| {let a=verts[a[0]];let b=verts[b[0]];a.1.atan2(a.0).total_cmp(&b.1.atan2(b.0))});
        if rows.len()<72{return None;}
        bore=Some(BorePatch{radius,bearing,inner_radius:p.ring_id/2.0,profiles:rows,faceted_faces});
    }
    Some(Solid{verts,tris,bore})
}

pub fn census(tris: &[[usize; 3]]) -> (usize, usize, usize) {
    let mut und: HashMap<(usize, usize), usize> = HashMap::new(); let mut dir: HashMap<(usize, usize), usize> = HashMap::new();
    for t in tris { for k in 0..3 { let (a, b) = (t[k], t[(k + 1) % 3]); *dir.entry((a, b)).or_default() += 1; *und.entry((a.min(b), a.max(b))).or_default() += 1; } }
    let once = und.values().filter(|&&v| v == 1).count(); let more = und.values().filter(|&&v| v > 2).count();
    (once, more, dir.values().copied().max().unwrap_or(0))
}

pub fn volume(s: &Solid) -> f64 {
    s.tris.iter().map(|t| { let (a, b, c) = (s.verts[t[0]], s.verts[t[1]], s.verts[t[2]]);
        (a.0 * (b.1 * c.2 - b.2 * c.1) - a.1 * (b.0 * c.2 - b.2 * c.0) + a.2 * (b.0 * c.1 - b.1 * c.0)) / 6.0 }).sum()
}

fn normal(a: V3, b: V3, c: V3) -> V3 {
    let u = (b.0 - a.0, b.1 - a.1, b.2 - a.2); let v = (c.0 - a.0, c.1 - a.1, c.2 - a.2);
    let n = (u.1 * v.2 - u.2 * v.1, u.2 * v.0 - u.0 * v.2, u.0 * v.1 - u.1 * v.0);
    let l = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt().max(1e-12); (n.0 / l, n.1 / l, n.2 / l)
}

pub fn stl(s: &Solid, name: &str) -> String {
    let mut o = format!("solid {name}\n");
    for t in &s.tris { let (a, b, c) = (s.verts[t[0]], s.verts[t[1]], s.verts[t[2]]); let nn = normal(a, b, c);
        o.push_str(&format!("facet normal {:.6} {:.6} {:.6}\n outer loop\n", nn.0, nn.1, nn.2));
        for v in [a, b, c] { o.push_str(&format!("  vertex {:.4} {:.4} {:.4}\n", v.0, v.1, v.2)); }
        o.push_str(" endloop\nendfacet\n"); }
    o.push_str(&format!("endsolid {name}\n")); o
}

pub fn step(s: &Solid, name: &str) -> String { crate::step_export::write(s,name) }



/// D1 item 4 — evolved grind-angle law as an ANALYTIC definition (machining-release input), not the
/// faceted STEP. Thickness inside the grind: th(q) = et + 2 d_e(q) tan(theta_e), capped by the stock/taper
/// base(x) = t - (t - t_tip) clamp((x - x_taper)/(x_max - x_taper)); faces z = +-th/2 (symmetric).
/// Each grind face is the ruled surface S(s, u) = C_e(s) + u (N_e(s), tan theta_e) over the planar edge curve
/// C_e at z = +-et/2, inward normal N_e, u in [0, w_e(s)], w_e = (base - et) / (2 tan theta_e) (grind-to-flat
/// runout = land line). This JSON gives C_e (0.25 mm samples), N_e, w_e and the land line per edge.
/// STATUS: law export REAL; analytic STEP B-rep (B_SPLINE_SURFACE_WITH_KNOTS deg (3,1) + trimmed ADVANCED_FACE)
/// NOT implemented — see docs/AESTHETIC_GAPS.md "D1 grind export". Faceted STEP is NOT a machining release.
pub fn grind_law_json(p: &Pheno, g: &crate::genome::Genome, label: &str) -> String {
    use crate::genome::*;
    let (t, t_tip, xt) = (p.t, g.v[T_TIP], g.v[TAPER_X0] + p.dx);
    let xmax = p.outline.iter().map(|q| q.0).fold(f64::MIN, f64::max);
    let base = |x: f64| t - (t - t_tip) * ((x - xt) / (xmax - xt)).clamp(0.0, 1.0);
    let et = g.v[EDGE_TH];
    let edge = |name: &str, c: &[P2], ang: f64| -> String {
        let c = densify(c, 0.25, false); let tn = ang.to_radians().tan(); let n = c.len();
        let mut rows = Vec::with_capacity(n);
        for i in 0..n {
            let tg = norm(sub(c[(i + 1).min(n - 1)], c[i.saturating_sub(1)])); let mut nn = (-tg.1, tg.0);
            let probe = add(c[i], mul(nn, 0.3)); if !point_in_poly(probe, &p.outline) { nn = (-nn.0, -nn.1); }
            let w = ((base(c[i].0) - et) / (2.0 * tn)).max(0.0); let land = add(c[i], mul(nn, w));
            rows.push(format!("[{:.4},{:.4},{:.5},{:.5},{:.4},{:.4},{:.4}]", c[i].0, c[i].1, nn.0, nn.1, w, land.0, land.1));
        }
        format!("{{\"edge\":\"{name}\",\"half_angle_deg\":{ang:.4},\"tan\":{tn:.6},\"edge_thickness_mm\":{et:.4},\"columns\":[\"x\",\"y\",\"nx_in\",\"ny_in\",\"grind_width_mm\",\"land_x\",\"land_y\"],\"samples\":[{}]}}", rows.join(","))
    };
    format!("{{\"label\":\"{label}\",\"kind\":\"analytic grind law (D1 item 4); NOT a B-rep\",\"units\":\"mm\",\"stock_t\":{t:.3},\"t_tip\":{t_tip:.3},\"taper_x0\":{xt:.3},\"x_max\":{xmax:.3},\"thickness_law\":\"th(q)=min(base(x), et+2*d_in(q)*tan_in, et+2*d_sp(q)*tan_sp); base(x)=t-(t-t_tip)*clamp((x-taper_x0)/(x_max-taper_x0),0,1); faces z=+-th/2; then pockets/grooves/perfs (see thickness raster)\",\"edges\":[{},{}]}}",
        edge("inner", &p.edge_in, g.v[EA_IN]), edge("spine", &p.edge_sp, g.v[EA_SP]))
}

/// The exact planar top facets used by the exported CAD, before submodel refinement.
/// Refinement interpolates these heights; it never re-samples a decimated thickness raster.
pub fn surface_json(s:&Solid)->serde_json::Value {
    let tris:Vec<_>=s.tris.iter().filter(|tr| {
        let(a,b,c)=(s.verts[tr[0]],s.verts[tr[1]],s.verts[tr[2]]);
        a.2>0.0 && b.2>0.0 && c.2>0.0 && (b.0-a.0)*(c.1-a.1)-(b.1-a.1)*(c.0-a.0)>1e-14
    }).collect();
    serde_json::json!({"schema":"jarambit.export_surface.v1","source":"exact exported CAD top facets",
        "verts":s.verts,"tris":tris})
}

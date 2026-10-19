//! Docs crate: run reports/tables and dependency-free SVG plots for docs/WHITEPAPER.md.
pub fn results_table(rows: &[(usize, f64, f64, f64)]) -> String {
    let mut s = String::from("| gen | best p05 strength-per-gram (N/g) | pop mean p05 | diversity |\n|---|---|---|---|\n");
    for (g, f, m, d) in rows { s.push_str(&format!("| {g} | {f:.3} | {m:.3} | {d:.3} |\n")); }
    s
}

/// Simple multi-series line/scatter SVG. `series` = (label, points, as_scatter).
pub fn svg_plot(title: &str, xl: &str, yl: &str, series: &[(&str, Vec<(f64, f64)>, bool)]) -> String {
    let (w, h, m) = (640.0, 420.0, 60.0);
    let pts: Vec<(f64, f64)> = series.iter().flat_map(|s| s.1.iter().cloned()).filter(|p| p.0.is_finite() && p.1.is_finite()).collect();
    if pts.is_empty() { return format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}'><text x='20' y='30'>{title}: no data</text></svg>"); }
    let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for p in &pts { x0 = x0.min(p.0); x1 = x1.max(p.0); y0 = y0.min(p.1); y1 = y1.max(p.1); }
    if x1 - x0 < 1e-9 { x1 = x0 + 1.0; } if y1 - y0 < 1e-9 { y1 = y0 + 1.0; }
    let sx = |x: f64| m + (x - x0) / (x1 - x0) * (w - 2.0 * m);
    let sy = |y: f64| h - m - (y - y0) / (y1 - y0) * (h - 2.0 * m);
    let colors = ["#1f77b4", "#d62728", "#2ca02c", "#ff7f0e", "#9467bd"];
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>\
<text x='{}' y='20' text-anchor='middle' font-size='14'>{title}</text><line x1='{m}' y1='{}' x2='{}' y2='{}' stroke='black'/><line x1='{m}' y1='{m}' x2='{m}' y2='{}' stroke='black'/>\
<text x='{}' y='{}' text-anchor='middle'>{xl}</text><text x='14' y='{}' transform='rotate(-90 14 {})' text-anchor='middle'>{yl}</text>\
<text x='{m}' y='{}'>{x0:.3}</text><text x='{}' y='{}' text-anchor='end'>{x1:.3}</text><text x='{}' y='{}' text-anchor='end'>{y0:.3}</text><text x='{}' y='{}' text-anchor='end'>{y1:.3}</text>",
        w / 2.0, h - m, w - m, h - m, h - m, w / 2.0, h - 15.0, h / 2.0, h / 2.0, h - m + 15.0, w - m, h - m + 15.0, m - 4.0, h - m, m - 4.0, m + 4.0);
    for (i, (label, p, scatter)) in series.iter().enumerate() {
        let c = colors[i % colors.len()];
        if *scatter { for q in p { if q.0.is_finite() && q.1.is_finite() { s.push_str(&format!("<circle cx='{:.1}' cy='{:.1}' r='2.5' fill='{c}' fill-opacity='0.6'/>", sx(q.0), sy(q.1))); } } }
        else { let d: Vec<String> = p.iter().filter(|q| q.1.is_finite()).map(|q| format!("{:.1},{:.1}", sx(q.0), sy(q.1))).collect(); s.push_str(&format!("<polyline points='{}' fill='none' stroke='{c}' stroke-width='2'/>", d.join(" "))); }
        s.push_str(&format!("<text x='{}' y='{}' fill='{c}'>{label}</text>", w - m - 150.0, m + 15.0 * i as f64));
    }
    s.push_str("</svg>"); s
}

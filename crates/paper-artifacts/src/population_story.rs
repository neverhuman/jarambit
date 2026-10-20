//! Reporting-only, source-bound within-run summaries. Never a scorer or birth decoder.
//! Proposed additive view: the existing full admitted all-record view remains required.
use serde_json::{json, Value};
use std::fmt::Write as _;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Preference { Higher, Lower }
impl Preference {
    fn better(self, a: f64, b: f64) -> bool {
        match self { Self::Higher => a > b, Self::Lower => a < b }
    }
    fn name(self) -> &'static str {
        match self { Self::Higher => "higher preferred", Self::Lower => "lower preferred" }
    }
}
#[derive(Clone, Debug)]
pub struct SourceBinding {
    pub path: String, pub sha256: String, pub scope_pointer: String,
    pub compatible_run: String, pub stage: String,
    pub membership_identity_order_sha256: String,
}
#[derive(Clone, Debug)]
pub struct LeaderTie {
    pub public_individual: u64, pub original_id: String, pub generation: u64,
}
#[derive(Clone, Debug)]
pub struct BoundLeader {
    pub original_value: f64, pub original_token: String,
    pub ledger_sha256: String, pub ties: Vec<LeaderTie>,
    /// Representative chosen by the already-bound leader ledger, NOT re-ranked by display code.
    pub representative_tie_index: usize,
}
#[derive(Clone, Debug)]
pub struct TraceSample {
    pub generation: u64, pub cumulative_best: Option<f64>, pub population_median: Option<f64>,
    /// Actual source JSON pointers. None means unavailable; Some with None value means recorded missing.
    /// Array indices MUST come from original source position, never from generation.
    pub best_source_pointer: Option<String>, pub median_source_pointer: Option<String>,
}
#[derive(Clone, Debug)]
pub struct BoundRunTrace {
    pub label: String, pub metric: String, pub preference: Preference,
    /// Extrema of the complete compatible original numeric scope, NOT just this summary.
    pub original_domain: [f64; 2], pub source: SourceBinding,
    pub leader: BoundLeader, pub samples: Vec<TraceSample>,
}
#[derive(Clone, Copy, Debug)]
pub enum View {
    /// The caller supplies the exact bound threshold from the existing axis contract.
    Asinh { linear_threshold: f64 },
    PositiveAffineLog10 { epsilon: f64 },
    /// A single fixed FINAL retained reference; never the running best itself.
    PositiveGapLog10 { epsilon: f64 },
}
#[derive(Clone, Copy, Debug)]
pub enum GenerationView { Linear, Log10GenerationPlusOne }
#[derive(Debug)]
pub struct StoryOutput { pub svg: String, pub receipt: Value }

fn valid_hash(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn required(s: &str, max: usize) -> bool { !s.trim().is_empty() && s.chars().count() <= max }
fn valid_pointer(s: &str) -> bool {
    if !s.starts_with('/') {return false;}
    let mut chars=s.chars();
    while let Some(c)=chars.next() {
        if c=='~' && !matches!(chars.next(),Some('0'|'1')) {return false;}
    }
    true
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&apos;")
}

/// Normalization is a display coordinate only. Comparisons always use original f.
#[derive(Clone, Copy, Debug)]
pub struct PositiveAxis {
    low: f64, high: f64, preference: Preference, gap: bool, epsilon: f64,
}
impl PositiveAxis {
    pub fn new(domain: [f64; 2], preference: Preference, gap: bool, epsilon: f64) -> Result<Self> {
        let [low, high] = domain;
        if !low.is_finite() || !high.is_finite() || low > high
            || !epsilon.is_finite() || epsilon <= 0. || epsilon >= 1. {
            return Err("population story: invalid positive display domain/floor".into());
        }
        Ok(Self { low, high, preference, gap, epsilon })
    }
    fn ratio(self, value: f64, from_high: bool) -> Result<f64> {
        if !value.is_finite() || value < self.low || value > self.high {
            return Err("population story: value outside original scope".into());
        }
        if self.low == self.high { return Ok(0.); }
        let range = self.high - self.low;
        let ratio = if range.is_finite() {
            if from_high { (self.high - value) / range } else { (value - self.low) / range }
        } else {
            // Finite endpoints can have overflowing differences. Factor BEFORE subtracting.
            let factor = self.low.abs().max(self.high.abs());
            let lo = self.low / factor;
            let hi = self.high / factor;
            if from_high { (hi - value / factor) / (hi - lo) }
            else { (value / factor - lo) / (hi - lo) }
        };
        if !ratio.is_finite() || !(0. ..=1.).contains(&ratio) {
            return Err("population story: corrupt normalized display difference".into());
        }
        Ok(ratio)
    }
    pub fn coordinate(self, original: f64) -> Result<f64> {
        let from_high = self.gap && self.preference == Preference::Higher;
        Ok(self.ratio(original, from_high)? + self.epsilon)
    }
    pub fn transform(self, original: f64) -> Result<f64> {
        Ok(self.coordinate(original)?.log10())
    }
    pub fn inverse_coordinate(self, coordinate: f64) -> Result<f64> {
        if !coordinate.is_finite() { return Err("population story: nonfinite inverse".into()); }
        if self.low == self.high {
            return if coordinate==self.epsilon {Ok(self.low)} else {Err("population story: constant inverse coordinate differs from floor".into())};
        }
        let from_high=self.gap && self.preference==Preference::Higher;
        if coordinate==self.epsilon {return Ok(if from_high {self.high} else {self.low});}
        if coordinate==1.+self.epsilon {return Ok(if from_high {self.low} else {self.high});}
        let ratio = coordinate - self.epsilon;
        if !(0. ..=1.).contains(&ratio) { return Err("population story: inverse outside domain".into()); }
        // Convex combination avoids range and signed-difference overflow.
        let t = if self.gap && self.preference == Preference::Higher { 1. - ratio } else { ratio };
        let value = if t == 0. { self.low } else if t == 1. { self.high }
            else { (1. - t) * self.low + t * self.high };
        if value.is_finite() { Ok(value) } else { Err("population story: inverse overflow".into()) }
    }
    /// Restore known exact display endpoints BEFORE powf/subtraction roundoff.
    /// This does not clamp source gaps, change raw comparisons, or create ties.
    pub fn inverse_log(self, transformed:f64)->Result<f64> {
        let low=self.epsilon.log10();let high=(1.+self.epsilon).log10();
        if !transformed.is_finite() || transformed<low || transformed>high {
            return Err("population story: log inverse outside declared display domain".into());
        }
        if transformed==low {return self.inverse_coordinate(self.epsilon);}
        if self.low==self.high {return Err("population story: constant log inverse not at sole coordinate".into());}
        if transformed==high {return self.inverse_coordinate(1.+self.epsilon);}
        self.inverse_coordinate(10_f64.powf(transformed))
    }
    pub fn metadata(self) -> Value {
        let range = self.high - self.low;
        let constant = self.low == self.high;
        let scale = if constant { json!({"units":1.,"constant_fallback":true}) }
        else if range.is_finite() { json!({"units":range,"constant_fallback":false}) }
        else {
            let factor = self.low.abs().max(self.high.abs());
            json!({"factor":factor,"scaled_range":self.high/factor-self.low/factor,"factorized_overflow_safe":true})
        };
        json!({"view":if self.gap {"positive_gap_log10"} else {"positive_affine_log10"},
            "min_original":self.low,"max_original":self.high,"scale":scale,
            "epsilon":self.epsilon,"epsilon_meaning":"display floor, not uncertainty or residual",
            "best_reference_original":if self.gap {Some(match self.preference { Preference::Higher=>self.high,Preference::Lower=>self.low })} else {None},
            "fixed_retrospective_reference":self.gap,"logarithm_base":10,
            "coordinate_formula":if !self.gap {"(f-m)/s+epsilon"} else if self.preference==Preference::Higher {"(b-f)/s+epsilon"} else {"(f-b)/s+epsilon"},
            "inverse_coordinate_formula":if !self.gap {"m+s*(coordinate-epsilon)"} else if self.preference==Preference::Higher {"b-s*(coordinate-epsilon)"} else {"b+s*(coordinate-epsilon)"},
            "constant_coordinate":"epsilon; inverse returns sole original value",
            "selection_on_display_transform":false,"physical_score_ratio_meaning":false})
    }
}

fn transform(run: &BoundRunTrace, view: View, value: f64) -> Result<f64> {
    match view {
        View::Asinh { linear_threshold: l } => {
            if !l.is_finite() || l <= 0. { return Err("population story: invalid asinh L".into()); }
            let ratio = value / l;
            Ok(if ratio.is_finite() {ratio.asinh()}
                else {value.signum() * (value.abs().ln() - l.ln() + 2_f64.ln())})
        }
        View::PositiveAffineLog10 { epsilon } => PositiveAxis::new(run.original_domain,run.preference,false,epsilon)?.transform(value),
        View::PositiveGapLog10 { epsilon } => PositiveAxis::new(run.original_domain,run.preference,true,epsilon)?.transform(value),
    }
}
fn generation_coordinate(g: u64, view: GenerationView) -> Result<f64> {
    if g >= (1u64 << 53) { return Err("population story: generation exceeds exact display integer range".into()); }
    Ok(match view { GenerationView::Linear => g as f64,
        GenerationView::Log10GenerationPlusOne => (g as f64 + 1.).log10() })
}

/// Only consumes already-bound summaries/leader ties; never decodes births or selects a new subject.
pub fn validate_run(run: &BoundRunTrace) -> Result<()> {
    if !required(&run.label,45) || !required(&run.metric,60)
        || !required(&run.source.path,1024) || !required(&run.source.scope_pointer,1024) || !valid_pointer(&run.source.scope_pointer)
        || !required(&run.source.compatible_run,128) || !required(&run.source.stage,128)
        || !valid_hash(&run.source.sha256) || !valid_hash(&run.source.membership_identity_order_sha256)
        || !valid_hash(&run.leader.ledger_sha256) || run.samples.is_empty() {
        return Err("population story: missing/invalid source, identity or compatible scope".into());
    }
    let [low,high] = run.original_domain;
    if !low.is_finite() || !high.is_finite() || low > high
        || !run.leader.original_value.is_finite()
        || run.leader.original_token.parse::<f64>().ok() != Some(run.leader.original_value)
        || !required(&run.leader.original_token,40) || run.leader.ties.is_empty()
        || run.leader.representative_tie_index >= run.leader.ties.len() {
        return Err("population story: invalid source domain/leader token/ties".into());
    }
    let reference = match run.preference {Preference::Higher=>high,Preference::Lower=>low};
    if run.leader.original_value != reference { return Err("population story: leader not bound original extremum".into()); }
    let mut previous: Option<(u64,f64)> = None;
    let mut last_generation = None;
    let mut source_pointers=std::collections::BTreeSet::new();
    for sample in &run.samples {
        if last_generation.is_some_and(|g| sample.generation <= g) {
            return Err("population story: duplicate/out-of-order generation".into());
        }
        generation_coordinate(sample.generation,GenerationView::Linear)?;
        last_generation = Some(sample.generation);
        for (value,pointer) in [(sample.cumulative_best,&sample.best_source_pointer),(sample.population_median,&sample.median_source_pointer)] {
            if value.is_some() && pointer.is_none() {return Err("population story: numeric summary missing original pointer".into());}
            if let Some(pointer)=pointer {
                let prefix=format!("{}/",run.source.scope_pointer);
                if !valid_pointer(pointer) || !pointer.starts_with(&prefix) || !source_pointers.insert(pointer.clone()) {
                    return Err("population story: invalid/duplicate/out-of-scope source pointer".into());
                }
            }
        }
        for value in [sample.cumulative_best,sample.population_median].into_iter().flatten() {
            if !value.is_finite() || value < low || value > high {
                return Err("population story: nonfinite/out-of-scope summary; missing must be None".into());
            }
        }
        if let Some(best) = sample.cumulative_best {
            if let Some((_,old)) = previous {
                if run.preference.better(old,best) {return Err("population story: cumulative best regressed".into());}
            }
            if let Some(median) = sample.population_median {
                if run.preference.better(median,best) {return Err("population story: median better than retained best".into());}
            }
            previous = Some((sample.generation,best));
        }
    }
    if previous.map(|(_,v)|v) != Some(reference) {return Err("population story: final observed best differs from fixed retained reference".into());}
    let mut ids = std::collections::BTreeSet::new();
    let mut original_ids = std::collections::BTreeSet::new();
    for tie in &run.leader.ties {
        if tie.public_individual==0 || !required(&tie.original_id,80) || !ids.insert(tie.public_individual) || !original_ids.insert(tie.original_id.clone())
            || !run.samples.iter().any(|s|s.generation==tie.generation && s.cumulative_best==Some(reference)) {
            return Err("population story: invalid/duplicate/unjoined retained leader tie".into());
        }
    }
    Ok(())
}

pub fn event_receipt(run: &BoundRunTrace) -> Result<Value> {
    validate_run(run)?;
    let reference=run.leader.original_value;
    let mut prior=None;
    let mut events=Vec::new();
    let mut suffix_start=None;
    let mut suffix_end=None;
    for s in &run.samples {
        let adjacent = prior.is_some_and(|(g,_)| g+1==s.generation);
        let improved = match (prior,s.cumulative_best) {
            (Some((_,old)),Some(value)) => Some(run.preference.better(value,old)), _=>None,
        };
        if s.cumulative_best==Some(reference) {
            if !adjacent || suffix_end!=Some(s.generation-1) {suffix_start=Some(s.generation);}
            suffix_end=Some(s.generation);
        } else {suffix_start=None;suffix_end=None;}
        events.push(json!({"generation":s.generation,"best_source_pointer":s.best_source_pointer,"median_source_pointer":s.median_source_pointer,
            "original_cumulative_best":s.cumulative_best,"population_median":s.population_median,
            "strict_improvement_since_preceding_observed":improved,
            "adjacent_observed_generation":adjacent,
            "improvement_generation_resolved":if improved==Some(true) {Some(adjacent)} else {None},
            "missing_best":s.cumulative_best.is_none()}));
        if let Some(v)=s.cumulative_best {prior=Some((s.generation,v));} else {prior=None;}
    }
    let final_sample=run.samples.last().unwrap();
    let terminal_transitions = match (suffix_start,suffix_end) {
        (Some(a),Some(b)) if b==final_sample.generation => Some(b-a),_=>None,
    };
    Ok(json!({"scope":run.source.compatible_run,"stage":run.source.stage,
        "metric":run.metric,"preference":run.preference.name(),
        "first_observed_retained_best_generation":run.samples.iter().find(|s|s.cumulative_best==Some(reference)).map(|s|s.generation),
        "terminal_contiguous_no_strict_improvement_transitions":terminal_transitions,
        "terminal_interval_start":suffix_start,"terminal_interval_end":suffix_end,
        "interpretation":"No better stored observation at source serialization precision in the observed contiguous interval; not optimality or physical convergence",
        "events":events}))
}

fn line(svg:&mut String,x:f64,y:f64,text:&str,size:u32) {
    writeln!(svg,"<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\">{}</text>",escape(text)).unwrap();
}

/// Conservative wrapping keeps full-width type at 22 SVG units; never shrinks labels.
fn wrapped(svg:&mut String,y:&mut f64,text:&str) {
    let mut row=String::new();
    for word in text.split_whitespace() {
        for chunk in word.chars().collect::<Vec<_>>().chunks(48) {
            let chunk:String=chunk.iter().collect();
            if !row.is_empty() && row.chars().count()+1+chunk.chars().count()>48 {
                line(svg,24.,*y,&row,22);*y+=30.;row.clear();
            }
            if !row.is_empty() {row.push(' ');}
            row.push_str(&chunk);
        }
    }
    if !row.is_empty() {line(svg,24.,*y,&row,22);*y+=30.;}
}
fn tick_label(value:f64)->String {
    if value.abs()>=1e5 || (value!=0. && value.abs()<1e-3) {format!("{value:.2e}")}
    else {format!("{value:.4}")}
}

/// One full-width run panel. Caller registers every admitted run; no cross-run line exists.
pub fn render_run_panel(run:&BoundRunTrace,view:View,x_view:GenerationView)->Result<StoryOutput> {
    validate_run(run)?;
    let mut ys=Vec::new();
    for s in &run.samples {
        for value in [s.cumulative_best,s.population_median].into_iter().flatten() {ys.push(transform(run,view,value)?);}
    }
    if ys.iter().any(|y|!y.is_finite()) {return Err("population story: nonfinite display coordinate".into());}
    let ymin=ys.iter().copied().fold(f64::INFINITY,f64::min);
    let ymax=ys.iter().copied().fold(f64::NEG_INFINITY,f64::max);
    let gx0=generation_coordinate(run.samples[0].generation,x_view)?;
    let gx1=generation_coordinate(run.samples.last().unwrap().generation,x_view)?;
    let px=|g:u64|->Result<f64>{Ok(140.+if gx0==gx1 {495.} else {(generation_coordinate(g,x_view)?-gx0)/(gx1-gx0)*990.})};
    let mut body=String::new();
    line(&mut body,24.,32.,"How did stored scores change within this run?",25);
    let mut header_y=64.;
    wrapped(&mut body,&mut header_y,&format!("{} — {} ({})",run.label,run.metric,run.preference.name()));
    let tie=&run.leader.ties[run.leader.representative_tie_index];
    wrapped(&mut body,&mut header_y,&format!("Best retained: {} · Individual {} at G{} · {} exact tie(s)",run.leader.original_token,tie.public_individual,tie.generation,run.leader.ties.len()));
    let axis_label=match view {View::Asinh{linear_threshold:l}=>format!("asinh(f/L), L={l}; stored-score ticks"),
        View::PositiveAffineLog10{epsilon}=>format!("Positive affine log10; ε={epsilon}; ticks are original f"),
        View::PositiveGapLog10{epsilon}=>format!("Gap to fixed retained best (log10); ε={epsilon}; smaller is closer")};
    wrapped(&mut body,&mut header_y,&axis_label);
    wrapped(&mut body,&mut header_y,"Retained best (solid blue) · population median (dashed gray)");
    let top=header_y+12.;let bottom=top+190.;
    let py=|v:f64|->Result<f64>{Ok(bottom-if ymin==ymax {95.} else {(transform(run,view,v)?-ymin)/(ymax-ymin)*190.})};
    writeln!(body,"<path d=\"M140 {top}V{bottom}H1130\" fill=\"none\" stroke=\"#152d40\" stroke-width=\"1.5\"/>").unwrap();
    let mut tick_rows=Vec::new();
    let fractions:Vec<f64>=if ymin==ymax {vec![0.5]} else {(0..=4).map(|i|i as f64/4.).collect()};
    for fraction in fractions {
        let transformed=if fraction==0. {ymin} else if fraction==1. {ymax} else {ymin+(ymax-ymin)*fraction};
        let original = match view {
            View::Asinh{linear_threshold:l} => {
                let f=if transformed.abs()<20. {transformed.sinh()*l}
                    else {transformed.signum()*(transformed.abs()+l.ln()-2_f64.ln()).exp()};
                if !f.is_finite() {return Err("population story: tick inverse overflow".into());} f
            },
            View::PositiveAffineLog10{epsilon} => PositiveAxis::new(run.original_domain,run.preference,false,epsilon)?.inverse_log(transformed)?,
            View::PositiveGapLog10{..} => transformed,
        };
        let label=if let View::PositiveGapLog10{epsilon}=view {
            if transformed==epsilon.log10() {"0 (best)".into()}
            else {
                let gap=10_f64.powf(transformed)-epsilon;
                if gap<0. || !gap.is_finite() {return Err("population story: invalid gap tick".into());}
                format!("{gap:.1e}")
            }
        } else {tick_label(original)};
        let y=bottom-fraction*190.;
        writeln!(body,"<text x=\"128\" y=\"{}\" text-anchor=\"end\" font-size=\"22\">{}</text><line x1=\"140\" y1=\"{y}\" x2=\"1130\" y2=\"{y}\" stroke=\"#d9e2e5\"/>",y+7.,escape(&label)).unwrap();
        tick_rows.push(json!({"display_log_or_asinh_coordinate":transformed,"label":label}));
    }
    for (median,stroke,dash) in [(true,"#626e79","6 4"),(false,"#007fa3","")] {
        let mut path=String::new();let mut prior=None;
        for s in &run.samples {
            let value=if median {s.population_median} else {s.cumulative_best};
            if let Some(v)=value {
                let x=px(s.generation)?;let y=py(v)?;
                if prior.is_some_and(|g|g+1==s.generation) {
                    if median {write!(path," L{x:.3},{y:.3}").unwrap();}
                    else {write!(path," H{x:.3} V{y:.3}").unwrap();}
                } else {write!(path," M{x:.3},{y:.3}").unwrap();}
                writeln!(body,"<circle cx=\"{x:.3}\" cy=\"{y:.3}\" r=\"1.5\" fill=\"{stroke}\"/>").unwrap();
                prior=Some(s.generation);
            } else {prior=None;}
        }
        writeln!(body,"<path d=\"{path}\" fill=\"none\" stroke=\"{stroke}\" stroke-width=\"2.5\" stroke-dasharray=\"{dash}\"/>").unwrap();
    }
    let first_g=run.samples[0].generation;
    let last_g=run.samples.last().unwrap().generation;
    let mut generations=vec![first_g];if last_g!=first_g {generations.push(last_g);}
    for g in generations {
        let x=px(g)?;
        writeln!(body,"<text x=\"{x}\" y=\"{}\" text-anchor=\"middle\" font-size=\"22\">G{g}</text>",bottom+34.).unwrap();
    }
    let event=event_receipt(run)?;
    let mut footer_y=bottom+70.;
    let gap_label=if matches!(view,View::PositiveGapLog10{..}) {"Ticks: normalized gap; display floor only permits log. Best = zero original gap."}
        else {"Stored-score display only; source observations and selection are unchanged."};
    wrapped(&mut body,&mut footer_y,gap_label);
    if let (Some(start),Some(end),Some(transitions))=(event["terminal_interval_start"].as_u64(),event["terminal_interval_end"].as_u64(),event["terminal_contiguous_no_strict_improvement_transitions"].as_u64()) {
        let direction=match run.preference {Preference::Higher=>"higher",Preference::Lower=>"lower"};
        if transitions>0 {
            wrapped(&mut body,&mut footer_y,&format!("G{start}–G{end}: no strictly {direction} stored best over {transitions} transitions at saved precision."));
        } else {wrapped(&mut body,&mut footer_y,"Only one terminal retained-best sample; no multi-generation plateau shown.");}
    } else {wrapped(&mut body,&mut footer_y,"Terminal plateau unavailable; source coverage gaps remain.");}
    wrapped(&mut body,&mut footer_y,"Missing generations break lines and plateau intervals; no later qualification inferred.");
    let x_label=match x_view {GenerationView::Linear=>"Generation (linear)",GenerationView::Log10GenerationPlusOne=>"Generation (log10(g+1)); ticks retain original generation"};
    wrapped(&mut body,&mut footer_y,x_label);
    let height=footer_y+16.;
    let svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1200 {height}\" width=\"1200\" height=\"{height}\"><rect width=\"1200\" height=\"{height}\" fill=\"white\"/><g font-family=\"Arial,sans-serif\" fill=\"#152d40\">{body}</g></svg>\n");
    let axis=match view {View::Asinh{linear_threshold:l}=>json!({"transform":"asinh(f/L)","inverse":"L*sinh(t)","L":l}),
        View::PositiveAffineLog10{epsilon}=>PositiveAxis::new(run.original_domain,run.preference,false,epsilon)?.metadata(),
        View::PositiveGapLog10{epsilon}=>PositiveAxis::new(run.original_domain,run.preference,true,epsilon)?.metadata()};
    let ties:Vec<_>=run.leader.ties.iter().map(|t|json!({"public_individual":t.public_individual,"original_id":t.original_id,"generation":t.generation})).collect();
    Ok(StoryOutput{svg,receipt:json!({"schema":"jarambit.bound_run_story.v1",
        "source":{"path":run.source.path,"sha256":run.source.sha256,"scope_pointer":run.source.scope_pointer,"compatible_run":run.source.compatible_run,"stage":run.source.stage,"membership_identity_order_sha256":run.source.membership_identity_order_sha256},
        "source_original_domain":run.original_domain,"view_axis":axis,"tick_rows":tick_rows,
        "display_extent_from":"all present cumulative-best and population-median summary samples; NOT an all-record plot",
        "transformed_summary_extent":[ymin,ymax],"generation_view":x_label,
        "original_leader":{"value":run.leader.original_value,"source_token":run.leader.original_token,"ledger_sha256":run.leader.ledger_sha256,"exact_ties":ties,"representative_tie_index_from_ledger":run.leader.representative_tie_index},
        "progress_events":event,"summary_slots":run.samples.len(),
        "missing_best_slots":run.samples.iter().filter(|s|s.cumulative_best.is_none()).count(),
        "missing_median_slots":run.samples.iter().filter(|s|s.population_median.is_none()).count(),
        "all_record_companion_required":true,"all_record_points_drawn_here":0,"all_record_membership_changed":false,
        "global_comparable_champion":null,"selection_on_display_transform":false,
        "minimum_font_svg_units":22,"viewbox_width":1200,"panel_height":height,
        "target_placement":"full text width; final PDF type must be >=8.5pt, target9pt",
        "physical_convergence_or_qualification_claim":false})})
}

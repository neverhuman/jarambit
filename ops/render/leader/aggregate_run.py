#!/usr/bin/env python3
"""Vendored from ~/jarambit_runs/evo_continuous/tools/aggregate.py; writes ONLY to OUT_DIR (argv[3]).
Aggregate a host's evo_full_20261018 run into plots + MANIFEST.json (read-only over run outputs)."""
import csv, glob, json, os, sys, time, hashlib
root, host = sys.argv[1], sys.argv[2]
agg = sys.argv[3] if len(sys.argv) > 3 else os.path.join(root, "_aggregate"); os.makedirs(agg, exist_ok=True)
def jl(p):
    try: return json.load(open(p))
    except Exception: return None
islands = sorted(glob.glob(f"{root}/islands/i*"))
man = {"schema": "jarambit.evo_full_20261018.manifest.v1", "host": host, "updated_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
       "run_config": jl(f"{root}/run_config.json"), "islands": [], "crescent": []}
screen_rows, s2_rows, curves, div = [], [], {}, {}
for d in islands:
    I = os.path.basename(d); info = {"island": I, **(jl(f"{d}/island.json") or {})}
    rounds = sorted(glob.glob(f"{d}/round_*_screen.json")); info["rounds_screened"] = len(rounds)
    for r in rounds:
        rn = int(os.path.basename(r).split("_")[1])
        for row in jl(r) or []:
            screen_rows.append({"island": I, "round": rn, **{k: row.get(k) for k in ("label", "family", "mass_g", "seq_ratio", "fat_sf", "ring_bearing_w_mm", "screen_deficit", "ultralight_weighted_fitness")}})
    for b in sorted(glob.glob(f"{d}/round_*_births.json")):
        bj = jl(b) or {}
        try:
            import numpy as np
            def fl(x):
                try: return float(x)
                except Exception: return float(x.strip().lower() == "true")
            M = np.array([[fl(x) for x in g["row"].split(",")] for g in bj.get("genomes", [])])
            if len(M) > 1:
                rng = M.max(0) - M.min(0); rng[rng == 0] = 1
                div.setdefault(I, []).append((bj.get("round", 0), float(((M - M.mean(0)) / rng).std(0).mean())))
        except Exception: pass
    sc = sorted(glob.glob(f"{d}/by-content/*/attempt-*/*_master_scorecard.json")); info["stage2_scorecards"] = len(sc)
    for f in sc:
        j = jl(f) or {}; em = j.get("engineering_metrics", {}); uw = j.get("ultralight_weighted_fitness")
        uwv = uw.get("global_weighted_fitness") if isinstance(uw, dict) else uw
        s2_rows.append({"island": I, "label": j.get("subject") or os.path.basename(f).replace("_master_scorecard.json", ""),
            "mass_3d_g": em.get("mass_3d_g"), "sf_hunter_strict": em.get("sf_hunter_strict"), "solid_tip_sf": em.get("solid_tip_sf_nodal_max"),
            "seq_ratio_cert": em.get("seq_ratio_cert"), "fat_sf_l7": em.get("fat_sf_l7"), "twist_tip_sf": em.get("twist_tip_sf"),
            "buckling_a4": em.get("buckling_eigenvalue_a4"), "rs_v2_all_hard": em.get("rs_v2_all_hard"),
            "all_16": (j.get("weighted_cad_score") or {}).get("all_engineering_gates_pass"), "W_CAD": (j.get("weighted_cad_score") or {}).get("W_CAD"),
            "G1": (j.get("global_fitness_G1") or {}).get("G"), "W_ULTRALIGHT_v1": uwv, "scorecard": os.path.relpath(f, root)})
    fc = jl(f"{d}/fitness_curve.json")
    if fc:
        gens = fc.get("generations") or []
        if gens: curves[I] = ([g.get("hall_of_fame_best_so_far") for g in gens], [g.get("round_best_fitness") for g in gens])
        info["best_so_far"] = {k: (fc.get("best_so_far") or {}).get(k) for k in ("label", "mass_3d_g", "ultralight_weighted_fitness")}
    info["finished"] = os.path.exists(f"{d}/EXIT"); info["exit"] = open(f"{d}/EXIT").read().strip() if info["finished"] else None
    info["leaderboard"] = os.path.exists(f"{d}/leaderboard.json")
    man["islands"].append(info)
cres = []
for d in sorted(glob.glob(f"{root}/crescent/c*")):
    C = os.path.basename(d); h = f"{d}/history_live.csv"; n = sum(1 for _ in open(h)) if os.path.exists(h) else 0
    man["crescent"].append({"proc": C, "history_lines": n, "finished": os.path.exists(f"{d}/EXIT"), "files": sorted(os.listdir(d))[:40]})
    if os.path.exists(h): cres.append((C, h))
def wcsv(name, rows):
    if not rows: return
    keys = list(rows[0].keys())
    with open(f"{agg}/{name}", "w", newline="") as fh:
        w = csv.DictWriter(fh, keys); w.writeheader(); w.writerows(rows)
wcsv("stage1_screen_all.csv", screen_rows); wcsv("stage2_assessed_all.csv", s2_rows)
plots = []
try:
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
    def num(v):
        try: return float(v)
        except Exception: return None
    if curves:
        fig, ax = plt.subplots(figsize=(9, 5), dpi=150)
        for I, (hof, rb) in curves.items():
            l = ax.plot(range(len(hof)), [num(v) for v in hof], marker="o", lw=1.6, alpha=.85, label=I)[0]
            ax.plot(range(len(rb)), [num(v) for v in rb], ls=":", lw=1, color=l.get_color(), alpha=.6)
        ax.set_xlabel("round / generation"); ax.set_ylabel("Hall-of-Fame W_ULTRALIGHT_v1"); ax.set_title(f"{host}: convergence per island (monotone HoF)")
        ax.legend(fontsize=6, ncol=4); fig.tight_layout(); fig.savefig(f"{agg}/convergence_islands.png"); plt.close(fig); plots.append("convergence_islands.png")
    pts = [(num(r["mass_g"]), num(r["ultralight_weighted_fitness"]), r["round"]) for r in screen_rows if num(r["mass_g"]) and num(r["ultralight_weighted_fitness"]) is not None]
    if not pts: pts = [(num(r["mass_g"]), -num(r["screen_deficit"]), r["round"]) for r in screen_rows if num(r["mass_g"]) and num(r["screen_deficit"]) is not None]
    if pts:
        fig, ax = plt.subplots(figsize=(9, 5.5), dpi=150); x, y, c = zip(*pts)
        s = ax.scatter(x, y, c=c, cmap="viridis", s=8, alpha=.6); plt.colorbar(s, label="round")
        P = sorted(pts); front, best = [], -1e9
        for m, f, _ in P:
            if f > best: front.append((m, f)); best = f
        if front: ax.plot(*zip(*front), "r-", lw=2, label="Pareto front (lighter & fitter)")
        ax.set_xlabel("screen mass (g)"); ax.set_ylabel("Stage-1 W_ULTRALIGHT_v1"); ax.set_title(f"{host}: Stage-1 mass vs weighted fitness ({len(pts)} candidates)"); ax.legend()
        fig.tight_layout(); fig.savefig(f"{agg}/pareto_stage1_mass_vs_fitness.png"); plt.close(fig); plots.append("pareto_stage1_mass_vs_fitness.png")
    if s2_rows:
        objs = [("sf_hunter_strict", "strict hunter SF"), ("solid_tip_sf", "solid tip SF"), ("seq_ratio_cert", "failure-sequence ratio"), ("fat_sf_l7", "Goodman fatigue SF"), ("twist_tip_sf", "tip twist SF"), ("W_ULTRALIGHT_v1", "W_ULTRALIGHT_v1")]
        fig, axs = plt.subplots(2, 3, figsize=(14, 8), dpi=140)
        for ax, (k, lab) in zip(axs.flat, objs):
            q = [(num(r["mass_3d_g"]), num(r[k]), bool(r["all_16"])) for r in s2_rows if num(r["mass_3d_g"]) and num(r[k]) is not None]
            if q:
                x, y, g = zip(*q); ax.scatter(x, y, c=["tab:green" if v else "tab:red" for v in g], s=14, alpha=.7)
            ax.set_xlabel("3D mass (g)"); ax.set_ylabel(lab); ax.grid(alpha=.3)
        fig.suptitle(f"{host}: Stage-2 independent assessments — objective trade-offs vs mass ({len(s2_rows)} designs; green = all 16 gates)")
        fig.tight_layout(); fig.savefig(f"{agg}/stage2_objective_tradeoffs.png"); plt.close(fig); plots.append("stage2_objective_tradeoffs.png")
    if div:
        fig, ax = plt.subplots(figsize=(9, 5), dpi=150)
        for I, v in div.items(): v = sorted(v); ax.plot([a for a, _ in v], [b for _, b in v], marker=".", label=I)
        ax.set_xlabel("round"); ax.set_ylabel("mean normalised genome std (births)"); ax.set_title(f"{host}: population diversity"); ax.legend(fontsize=6, ncol=4)
        fig.tight_layout(); fig.savefig(f"{agg}/diversity.png"); plt.close(fig); plots.append("diversity.png")
    for C, h in cres:
        rows = list(csv.reader(open(h)))
        if len(rows) > 2:
            hdr = rows[0]
            try:
                fig, ax = plt.subplots(figsize=(9, 5), dpi=150); gi = 0
                for j, name in enumerate(hdr[1:], 1):
                    ys = [num(r[j]) for r in rows[1:] if len(r) > j]
                    if ys and all(v is not None for v in ys): ax.plot(ys, label=name)
                ax.set_xlabel("generation"); ax.set_title(f"{host}/{C}: crescent-evolve history"); ax.legend(fontsize=6)
                fig.tight_layout(); fig.savefig(f"{agg}/crescent_{C}_history.png"); plt.close(fig); plots.append(f"crescent_{C}_history.png")
            except Exception: plt.close("all")
        qa = os.path.join(os.path.dirname(h), "qd_archive.csv")
        if os.path.exists(qa):
            rr = list(csv.DictReader(open(qa)))
            if rr:
                fig, ax = plt.subplots(figsize=(8, 6), dpi=150)
                s = ax.scatter([int(r["cell_mass_bin"]) for r in rr], [int(r["cell_removal"]) for r in rr], c=[num(r["fit"]) or 0 for r in rr], cmap="magma", s=60, marker="s")
                plt.colorbar(s, label="fitness"); ax.set_xlabel("mass bin"); ax.set_ylabel("removal bin"); ax.set_title(f"{host}/{C}: MAP-Elites archive")
                fig.tight_layout(); fig.savefig(f"{agg}/crescent_{C}_map_elites.png"); plt.close(fig); plots.append(f"crescent_{C}_map_elites.png")
except Exception as e:
    man["plot_error"] = repr(e)
man["plots"] = plots; man["stage1_candidates"] = len(screen_rows); man["stage2_assessed"] = len(s2_rows)
n = 0; size = 0
for dp, _, fs in os.walk(root):
    for f in fs: n += 1; size += os.path.getsize(os.path.join(dp, f))
man["files"] = n; man["bytes"] = size
json.dump(man, open(f"{agg}/MANIFEST.json", "w"), indent=1)
print(man["updated_utc"], "islands", len(islands), "stage1", len(screen_rows), "stage2", len(s2_rows), "plots", len(plots))

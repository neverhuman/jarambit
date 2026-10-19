import type { WebStudioBundle } from "../types";

export const DEFAULT_STUDIO_BUNDLE: WebStudioBundle = {
  "battery_records": {
    "c10a": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "geom.mass",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "geom.pin",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "plate.loads",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "plate.seq",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "plate.l7",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "side.a1",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "side.a2",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "side.a3",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "side.a4",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "R_CUT": 30.0,
            "SF_nodal_avg": 0.2216,
            "SF_nodal_max": 0.2101,
            "dolfinx_cross_check": {
              "SF_nodal_avg": 0.2216,
              "SF_nodal_max": 0.2101,
              "container": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "ksp_its": 172,
              "rel_err_nodal_avg": 0.0,
              "rel_err_nodal_max": 0.0,
              "within_2pct": true
            },
            "entered_as_solver_run": true,
            "ksp_its": 125,
            "n_dof": 372357,
            "n_tet": 80496
          },
          "evidence_path": "evidence/cad_battery/side/c10a_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.2101
          },
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "B-rep is not an RS-karambit-v2 genome",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        }
      ],
      "global_fitness": {
        "G": null,
        "inputs": {
          "solid_tip_sf_nodal_max": 0.2101
        },
        "metric": "G1",
        "missing": [
          "spg: missing",
          "aes_score: missing",
          "mass_g: missing",
          "seq_ratio: missing"
        ],
        "provisional": null,
        "status": "INCOMPLETE",
        "title": "G INCOMPLETE (4 missing) | mass n/a | jarambit v1.0.0"
      },
      "pass_count": 5,
      "role": "native_cad",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "c10a"
    },
    "c10c": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "geom.mass",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "geom.pin",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "plate.loads",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "plate.seq",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "plate.l7",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "side.a1",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "side.a2",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "side.a3",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C10c/cad/switchback_revC10c_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "side.a4",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "solid.tip",
          "kind": "solver_run",
          "reason": "no solid tip bound to this B-rep",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "4211f35b1c04d795029c85bcd3fa6a81ed3456f774a935d419e326bc06d4eae4",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "B-rep is not an RS-karambit-v2 genome",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        }
      ],
      "global_fitness": {
        "G": null,
        "inputs": {},
        "metric": "G1",
        "missing": [
          "tip: solid_tip_sf_nodal_max: missing; plate_tipsub_screen_sf: missing; twist_tipzone_sf_strict: missing",
          "spg: missing",
          "aes_score: missing",
          "mass_g: missing",
          "seq_ratio: missing"
        ],
        "provisional": null,
        "status": "INCOMPLETE",
        "title": "G INCOMPLETE (5 missing) | mass n/a | jarambit v1.0.0"
      },
      "pass_count": 4,
      "role": "archived_not_on_main",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 0,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "c10c"
    },
    "c92": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "geom.mass",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "geom.pin",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "plate.loads",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "plate.seq",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "plate.l7",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "side.a1",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "side.a2",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "side.a3",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.2/cad/switchback_revC9_2_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "side.a4",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "entered_as_solver_run": false,
            "files": [
              {
                "names_full_step_sha256": false,
                "objects_with_SF_only": 5,
                "objects_with_n_tet_and_SF": 12,
                "path": "designs/switchback/C9.2/data/w75_1_results.json",
                "sha16": "2ad2a8ecaf4fca67"
              }
            ],
            "missing_fields": [
              "n_dof",
              "ksp_its"
            ]
          },
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "solid.tip",
          "kind": "solver_run",
          "reason": "no solid tip bound to this B-rep",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "2ad2a8ecaf4fca67ea4debe5b5da48d2d6216f4a8960296985a9fa38f5b1c307",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "B-rep is not an RS-karambit-v2 genome",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        }
      ],
      "global_fitness": {
        "G": null,
        "inputs": {},
        "metric": "G1",
        "missing": [
          "tip: solid_tip_sf_nodal_max: missing; plate_tipsub_screen_sf: missing; twist_tipzone_sf_strict: missing",
          "spg: missing",
          "aes_score: missing",
          "mass_g: missing",
          "seq_ratio: missing"
        ],
        "provisional": null,
        "status": "INCOMPLETE",
        "title": "G INCOMPLETE (5 missing) | mass n/a | jarambit v1.0.0"
      },
      "pass_count": 4,
      "role": "native_cad",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 0,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "c92"
    },
    "c93": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "geom.mass",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "geom.pin",
          "kind": "gate",
          "reason": "no mesh bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "plate.loads",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "plate.seq",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "plate.l7",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "side.a1",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "side.a2",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "side.a3",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "designs/switchback/C9.3/cad/switchback_revC9_3_ground.step",
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "side.a4",
          "kind": "derived",
          "reason": "no mesh bound",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "entered_as_solver_run": false,
            "files": [
              {
                "base_line_names_c92_sha16": "2ad2a8ecaf4fca67",
                "names_c93_step_sha256": false,
                "objects_with_n_tet_and_SF": 48,
                "path": "designs/switchback/C9.3/data/w76_results.json"
              },
              {
                "names_full_step_sha256": false,
                "objects_with_n_tet": 8,
                "objects_with_numeric_SF": 0,
                "path": "designs/switchback/C9.3/data/w77_results.json",
                "sha16": "3a818183bcb5b9a9"
              }
            ],
            "missing_fields": [
              "n_dof",
              "ksp_its"
            ]
          },
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "solid.tip",
          "kind": "solver_run",
          "reason": "no solid tip bound to this B-rep",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": null,
          "fresh_this_lane": false,
          "geometry_sha256": "3a818183bcb5b9a9093da947e00414ed29aa9bfebbb614046b8fb0b16dcbe919",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "B-rep is not an RS-karambit-v2 genome",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        }
      ],
      "global_fitness": {
        "G": null,
        "inputs": {},
        "metric": "G1",
        "missing": [
          "tip: solid_tip_sf_nodal_max: missing; plate_tipsub_screen_sf: missing; twist_tipzone_sf_strict: missing",
          "spg: missing",
          "aes_score: missing",
          "mass_g: missing",
          "seq_ratio: missing"
        ],
        "provisional": null,
        "status": "INCOMPLETE",
        "title": "G INCOMPLETE (5 missing) | mass n/a | jarambit v1.0.0"
      },
      "pass_count": 4,
      "role": "native_cad",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 0,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "c93"
    },
    "g3": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/g3_tip+00_t60_tt3.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/g3_tip+00_t60_tt3.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 84.80209229536577
          },
          "evidence_path": "evidence/rs_v2/g3_tip+00_t60_tt3.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 84.80209229536577
          },
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/g3_tip+00_t60_tt3.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.13210292745211555
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 351.6634692341955
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 351.6634692341955
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.660229597729158
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 17463.975804201444
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4189305474633907
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5648925162300026
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/g3_tip+00_t60_tt3.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 4.14687
          },
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.9590825616164744,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.9765216896409544
          },
          "evidence_path": "evidence/rs_v2/g3_tip+00_t60_tt3.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.9590825616164744
          },
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.9823103780720044,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/g3_tip+00_t60_tt3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": true,
            "discretization_under_3pct": true,
            "fit_status": "admissible_estimate",
            "gci_fine": 0.013794225201111676,
            "p": 0.9925256080950314,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/cad_battery/plate_gci/g3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "plate.gci",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3e419ac1cbd2576b9b2f92296862d937e763abe27cc2a065c4937ff6ec4a85f9"
          },
          "evidence_path": "evidence/cad_battery/side/g3_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3e419ac1cbd2576b9b2f92296862d937e763abe27cc2a065c4937ff6ec4a85f9"
          },
          "evidence_path": "evidence/cad_battery/side/g3_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3e419ac1cbd2576b9b2f92296862d937e763abe27cc2a065c4937ff6ec4a85f9"
          },
          "evidence_path": "evidence/cad_battery/side/g3_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2723.084805296607,
            "buckling_eigenvalue": 10.473403097294645,
            "certified_seq_ratio": 0.9590825616164744,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "e7307d1798bbe5f1284b3e3098525767217778793f9b9d64472670610cb771e1",
            "supported_seq_ratio": 2.7900975009088227
          },
          "evidence_path": "evidence/cad_battery/side/g3_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 8,
            "NSUB": 2,
            "SF_nodal_max": 1.6003,
            "SF_nodal_max_published_fine": 1.6002,
            "copied_from_another_genome": false,
            "ksp_its": 74,
            "missing": [],
            "n_dof": 3169395,
            "n_tet": 739584,
            "n_tet_counted_this_lane": true,
            "solve_rerun": false
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/g3.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.6003
          },
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "fail",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/g3_tip+00_t60_tt3.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6486
          },
          "geometry_sha256": "fba8743fd011683b4fd171eb73dcd098941d06d0f6ea949ee02ebafa0f2cf1e2",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 1.0036274369238576,
        "inputs": {
          "aes_score": 0.6486,
          "mass_g": 84.80209229536577,
          "seq_ratio": 0.9590825616164744,
          "solid_tip_sf_nodal_max": 1.6003,
          "spg": 4.14687
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 1.004 | 84.8 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "g3"
    },
    "w271b_11772": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11772_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11772_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 78.5515514925079
          },
          "evidence_path": "evidence/rs_v2/w271b_11772.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 78.5515514925079
          },
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11772_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.07566315653722758
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 306.3229630221624
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 306.3229630221624
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.418769313140643
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15492.611845093024
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4177895337213913
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5967145313584783
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w271b_11772.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.89964
          },
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.8823392882835103,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.8918998553971407
          },
          "evidence_path": "evidence/rs_v2/w271b_11772.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.8823392882835103
          },
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.8556596062528332,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w271b_11772.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "nonpositive_order",
            "gci_fine": null,
            "p": -0.6557511798638445,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w271b_11772.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "nonpositive_observed_order",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "843f64b9a1ab9270d43a488adcf55b57a68bdefb957513ecc4e9359eae0f8340"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11772_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "843f64b9a1ab9270d43a488adcf55b57a68bdefb957513ecc4e9359eae0f8340"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11772_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "843f64b9a1ab9270d43a488adcf55b57a68bdefb957513ecc4e9359eae0f8340"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11772_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2958.0586381575727,
            "buckling_eigenvalue": 11.377148608298356,
            "certified_seq_ratio": 0.8823392882835103,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "342c4292000854f1b5bc509bf5176b9dca19767a6c911c72f781218768e511de",
            "supported_seq_ratio": 3.819513355049345
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11772_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 8,
            "NSUB": 2,
            "SF_nodal_max": 0.8337,
            "SF_nodal_max_published_fine": 0.8337,
            "copied_from_another_genome": false,
            "ksp_its": 85,
            "missing": [],
            "n_dof": 2590443,
            "n_tet": 603648,
            "n_tet_counted_this_lane": true,
            "solve_rerun": false
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w271b_11772.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.8337
          },
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "fail",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w271b_11772.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6928
          },
          "geometry_sha256": "5418a4f1dde60341675a202b6b9d4d834b07c42a90ae0a11946a889039322cab",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7505798795492734,
        "inputs": {
          "aes_score": 0.6928,
          "mass_g": 78.5515514925079,
          "seq_ratio": 0.8823392882835103,
          "solid_tip_sf_nodal_max": 0.8337,
          "spg": 3.89964
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.751 | 78.6 g | jarambit v1.0.0"
      },
      "pass_count": 10,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w271b_11772"
    },
    "w271b_11860": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11860_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11860_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 73.32648797691759
          },
          "evidence_path": "evidence/rs_v2/w271b_11860.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 73.32648797691759
          },
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271b_11860_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.07501161359989089
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 270.6431212247853
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 270.6431212247853
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 1.911415462184051
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 12880.024538819793
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.384077611986592
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5275654193903646
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w271b_11860.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.69093
          },
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.8917967778864481,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.917559906566461
          },
          "evidence_path": "evidence/rs_v2/w271b_11860.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.8917967778864481
          },
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7559942103507366,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w271b_11860.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "nonpositive_order",
            "gci_fine": null,
            "p": -0.9593003902133572,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w271b_11860.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "nonpositive_observed_order",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "90d1e21eded6264dd5d6afc7a26b6af4027e3ac06e07f06c45b39a297474b071"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11860_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "90d1e21eded6264dd5d6afc7a26b6af4027e3ac06e07f06c45b39a297474b071"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11860_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "90d1e21eded6264dd5d6afc7a26b6af4027e3ac06e07f06c45b39a297474b071"
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11860_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1891.738519652392,
            "buckling_eigenvalue": 7.27591738327843,
            "certified_seq_ratio": 0.8917967778864481,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "660f86800f05462cb4af2abd3055522ead85cd8dd97de16fc511c57fd9d5cffc",
            "supported_seq_ratio": 3.4062552940501503
          },
          "evidence_path": "evidence/cad_battery/side/w271b_11860_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.814,
            "copied_from_another_genome": false,
            "d1_sha256": "2cc8206a9bcb6c2dc5b40182467b9e4877bb0bf20a010ee813ae66874f18d9e5",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 54,
            "missing": [],
            "n_dof": 351297,
            "n_tet": 76560,
            "solve_rerun": true
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w271b_11860.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.814
          },
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "unverified",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w271b_11860.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6332
          },
          "geometry_sha256": "3340300af53f3244b85d546c44e2065828ffe27b2d670567bfca937d0f192e03",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7216545740786019,
        "inputs": {
          "aes_score": 0.6332,
          "mass_g": 73.32648797691759,
          "seq_ratio": 0.8917967778864481,
          "solid_tip_sf_nodal_max": 0.814,
          "spg": 3.69093
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.722 | 73.3 g | jarambit v1.0.0"
      },
      "pass_count": 10,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w271b_11860"
    },
    "w271c_11522": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271c_11522.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271c_11522.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 79.96161512803431
          },
          "evidence_path": "evidence/rs_v2/w271c_11522.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 79.96161512803431
          },
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/genomes/w271c_11522.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.09651947249219368
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 274.69077251697746
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 274.69077251697746
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 1.9923488470274096
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 13403.844789238432
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4193974719454708
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5739417829204602
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w271c_11522.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.43528
          },
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.7784421108892615,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.8028716606255869
          },
          "evidence_path": "evidence/rs_v2/w271c_11522.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.7784421108892615
          },
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7673006161022223,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w271c_11522.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": true,
            "discretization_under_3pct": false,
            "fit_status": "admissible_estimate",
            "gci_fine": 0.053564791694462814,
            "p": 1.4699067526827303,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "admissible_estimate",
            "sf_hunter_gci_fine": 0.1587896306477363,
            "sf_hunter_p": 0.5423416628804812
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w271c_11522.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "plate.gci",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "0a41132f42bbe44bb5050839d47692c372987c58f728c7e4ad038ec0ce681bf5"
          },
          "evidence_path": "evidence/cad_battery/side/w271c_11522_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "0a41132f42bbe44bb5050839d47692c372987c58f728c7e4ad038ec0ce681bf5"
          },
          "evidence_path": "evidence/cad_battery/side/w271c_11522_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "0a41132f42bbe44bb5050839d47692c372987c58f728c7e4ad038ec0ce681bf5"
          },
          "evidence_path": "evidence/cad_battery/side/w271c_11522_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2933.9945939128033,
            "buckling_eigenvalue": 11.28459459197232,
            "certified_seq_ratio": 0.7784421108892615,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "1374605b3990f1e9385223ad3276cc4b35eb12986e6a3699e15664a36f775a97",
            "supported_seq_ratio": 3.0674209074117047
          },
          "evidence_path": "evidence/cad_battery/side/w271c_11522_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.033,
            "copied_from_another_genome": false,
            "d1_sha256": "b0f469388c9fbed04553c09803b6edc8db5a8909f3f9b197c113da69c850323c",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 57,
            "missing": [],
            "n_dof": 367767,
            "n_tet": 80160,
            "solve_rerun": true
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w271c_11522.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.033
          },
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "unverified",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w271c_11522.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6498
          },
          "geometry_sha256": "735d7c49199393af55653525fd19dfb6a0e075bb1e1c34c661e8cea39ab70276",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7597240123135478,
        "inputs": {
          "aes_score": 0.6498,
          "mass_g": 79.96161512803431,
          "seq_ratio": 0.7784421108892615,
          "solid_tip_sf_nodal_max": 1.033,
          "spg": 3.43528
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.760 | 80.0 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w271c_11522"
    },
    "w272_s2902_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w272_xbabe3/s2902/wave272_xbabe3_s2902_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w272_xbabe3/s2902/wave272_xbabe3_s2902_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 79.42608648214917
          },
          "evidence_path": "evidence/rs_v2/w272_xbabe3/w272_s2902_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 79.42608648214917
          },
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w272_xbabe3/s2902/wave272_xbabe3_s2902_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.07816312894639287
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 309.68814409318253
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 309.68814409318253
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.1738345379057935
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 13950.283085533229
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4299108699646969
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5406725958827919
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w272_xbabe3/w272_s2902_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.899227
          },
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.0000086521751033,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.0382084556958833
          },
          "evidence_path": "evidence/rs_v2/w272_xbabe3/w272_s2902_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.0000086521751033
          },
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.8650596508390767,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w272_xbabe3/w272_s2902_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "nonpositive_order",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": -0.6869933030739545
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w272_s2902_c01.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ec760260a0bae9f22f15648c2fcaa2dcd0c7be81b630d3c84b634a988eafa1"
          },
          "evidence_path": "evidence/cad_battery/side/w272_s2902_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ec760260a0bae9f22f15648c2fcaa2dcd0c7be81b630d3c84b634a988eafa1"
          },
          "evidence_path": "evidence/cad_battery/side/w272_s2902_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ec760260a0bae9f22f15648c2fcaa2dcd0c7be81b630d3c84b634a988eafa1"
          },
          "evidence_path": "evidence/cad_battery/side/w272_s2902_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1473.0486199477475,
            "buckling_eigenvalue": 5.665571615183645,
            "certified_seq_ratio": 1.0000086521751033,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "3d67803e726d65ab82952df93b75793add56e50f787484b7f826ce0dc7a9cb4c",
            "supported_seq_ratio": 4.603708175331892
          },
          "evidence_path": "evidence/cad_battery/side/w272_s2902_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.875,
            "copied_from_another_genome": false,
            "d1_sha256": "25498a873b28e96ef4637ee416244c2a6bdd29b30f40dbabb63b2cde71a3e547",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 52,
            "missing": [],
            "n_dof": 330453,
            "n_tet": 71952,
            "solve_rerun": true
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w272_s2902_c01.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.875
          },
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "unverified",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w272_xbabe3/w272_s2902_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6271
          },
          "geometry_sha256": "36bcc571bc9f800cbf53da830a66c69e6967ee745221ab824b5cb62a7899aa99",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7523889572632168,
        "inputs": {
          "aes_score": 0.6271,
          "mass_g": 79.42608648214917,
          "seq_ratio": 1.0000086521751033,
          "solid_tip_sf_nodal_max": 0.875,
          "spg": 3.899227
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.752 | 79.4 g | jarambit v1.0.0"
      },
      "pass_count": 10,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w272_s2902_c01"
    },
    "w273_s2903_c04": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2903/wave273_xbabe3_s2903_c04_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2903/wave273_xbabe3_s2903_c04_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 79.2991654804125
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2903_c04_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 79.2991654804125
          },
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2903/wave273_xbabe3_s2903_c04_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.07816030806039129
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 306.9769023207025
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 306.9769023207025
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.173834080610009
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 13950.269669140143
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.42990682478511694
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5406381848924569
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2903_c04_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.87112
          },
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.0088265601854227,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.0472756026638157
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2903_c04_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.0088265601854227
          },
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.8574862712771643,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2903_c04_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "nonpositive_order",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": -0.8718779177031006
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w273_s2903_c04.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ac27f1077a3eb9610a3985e25be9b9ef0d97e5cf4088800dfc2e21dac56286"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2903_c04_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ac27f1077a3eb9610a3985e25be9b9ef0d97e5cf4088800dfc2e21dac56286"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2903_c04_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "f6ac27f1077a3eb9610a3985e25be9b9ef0d97e5cf4088800dfc2e21dac56286"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2903_c04_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2356.235003457764,
            "buckling_eigenvalue": 9.0624423209914,
            "certified_seq_ratio": 1.0088265601854227,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "20f458a56cbb460538b13e15a76ffbc39df6fd6dea451aaf682e21f80f8dfc19",
            "supported_seq_ratio": 4.636671916752612
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2903_c04_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.875,
            "copied_from_another_genome": false,
            "d1_sha256": "c3076b1dbee5ef5599bc51a397228656c7f614fb432a358d15cc338e8c081392",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 51,
            "missing": [],
            "n_dof": 330453,
            "n_tet": 71952,
            "solve_rerun": true
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w273_s2903_c04.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.875
          },
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "unverified",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2903_c04_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6271
          },
          "geometry_sha256": "b6bbf3a22bb48a61ee1dd47eadd0d6a1db47320615c851aa897b878aad544695",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7509316126056635,
        "inputs": {
          "aes_score": 0.6271,
          "mass_g": 79.2991654804125,
          "seq_ratio": 1.0088265601854227,
          "solid_tip_sf_nodal_max": 0.875,
          "spg": 3.87112
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.751 | 79.3 g | jarambit v1.0.0"
      },
      "pass_count": 10,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w273_s2903_c04"
    },
    "w273_s2904_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2904/wave273_xbabe3_s2904_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2904/wave273_xbabe3_s2904_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 78.58519741483542
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2904_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 78.58519741483542
          },
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w273_xbabe3/s2904/wave273_xbabe3_s2904_c01_fit_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.07816312989858451
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 309.66384703207723
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 309.66384703207723
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.1134434366980357
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 13520.26429083442
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4298986433845655
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5406557044282082
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2904_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.94049
          },
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.0012028280770504,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.0179783550197032
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2904_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.0012028280770504
          },
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.8649917812496298,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2904_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": true,
            "discretization_under_3pct": false,
            "fit_status": "admissible_estimate",
            "gci_fine": 0.37082664118813286,
            "p": 0.2867815118213706,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "nonpositive_order",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": -0.681388547057524
          },
          "evidence_path": "evidence/cad_battery/plate_gci/w273_s2904_c01.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "plate.gci",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "164eac4827a29aa1f7bcc9e6d36abf067f4d1f0ba11908a99007629102649ccb"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2904_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "164eac4827a29aa1f7bcc9e6d36abf067f4d1f0ba11908a99007629102649ccb"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2904_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "164eac4827a29aa1f7bcc9e6d36abf067f4d1f0ba11908a99007629102649ccb"
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2904_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1860.9019172289704,
            "buckling_eigenvalue": 7.15731506626527,
            "certified_seq_ratio": 1.0012028280770504,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "7e64fd52cd1525bda96c4ee1f78fe3c0b9a241a7de56bec1368e29dfb2af752d",
            "supported_seq_ratio": 4.464915853368308
          },
          "evidence_path": "evidence/cad_battery/side/w273_s2904_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.875,
            "copied_from_another_genome": false,
            "d1_sha256": "3de317e8ee023ae3fd95b3d814da56cc0b4c48fb8f9384ae98ed6a84ff4e8c22",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 52,
            "missing": [],
            "n_dof": 330453,
            "n_tet": 71952,
            "solve_rerun": true
          },
          "evidence_path": "evidence/cad_battery/tip_mesh/w273_s2904_c01.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.875
          },
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "unverified",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "fail"
            }
          },
          "evidence_path": "evidence/rs_v2/w273_xbabe3/w273_s2904_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.6271
          },
          "geometry_sha256": "b46f9cf2181b21c06a8472939178ccca8b1ea729035eca32c094152c16c2bd63",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7602536724290332,
        "inputs": {
          "aes_score": 0.6271,
          "mass_g": 78.58519741483542,
          "seq_ratio": 1.0012028280770504,
          "solid_tip_sf_nodal_max": 0.875,
          "spg": 3.94049
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.760 | 78.6 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w273_s2904_c01"
    },
    "w274_s2905_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 84.97916178866379
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 84.97916178866379
          },
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12565751503254272
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 267.69283863318356
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 267.69283863318356
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.8210302521654995
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 18751.35725170091
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.3853925348444139
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.4858640261787062
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.1501
          },
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2730313869258605,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.3659513989214631
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2730313869258605
          },
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7477531120806018,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b2edc31873e763ba065e6ae928b70bda381aa88521b07c41ef66f9a95ed42415"
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b2edc31873e763ba065e6ae928b70bda381aa88521b07c41ef66f9a95ed42415"
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b2edc31873e763ba065e6ae928b70bda381aa88521b07c41ef66f9a95ed42415"
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2692.6898065806577,
            "buckling_eigenvalue": 10.356499256079452,
            "certified_seq_ratio": 1.2730313869258605,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "cb794417303b0069cfd919ecccbe037f7ee17df912104bb3ff2467946c561a70",
            "supported_seq_ratio": 2.7214781077131365
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.304,
            "copied_from_another_genome": false,
            "d1_sha256": "e3928d52edcc2793ab812dd632401701f71cc84f2624bb55bf86168d25b2d02c",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 61,
            "missing": [],
            "n_dof": 413613,
            "n_tet": 90384,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.304
          },
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w274_calibrated/w274_s2905_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.3896
          },
          "geometry_sha256": "6790f4a858771650159ba64cdf46521c3d57f514f59509988de8b0963b5b7ddf",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.7700579410566811,
        "inputs": {
          "aes_score": 0.3896,
          "mass_g": 84.97916178866379,
          "seq_ratio": 1.2730313869258605,
          "solid_tip_sf_nodal_max": 1.304,
          "spg": 3.1501
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.770 | 85.0 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w274_s2905_c01"
    },
    "w275_s2909_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 81.77438570899143
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 81.77438570899143
          },
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12500484804776787
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 274.4359066785205
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 274.4359066785205
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.395148012902199
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15911.563182239684
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.38442939063261394
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.4846895202412937
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.35601
          },
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2056272412039215,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2170076681062767
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2056272412039215
          },
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.766588692971061,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b65c5fabee2ae7fa0423d9ae57893bc1dfb4930667dc207d15c14c9ab5df1a4f"
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b65c5fabee2ae7fa0423d9ae57893bc1dfb4930667dc207d15c14c9ab5df1a4f"
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "b65c5fabee2ae7fa0423d9ae57893bc1dfb4930667dc207d15c14c9ab5df1a4f"
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 3412.0712596465005,
            "buckling_eigenvalue": 13.123350998640388,
            "certified_seq_ratio": 1.2056272412039215,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "304e810f5d2cd0987f7e7d88356b8d9e072c96c2d7e546a845dc78752fd93bb0",
            "supported_seq_ratio": 3.373461754805055
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.2967,
            "copied_from_another_genome": false,
            "d1_sha256": "b25e51f6878688e6d5bea6372cd7deabf1fd66b56a28d4a6c91b8f1127b03587",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 61,
            "missing": [],
            "n_dof": 413613,
            "n_tet": 90384,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.2967
          },
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.3896
          },
          "geometry_sha256": "d0defafe6d7eda0265cd3130ea1fbda6d1fc1161461b4092d93c89fab799ee6d",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8015439464550428,
        "inputs": {
          "aes_score": 0.3896,
          "mass_g": 81.77438570899143,
          "seq_ratio": 1.2056272412039215,
          "solid_tip_sf_nodal_max": 1.2967,
          "spg": 3.35601
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.802 | 81.8 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w275_s2909_c01"
    },
    "w276_s2913_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 79.99479010835532
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 79.99479010835532
          },
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12892114271644683
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 274.7789079802282
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 274.7789079802282
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.3115458858692217
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15402.166173778292
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.393424883962932
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.4958595341811897
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.43496
          },
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.212868756309451,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.212868756309451
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.212868756309451
          },
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7675468070995868,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "595f3a63ab584693fc76b4f920f40722423c56b113a4ea897d3984067464c043"
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "595f3a63ab584693fc76b4f920f40722423c56b113a4ea897d3984067464c043"
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "595f3a63ab584693fc76b4f920f40722423c56b113a4ea897d3984067464c043"
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2342.6801285601528,
            "buckling_eigenvalue": 9.010308186769818,
            "certified_seq_ratio": 1.212868756309451,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "87d9605778bc72b3f7d4e9ce3292da94c67e5d054cb9268c78d8a53149398253",
            "supported_seq_ratio": 4.096218692258367
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.3393,
            "copied_from_another_genome": false,
            "d1_sha256": "b855f4a3b1ed6c03ff770bbefee9a4b4d2fe59e661d9cc290a9d717e598ce8d5",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 61,
            "missing": [],
            "n_dof": 413613,
            "n_tet": 90384,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.3393
          },
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.3896
          },
          "geometry_sha256": "536b54c307aaf740e09be43f07df2a57b39e9182aca9a9a462f9d9acbaae37d0",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8322600494582232,
        "inputs": {
          "aes_score": 0.3896,
          "mass_g": 79.99479010835532,
          "seq_ratio": 1.212868756309451,
          "solid_tip_sf_nodal_max": 1.3393,
          "spg": 3.43496
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.832 | 80.0 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w276_s2913_c01"
    },
    "w277_s2919_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 79.5094905062415
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 79.5094905062415
          },
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.1289199647921879
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 273.2014249899909
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 273.2014249899909
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.305612693829547
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15245.538502524058
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.3934080825545573
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.49581856163371113
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.43609
          },
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2183232423497308,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2192247435495156
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2183232423497308
          },
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7631403843457062,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3bb0a2656abb2508f7427b7d9de67af41c88944bea02915a6ebedfeea2920c89"
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3bb0a2656abb2508f7427b7d9de67af41c88944bea02915a6ebedfeea2920c89"
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "3bb0a2656abb2508f7427b7d9de67af41c88944bea02915a6ebedfeea2920c89"
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1948.8964026434044,
            "buckling_eigenvalue": 7.495755394782324,
            "certified_seq_ratio": 1.2183232423497308,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "d2f3713c43b0e0e74a43a276fc0c38b4af819b228e2e13f0ae7b653f58f33877",
            "supported_seq_ratio": 3.399741158537304
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.3393,
            "copied_from_another_genome": false,
            "d1_sha256": "c71eb2f3f918881a70831746b9c5ad81cb9e2df553a7858760e0170d191673fd",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 61,
            "missing": [],
            "n_dof": 413613,
            "n_tet": 90384,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.3393
          },
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.3896
          },
          "geometry_sha256": "31d2c57317802f9b3bb37ac2616490c110ef07eacb9d62b795a861c3a7653663",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8347854224687924,
        "inputs": {
          "aes_score": 0.3896,
          "mass_g": 79.5094905062415,
          "seq_ratio": 1.2183232423497308,
          "solid_tip_sf_nodal_max": 1.3393,
          "spg": 3.43609
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.835 | 79.5 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w277_s2919_c01"
    },
    "w278_s2922_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 78.57968061789411
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 78.57968061789411
          },
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12332552880369892
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 273.4352054228489
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 273.4352054228489
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.229178609659206
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15027.997963060408
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4244558345861335
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5466244108593139
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.47972
          },
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2228727411771505,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2228727411771505
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2228727411771505
          },
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7637934090852012,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "d0f3a0f5eed3843093c152cc338ba17c05447c13a4098307452dd9e2d554aa1f"
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "d0f3a0f5eed3843093c152cc338ba17c05447c13a4098307452dd9e2d554aa1f"
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "d0f3a0f5eed3843093c152cc338ba17c05447c13a4098307452dd9e2d554aa1f"
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 3051.252587842093,
            "buckling_eigenvalue": 11.73558687631574,
            "certified_seq_ratio": 1.2228727411771505,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "53c0aca1f646c8b21167135c26910a490d996780a8f711c6b04d06f8cc981249",
            "supported_seq_ratio": 3.568698827533611
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.3819,
            "copied_from_another_genome": false,
            "d1_sha256": "a57004298ca9417fea17471650430faabd6569a6d3bdc69b123af97420376b9c",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 73,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.3819
          },
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "99a7d2c81cf32651683d4ed404820e556d99cd94a67159b3e34d2a8b8f25ea77",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8618220969105294,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 78.57968061789411,
          "seq_ratio": 1.2228727411771505,
          "solid_tip_sf_nodal_max": 1.3819,
          "spg": 3.47972
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.862 | 78.6 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w278_s2922_c01"
    },
    "w279_s2928_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 77.53467420484222
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 77.53467420484222
          },
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12620245469085123
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 274.2832287540307
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 274.2832287540307
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.1658273395075787
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 14570.880291339829
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4300736047781279
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.554118256907753
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.53756
          },
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2022212649660606,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2022212649660606
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2022212649660606
          },
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7661622138998756,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2427.8904743752964,
            "buckling_eigenvalue": 9.338040286058831,
            "certified_seq_ratio": 1.2022212649660606,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "1c46456c0c75e262dc0be04570d1ae159cdac30fba90a118751b9eead081a17d",
            "supported_seq_ratio": 3.877923828896846
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.3866,
            "copied_from_another_genome": false,
            "d1_sha256": "b87601e8e6f335012391ff78f5f13c7493a0609bd3336c2cda9b3f32d3a2dd6f",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 74,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.3866
          },
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8737531289757889,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 77.53467420484222,
          "seq_ratio": 1.2022212649660606,
          "solid_tip_sf_nodal_max": 1.3866,
          "spg": 3.53756
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.874 | 77.5 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w279_s2928_c01"
    },
    "w280_s2931_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 76.96542828827828
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 76.96542828827828
          },
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12620215483825892
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 273.342765198455
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 273.342765198455
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.173058163868794
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 14622.446741474274
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4300957926350708
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5541246002629963
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.5515
          },
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2015964090488782,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.205468301201764
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2015964090488782
          },
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7635351934906979,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2034.7598947443553,
            "buckling_eigenvalue": 7.825999595170598,
            "certified_seq_ratio": 1.2015964090488782,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "2e89f66640d9155122e40c2ff63cbdea08953bb831206513caf90f0a96a79d4d",
            "supported_seq_ratio": 3.889247942779815
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.3866,
            "copied_from_another_genome": false,
            "d1_sha256": "1104fc856d7eba16f28d0697811f1e55ebeae00ca9259f2aedea79e829e9c7b1",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 74,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.3866
          },
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.87764625,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 76.96542828827828,
          "seq_ratio": 1.2015964090488782,
          "solid_tip_sf_nodal_max": 1.3866,
          "spg": 3.5515
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.878 | 77.0 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w280_s2931_c01"
    },
    "w281_s2932_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 75.91091516167711
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 75.91091516167711
          },
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12815016334731302
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 265.43215483924297
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 265.43215483924297
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.155905190290204
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 14455.093590293656
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4318045077381332
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5560369842669616
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.49663
          },
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.211513817006697,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2233103040138726
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.211513817006697
          },
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7414382874069928,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 3041.6418881002987,
            "buckling_eigenvalue": 11.69862264653961,
            "certified_seq_ratio": 1.211513817006697,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "4a79e9358354b88a858de3cf7cf64fb1d195eda7d70015264c3596a02d955eb1",
            "supported_seq_ratio": 3.950007995935015
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.4094,
            "copied_from_another_genome": false,
            "d1_sha256": "b56c1ae3557abd3e40751575b924aabe30151fe51f18ee85e1dc20e037cab1da",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 73,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.4094
          },
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8808251249999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.91091516167711,
          "seq_ratio": 1.211513817006697,
          "solid_tip_sf_nodal_max": 1.4094,
          "spg": 3.49663
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.881 | 75.9 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w281_s2932_c01"
    },
    "w282_p10_c02": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 75.83557190472592
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 75.83557190472592
          },
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12815016334782955
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 265.40790341137307
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 265.40790341137307
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.145791197557754
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 14388.145220863837
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.4318045040633346
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.556036981873234
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.49978
          },
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2145102485240162,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.2175635322663738
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2145102485240162
          },
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7413705452860055,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 3041.561106070652,
            "buckling_eigenvalue": 11.698311946425584,
            "certified_seq_ratio": 1.2145102485240162,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "3a503cd683d7ea40a5b0cf61f062eb8f661ecd587a6bc6275e9f74f72998269e",
            "supported_seq_ratio": 3.950545606912083
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.4094,
            "copied_from_another_genome": false,
            "d1_sha256": "eb30e1b8e35a2ab51ecdd2a24f48b6c5d989698853cd736a8aebe1771666cd01",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 73,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.4094
          },
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8811007499999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.83557190472592,
          "seq_ratio": 1.2145102485240162,
          "solid_tip_sf_nodal_max": 1.4094,
          "spg": 3.49978
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.881 | 75.8 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w282_p10_c02"
    },
    "w282_p14_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 75.56848537322294
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 75.56848537322294
          },
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.12813258795748428
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 262.0064372966974
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 262.0064372966974
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.1457723877813017
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 14388.044481915833
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.431479965386194
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5555026784426982
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 3.46714
          },
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 1.2011929520821971,
            "seq_target": 1.2,
            "side_report_seq_ratio": 1.213051325188552
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 1.2011929520821971
          },
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "plate.seq",
          "kind": "gate",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.7318691447783486,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1521.565141269066,
            "buckling_eigenvalue": 5.8521736202656385,
            "certified_seq_ratio": 1.2011929520821971,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "1a8dd5c6f33227c0ced3d03235fbabf4f6302038dd2cd2f9149835e7472f85a4",
            "supported_seq_ratio": 4.042478315315074
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 1.4093,
            "copied_from_another_genome": false,
            "d1_sha256": "184e454feca52afffd156ce1b607025fb50dc3eca0687b835b36ef8f48c5fc46",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 73,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": true
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 1.4093
          },
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "pass",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "unverified",
              "human_max": "fail",
              "seq_hard": "pass",
              "solid_tip_sf": "pass",
              "strict_hunter_lateral": "pass",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
          "status": "fail"
        }
      ],
      "global_fitness": {
        "G": 0.8782097499999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.56848537322294,
          "seq_ratio": 1.2011929520821971,
          "solid_tip_sf_nodal_max": 1.4093,
          "spg": 3.46714
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.878 | 75.6 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w282_p14_c01"
    },
    "w283_p10_c02": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 75.6415527306541
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 75.6415527306541
          },
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.11099883356234613
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 194.42681634169116
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 194.42681634169116
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.8595559328398195
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15491.856474946993
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.43166014183598034
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5558736510517096
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 2.57037
          },
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.7179476490627898,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.7179476490627898
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.7179476490627898
          },
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.5430972966394524,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": true,
            "discretization_under_3pct": true,
            "fit_status": "admissible_estimate",
            "gci_fine": 0.006442822289578268,
            "p": 2.6233982835419463,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "admissible_estimate",
            "sf_hunter_gci_fine": 0.017190779522576458,
            "sf_hunter_p": 1.8854889735835478
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "plate.gci",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 2623.46885008842,
            "buckling_eigenvalue": 10.090264808032384,
            "certified_seq_ratio": 0.7179476490627898,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "2331b0f0007e5bf45ee1c1123ed4edd547c0dfc7486c49615c9ec6c28f267136",
            "supported_seq_ratio": 3.7872144876238547
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.9578,
            "copied_from_another_genome": false,
            "d1_sha256": "f41e6ca2d635ad008a5a3b0c680e50d01faae81743ee3b30660c3727100473a2",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 130,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": false
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.9578
          },
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "fail",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "fail",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "fail",
              "strict_hunter_lateral": "fail",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        }
      ],
      "certificate_identity": "29cc265209093fc44d3bedf788f1210438d27be31ec16e644738453c53f1e25b",
      "global_fitness": {
        "G": 0.6216318603776162,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.6415527306541,
          "seq_ratio": 0.7179476490627898,
          "solid_tip_sf_nodal_max": 0.9578,
          "spg": 2.57037
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.622 | 75.6 g | jarambit v1.0.0"
      },
      "pass_count": 11,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
      "subject": "w283_p10_c02"
    },
    "w283_p14_c01": {
      "cases": [
        {
          "evidence_path": "crates/physics/tests/invariance.rs",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.analytic",
          "kind": "software_contract",
          "solver": "analytic",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "fem_solid/tests/test_gci.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.gci_contracts",
          "kind": "software_contract",
          "solver": "gci",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "expected_unaided_exit": 2,
            "reason": "independent_caller_admission_unavailable"
          },
          "evidence_path": "ops/ci/check_claims.py",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.claims",
          "kind": "software_contract",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "all_within_2pct": true,
            "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
            "script": "fem_solid/cross_check.py",
            "subjects_checked": [
              "g3",
              "w271b_11772",
              "w271b_11860",
              "w271c_11522",
              "w272_s2902_c01",
              "w273_s2903_c04",
              "w273_s2904_c01",
              "w274_s2905_c01",
              "w275_s2909_c01",
              "w276_s2913_c01",
              "w277_s2919_c01",
              "w278_s2922_c01",
              "w279_s2928_c01",
              "w280_s2931_c01",
              "w281_s2932_c01",
              "w282_p10_c02",
              "w282_p14_c01",
              "w283_p10_c02",
              "w283_p14_c01"
            ],
            "tolerance_rel": 0.02
          },
          "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
          "fresh_this_lane": false,
          "geometry_sha256": null,
          "id": "build.dolfinx",
          "kind": "software_contract",
          "solver": "solid_dolfinx",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "geom.unit",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "geom.one_solid",
          "kind": "gate",
          "reason": "no step bound",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "detail": {
            "note": "fem::certify mass, not a STEP",
            "plate_mass_g": 75.38207297021373
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "mass_g": 75.38207297021373
          },
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "geom.mass",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "geom.pin",
          "kind": "software_contract",
          "reason": "genome is not a native CAD pin",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "blocked"
        },
        {
          "detail": {
            "loads": {
              "chord_twist_human_max_sf": {
                "compared_with": 16200,
                "value_n_or_sf": 0.110986187014051
              },
              "hiker_lateral_n": {
                "compared_with": 180,
                "value_n_or_sf": 194.81115243123332
              },
              "hunter_lateral_n": {
                "compared_with": 260,
                "value_n_or_sf": 194.81115243123332
              },
              "pry": null,
              "ring_back_sf": {
                "compared_with": 3000,
                "value_n_or_sf": 2.859525228086611
              },
              "ring_pull_n": {
                "compared_with": 1500,
                "value_n_or_sf": 15491.915610772525
              },
              "spine_sf": {
                "compared_with": 3427,
                "value_n_or_sf": 0.431348153525743
              },
              "tip_inplane_sf_record_only": {
                "compared_with": 1000,
                "value_n_or_sf": 0.5553764837608903
              }
            },
            "note": "derived from the certify record"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "spg": 2.58432
          },
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "plate.loads",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "seq_ratio": 0.743647399006731,
            "seq_target": 1.2,
            "side_report_seq_ratio": 0.743647399006731
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "seq_ratio": 0.743647399006731
          },
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "plate.seq",
          "kind": "gate",
          "reason": "below 1.2",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "cycles": "1e7",
            "fat_sf": 0.5441708722663069,
            "label": "screening",
            "load": "0..180 N",
            "not": "spec \u00a77.3 L7"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "plate.l7",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "admitted": false,
            "discretization_under_3pct": false,
            "fit_status": "oscillatory",
            "gci_fine": null,
            "p": null,
            "primary_observable": "seq_ratio",
            "refinement_ratio_r": 2.0,
            "sf_hunter_fit_status": "oscillatory",
            "sf_hunter_gci_fine": null,
            "sf_hunter_p": null
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_plate_gci.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "plate.gci",
          "kind": "derived",
          "reason": "oscillatory_sequence",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "side.a1",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "side.a2",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "changes_seq_ratio": false,
            "mesh_mm2": 0.25,
            "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "side.a3",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "bore_support": "radial_pin_bearing_and_flange",
            "buckling_critical_load_n": 1198.0424514916424,
            "buckling_eigenvalue": 4.6078555826601635,
            "certified_seq_ratio": 0.743647399006731,
            "changes_seq_ratio": false,
            "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
            "grip_foundation_stiffness": "winkler_grip_pad_foundation",
            "partial_solver_edit": false,
            "sha256": "78c627634c97598673724b3634ce1eed80e3e917cad489aed3e5ab3572203694",
            "supported_seq_ratio": 4.550625489280957
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a4.json",
          "fresh_this_lane": false,
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "side.a4",
          "kind": "derived",
          "solver": "plate_cst_dkt",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "NL": 4,
            "NSUB": 1,
            "SF_nodal_max": 0.9577,
            "copied_from_another_genome": false,
            "d1_sha256": "efbfcb1ddd68f93349f5fe5ae8270599c0adda88efd13618042b39e09afc836b",
            "entered_as_solver_run": true,
            "genome_data_row_matches_csv": true,
            "ksp_its": 130,
            "missing": [],
            "n_dof": 416583,
            "n_tet": 91008,
            "solve_rerun": false
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_solid_tip.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "solid_tip_sf_nodal_max": 0.9577
          },
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "solid.tip",
          "kind": "solver_run",
          "solver": "solid_skfem_p2",
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "pass"
        },
        {
          "detail": {
            "all_hard": false,
            "gates": {
              "aesthetic_and_geometric": "pass",
              "fatigue": "fail",
              "floors_828_joint": "fail",
              "gate_s_sharp_law": "fail",
              "gate_s_tip_point_brep": "fail",
              "human_max": "fail",
              "seq_hard": "fail",
              "solid_tip_sf": "fail",
              "strict_hunter_lateral": "fail",
              "twist_floor_body_and_tipsub_strict": "pass"
            }
          },
          "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
          "fresh_this_lane": false,
          "g_inputs": {
            "aes_score": 0.4079
          },
          "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
          "id": "gate.rs_v2",
          "kind": "gate",
          "reason": "does not pass every hard gate",
          "solver": null,
          "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
          "status": "fail"
        }
      ],
      "certificate_identity": "17576c751c5c0157cac7d098fd821915e54e811c5b7b2f9883889a0d0f11f245",
      "global_fitness": {
        "G": 0.623888308291947,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.38207297021373,
          "seq_ratio": 0.743647399006731,
          "solid_tip_sf_nodal_max": 0.9577,
          "spg": 2.58432
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.624 | 75.4 g | jarambit v1.0.0"
      },
      "pass_count": 10,
      "role": "genome",
      "schema": "jarambit.cad_battery.subject.v1",
      "search_fitness": "unchanged",
      "solver_run_count": 1,
      "solver_run_objects": 1,
      "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
      "subject": "w283_p14_c01"
    }
  },
  "case_ids": [
    "build.analytic",
    "build.gci_contracts",
    "build.claims",
    "build.dolfinx",
    "geom.unit",
    "geom.one_solid",
    "geom.mass",
    "geom.pin",
    "plate.loads",
    "plate.seq",
    "plate.l7",
    "plate.gci",
    "side.a1",
    "side.a2",
    "side.a3",
    "side.a4",
    "solid.tip",
    "gate.rs_v2"
  ],
  "featured_scorecards": {
    "c10a": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 93608,
        "manifold_solids": 1,
        "mass_3d_g": 64.8484,
        "mass_raster_g": 64.9498,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.001561,
        "step_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
        "step_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
        "stl_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.stl",
        "stl_sha256": "0bcadad61bff550d9c527266e183440e8242c3a0222b9a89755cdc682b214411",
        "units": "mm",
        "units_source": "step_declaration",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "units": "mm",
              "units_source": "step_declaration"
            },
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "geom.unit",
            "kind": "gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "solids": 1
            },
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "geom.one_solid",
            "kind": "gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "readme_g": 64.9498,
              "step_g": 64.8484,
              "step_vs_readme": {
                "rel": 0.001561,
                "status": "PASS",
                "tolerance": 0.001
              },
              "stl_g": 64.8484,
              "stl_vs_readme": {
                "rel": 0.001561,
                "status": "PASS",
                "tolerance": 0.01
              }
            },
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 64.8484
            },
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "geom.mass",
            "kind": "derived",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "candidate_status": "INCOMPLETE",
              "product_pass": false
            },
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "geom.pin",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "plate.loads",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "plate.seq",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "plate.l7",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "side.a1",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "side.a2",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "side.a3",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "designs/switchback/C10a/cad/switchback_revC10a_ground.step",
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "side.a4",
            "kind": "derived",
            "reason": "no mesh bound",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "R_CUT": 30.0,
              "SF_nodal_avg": 0.2216,
              "SF_nodal_max": 0.2101,
              "dolfinx_cross_check": {
                "SF_nodal_avg": 0.2216,
                "SF_nodal_max": 0.2101,
                "container": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
                "ksp_its": 172,
                "rel_err_nodal_avg": 0.0,
                "rel_err_nodal_max": 0.0,
                "within_2pct": true
              },
              "entered_as_solver_run": true,
              "ksp_its": 125,
              "n_dof": 372357,
              "n_tet": 80496
            },
            "evidence_path": "evidence/cad_battery/side/c10a_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 0.2101
            },
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": null,
            "fresh_this_lane": false,
            "geometry_sha256": "441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "B-rep is not an RS-karambit-v2 genome",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          }
        ],
        "global_fitness": {
          "G": null,
          "inputs": {
            "mass_g": 64.8484,
            "solid_tip_sf_nodal_max": 0.2101
          },
          "metric": "G1",
          "missing": [
            "spg: missing",
            "aes_score: missing",
            "seq_ratio: missing"
          ],
          "provisional": null,
          "status": "INCOMPLETE",
          "title": "G INCOMPLETE (3 missing) | 64.8 g | jarambit v1.0.0"
        },
        "pass_count": 9,
        "role": "native_cad",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "c10a"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.58,
        "buckling_eigenvalue_a4": 5.757,
        "fat_sf_l7": 0.3369,
        "lat_rob_cert_n": 120.61,
        "mass_3d_g": 64.8484,
        "mass_g": 64.9498,
        "plate_gci_fine": 0.018,
        "promote_eligible": false,
        "ring_bearing_w_mm": 4.5,
        "ring_rob_cert_n": 6752.4,
        "rs_v2_screen_plate_pass": false,
        "seq_ratio_cert": 0.4315,
        "seq_ratio_supported_a4": 1.3377,
        "sf_hunter_cert": 0.4639,
        "sf_hunter_strict": 0.4523,
        "sf_ring_cert": 4.5016,
        "solid_tip_dolfinx_rel_err": 0.0,
        "solid_tip_ksp_its": 125,
        "solid_tip_n_dof": 372357,
        "solid_tip_n_tet": 80496,
        "solid_tip_sf_nodal_avg": 0.2216,
        "solid_tip_sf_nodal_max": 0.2101,
        "spg_n_per_g": 1.857,
        "strict_lat_rob_0p0625_n": 117.59,
        "t_stock_mm": 4.5,
        "twist_body_sf": 0.452,
        "twist_tip_sf": 0.25,
        "twist_tipzone_sf_cert": 0.22,
        "twist_tipzone_sf_strict": 0.22
      },
      "evaluation_mode": "direct_3d_cad_mesh_and_brep",
      "genome_csv": null,
      "genome_sha256": null,
      "global_fitness_G1": {
        "G": 0.32000166666666663,
        "inputs": {
          "aes_score": 0.58,
          "mass_g": 64.9498,
          "seq_ratio": 0.4315,
          "solid_tip_sf_nodal_max": 0.2101,
          "spg": 1.857
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.58,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.6404166666666666,
          "spg_n": 0.46425,
          "status": "COMPLETE",
          "tip": 0.2101,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.320 | 64.9 g | jarambit v1.0.0"
      },
      "input_target": "c10a",
      "karambit_archetype": {
        "blade_min_in": 3.4692,
        "c6_ring_groove_w_mm": 3.39,
        "hook_drop_mm": 41.733,
        "inside_concave_fraction": 0.95,
        "karambit_valid": true,
        "ring_id_mm": 25.4,
        "ring_wall_mm": 6.3,
        "spine_convex_fraction": 0.95,
        "teeth_in": 4,
        "teeth_sp": 4
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "c10a",
      "weighted_cad_score": {
        "W_CAD": -14.12419,
        "all_engineering_gates_pass": false,
        "components": {
          "base_feasible": -1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.079,
          "mass_term_70pct": 0.7,
          "seq_buffer_5pct": 0.0,
          "shortfall_penalty": 13.22419,
          "survival_buffer_10pct": 0.0
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": false,
          "cert_hunter_260n": false,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": false,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": false,
          "strict_hunter_260n": false,
          "twist_body_1p82nm": false,
          "twist_tip_only_1p82nm": false,
          "twist_tip_submodel_strict_1p82nm": false
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.658333,
          "s_mass": 1.0,
          "s_seq_knee": 0.0,
          "s_survival_knee": 0.0
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.5477307692307692,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.6404166666666666,
          "solid_tip_3d": 0.7899,
          "twist_body": 0.548,
          "twist_tip_strict": 0.78
        },
        "unpenalized_quality_score": 1.797,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w279_s2928_c01": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15672,
        "manifold_solids": 1,
        "mass_3d_g": 77.4142,
        "mass_raster_g": 77.535,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.001558,
        "step_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01.step",
        "step_sha256": "f5788696f75dc835db0c19e0167c462f27d982b635ff07a4fd9e267405b2b004",
        "stl_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01.stl",
        "stl_sha256": "cad555ac0bb94cbda376897d2b1c04c6161e8e19fdfc6c8ada1edfb63eb5f681",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 77.53467420484222
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 77.53467420484222
            },
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.12620245469085123
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 274.2832287540307
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 274.2832287540307
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.1658273395075787
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 14570.880291339829
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.4300736047781279
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.554118256907753
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 3.53756
            },
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 1.2022212649660606,
              "seq_target": 1.2,
              "side_report_seq_ratio": 1.2022212649660606
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 1.2022212649660606
            },
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "plate.seq",
            "kind": "gate",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.7661622138998756,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "1c9dc09a5eef6792abe493aa086a95db8775aceb2cbebc87f3afab85f1b6842d"
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 2427.8904743752964,
              "buckling_eigenvalue": 9.338040286058831,
              "certified_seq_ratio": 1.2022212649660606,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "1c46456c0c75e262dc0be04570d1ae159cdac30fba90a118751b9eead081a17d",
              "supported_seq_ratio": 3.877923828896846
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 1.3866,
              "copied_from_another_genome": false,
              "d1_sha256": "b87601e8e6f335012391ff78f5f13c7493a0609bd3336c2cda9b3f32d3a2dd6f",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 74,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": true
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 1.3866
            },
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "pass",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "unverified",
                "human_max": "fail",
                "seq_hard": "pass",
                "solid_tip_sf": "pass",
                "strict_hunter_lateral": "pass",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          }
        ],
        "global_fitness": {
          "G": 0.8737531289757889,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 77.53467420484222,
            "seq_ratio": 1.2022212649660606,
            "solid_tip_sf_nodal_max": 1.3866,
            "spg": 3.53756
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.874 | 77.5 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "w279_s2928_c01"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 9.338040286058831,
        "fat_sf_l7": 0.7661622138998756,
        "lat_rob_cert_n": 274.3,
        "mass_3d_g": 77.4142,
        "mass_g": 77.535,
        "plate_gci_fine": null,
        "promote_eligible": true,
        "ring_bearing_w_mm": 5.292684555053711,
        "ring_rob_cert_n": 14570.9,
        "rs_v2_screen_plate_pass": true,
        "seq_ratio_cert": 1.202,
        "seq_ratio_supported_a4": 3.877923828896846,
        "sf_hunter_cert": 1.055,
        "sf_hunter_strict": 1.0554,
        "sf_ring_cert": 9.714,
        "solid_tip_dolfinx_rel_err": 0.000288,
        "solid_tip_sf_nodal_avg": 1.4721,
        "solid_tip_sf_nodal_max": 1.3866,
        "spg_n_per_g": 3.53756,
        "strict_lat_rob_0p0625_n": 274.4,
        "t_stock_mm": 5.9,
        "twist_body_sf": 3.192,
        "twist_tip_sf": 1.123,
        "twist_tipzone_sf_cert": 1.123,
        "twist_tipzone_sf_strict": 1.107
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_genome.csv",
      "genome_sha256": "56fb45ba5f98862c89eacd8c5d49ecd42b091497b1464981bd46b7a77371106c",
      "global_fitness_G1": {
        "G": 0.8737515,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 77.535,
          "seq_ratio": 1.202,
          "solid_tip_sf_nodal_max": 1.3866,
          "spg": 3.53756
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.05349999999999966,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.0,
          "spg_n": 0.88439,
          "status": "COMPLETE",
          "tip": 1.3866,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.874 | 77.5 g | jarambit v0.32.5"
      },
      "input_target": "w279_s2928_c01",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034484,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w279_s2928_c01",
      "weighted_cad_score": {
        "W_CAD": 1.72123,
        "all_engineering_gates_pass": true,
        "components": {
          "base_feasible": 1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.558133,
          "seq_buffer_5pct": 0.003225,
          "shortfall_penalty": 0.0,
          "survival_buffer_10pct": 0.093769
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": true,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": true,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": true,
          "strict_hunter_260n": true,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": true,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.797332,
          "s_seq_knee": 0.064493,
          "s_survival_knee": 0.937691
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.0,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.0,
          "solid_tip_3d": 0.0,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.72123,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w280_s2931_c01": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15624,
        "manifold_solids": 1,
        "mass_3d_g": 76.8339,
        "mass_raster_g": 76.965,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.001703,
        "step_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01.step",
        "step_sha256": "d71ce204f32dbc22985ce902117753826d7d93c0993277191ac6989638b959f4",
        "stl_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01.stl",
        "stl_sha256": "9f8934ef5b49ba6a591dee119f1028f465742d9beca7a431d878958f54b08f42",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 76.96542828827828
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 76.96542828827828
            },
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.12620215483825892
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 273.342765198455
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 273.342765198455
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.173058163868794
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 14622.446741474274
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.4300957926350708
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.5541246002629963
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 3.5515
            },
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 1.2015964090488782,
              "seq_target": 1.2,
              "side_report_seq_ratio": 1.205468301201764
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 1.2015964090488782
            },
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "plate.seq",
            "kind": "gate",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.7635351934906979,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "6729d1cc6fa945a8590de7bdfcab4f65b9b1f707a2d1d48603b3b6381be1699b"
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 2034.7598947443553,
              "buckling_eigenvalue": 7.825999595170598,
              "certified_seq_ratio": 1.2015964090488782,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "2e89f66640d9155122e40c2ff63cbdea08953bb831206513caf90f0a96a79d4d",
              "supported_seq_ratio": 3.889247942779815
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 1.3866,
              "copied_from_another_genome": false,
              "d1_sha256": "1104fc856d7eba16f28d0697811f1e55ebeae00ca9259f2aedea79e829e9c7b1",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 74,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": true
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 1.3866
            },
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "pass",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "unverified",
                "human_max": "fail",
                "seq_hard": "pass",
                "solid_tip_sf": "pass",
                "strict_hunter_lateral": "pass",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          }
        ],
        "global_fitness": {
          "G": 0.87764625,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 76.96542828827828,
            "seq_ratio": 1.2015964090488782,
            "solid_tip_sf_nodal_max": 1.3866,
            "spg": 3.5515
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.878 | 77.0 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "w280_s2931_c01"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 7.825999595170598,
        "fat_sf_l7": 0.7635351934906979,
        "lat_rob_cert_n": 273.3,
        "mass_3d_g": 76.8339,
        "mass_g": 76.965,
        "plate_gci_fine": null,
        "promote_eligible": true,
        "ring_bearing_w_mm": 5.300562858581543,
        "ring_rob_cert_n": 14622.4,
        "rs_v2_screen_plate_pass": true,
        "seq_ratio_cert": 1.202,
        "seq_ratio_supported_a4": 3.889247942779815,
        "sf_hunter_cert": 1.051,
        "sf_hunter_strict": 1.0515,
        "sf_ring_cert": 9.748,
        "solid_tip_dolfinx_rel_err": 0.000288,
        "solid_tip_sf_nodal_avg": 1.4721,
        "solid_tip_sf_nodal_max": 1.3866,
        "spg_n_per_g": 3.5515,
        "strict_lat_rob_0p0625_n": 273.4,
        "t_stock_mm": 5.9,
        "twist_body_sf": 3.178,
        "twist_tip_sf": 1.123,
        "twist_tipzone_sf_cert": 1.123,
        "twist_tipzone_sf_strict": 1.107
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_genome.csv",
      "genome_sha256": "f3badbf674517f6252a3ef9875d90a1a0931d5ae01c0c60514ea65ea93d77a96",
      "global_fitness_G1": {
        "G": 0.87764625,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 76.96542828827828,
          "seq_ratio": 1.2015964090488782,
          "solid_tip_sf_nodal_max": 1.3866,
          "spg": 3.5515
        },
        "metric": "G1",
        "missing": [],
        "provisional": false,
        "status": "COMPLETE",
        "title": "G 0.878 | 77.0 g | jarambit v0.32.7"
      },
      "input_target": "w280_s2931_c01",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034484,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w280_s2931_c01",
      "weighted_cad_score": {
        "W_CAD": 1.743186,
        "all_engineering_gates_pass": true,
        "components": {
          "base_feasible": 1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.580844,
          "seq_buffer_5pct": 0.003225,
          "shortfall_penalty": 0.0,
          "survival_buffer_10pct": 0.093013
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": true,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": true,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": true,
          "strict_hunter_260n": true,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": true,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.829777,
          "s_seq_knee": 0.064493,
          "s_survival_knee": 0.930135
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.0,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.0,
          "solid_tip_3d": 0.0,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.743186,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w281_s2932_c01": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15580,
        "manifold_solids": 1,
        "mass_3d_g": 75.6954,
        "mass_raster_g": 75.911,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.00284,
        "step_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01.step",
        "step_sha256": "2a02a27a1fdfa538e3c141b95869ca85c62b34463804bb12890c0e1cdf72a3f5",
        "stl_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01.stl",
        "stl_sha256": "3991f7bec4ebd32d081c58735421a579c30b2f81a7543b6d08e10dfd0999006a",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 75.91091516167711
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 75.91091516167711
            },
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.12815016334731302
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 265.43215483924297
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 265.43215483924297
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.155905190290204
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 14455.093590293656
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.4318045077381332
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.5560369842669616
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 3.49663
            },
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 1.211513817006697,
              "seq_target": 1.2,
              "side_report_seq_ratio": 1.2233103040138726
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 1.211513817006697
            },
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "plate.seq",
            "kind": "gate",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.7414382874069928,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "c15b02a0ed5234c17cb8338dc1e982db9ed1365d4c3cb735314e087dbd7fbc60"
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 3041.6418881002987,
              "buckling_eigenvalue": 11.69862264653961,
              "certified_seq_ratio": 1.211513817006697,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "4a79e9358354b88a858de3cf7cf64fb1d195eda7d70015264c3596a02d955eb1",
              "supported_seq_ratio": 3.950007995935015
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 1.4094,
              "copied_from_another_genome": false,
              "d1_sha256": "b56c1ae3557abd3e40751575b924aabe30151fe51f18ee85e1dc20e037cab1da",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 73,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": true
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 1.4094
            },
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "pass",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "unverified",
                "human_max": "fail",
                "seq_hard": "pass",
                "solid_tip_sf": "pass",
                "strict_hunter_lateral": "pass",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          }
        ],
        "global_fitness": {
          "G": 0.8808251249999999,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 75.91091516167711,
            "seq_ratio": 1.211513817006697,
            "solid_tip_sf_nodal_max": 1.4094,
            "spg": 3.49663
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.881 | 75.9 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "w281_s2932_c01"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 11.69862264653961,
        "fat_sf_l7": 0.7414382874069928,
        "lat_rob_cert_n": 265.4,
        "mass_3d_g": 75.6954,
        "mass_g": 75.911,
        "plate_gci_fine": null,
        "promote_eligible": true,
        "ring_bearing_w_mm": 5.3006134033203125,
        "ring_rob_cert_n": 14455.1,
        "rs_v2_screen_plate_pass": true,
        "seq_ratio_cert": 1.212,
        "seq_ratio_supported_a4": 3.950007995935015,
        "sf_hunter_cert": 1.021,
        "sf_hunter_strict": 1.0285,
        "sf_ring_cert": 9.637,
        "solid_tip_dolfinx_rel_err": 0.000284,
        "solid_tip_sf_nodal_avg": 1.496,
        "solid_tip_sf_nodal_max": 1.4094,
        "spg_n_per_g": 3.49663,
        "strict_lat_rob_0p0625_n": 267.4,
        "t_stock_mm": 5.9,
        "twist_body_sf": 3.015,
        "twist_tip_sf": 1.141,
        "twist_tipzone_sf_cert": 1.141,
        "twist_tipzone_sf_strict": 1.125
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_genome.csv",
      "genome_sha256": "6de776dfd45031b1697c13072afc5555eb7b8402a38defe593eaece6022b413e",
      "global_fitness_G1": {
        "G": 0.8808251249999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.911,
          "seq_ratio": 1.212,
          "solid_tip_sf_nodal_max": 1.4094,
          "spg": 3.49663
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.0,
          "spg_n": 0.8741575,
          "status": "COMPLETE",
          "tip": 1.4094,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.881 | 75.9 g | jarambit v0.32.8"
      },
      "input_target": "w281_s2932_c01",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034484,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w281_s2932_c01",
      "weighted_cad_score": {
        "W_CAD": 1.79165,
        "all_engineering_gates_pass": true,
        "components": {
          "base_feasible": 1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.623003,
          "seq_buffer_5pct": 0.016484,
          "shortfall_penalty": 0.0,
          "survival_buffer_10pct": 0.086059
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": true,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": true,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": true,
          "strict_hunter_260n": true,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": true,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.890005,
          "s_seq_knee": 0.32968,
          "s_survival_knee": 0.860591
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.0,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.0,
          "solid_tip_3d": 0.0,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.79165,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w282_p10_c02": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15580,
        "manifold_solids": 1,
        "mass_3d_g": 75.6157,
        "mass_raster_g": 75.836,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.002905,
        "step_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02.step",
        "step_sha256": "f587fd3bb944bb1ca633d65644401144bfc70c9543e9abd5ebc9713ba729b253",
        "stl_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02.stl",
        "stl_sha256": "c6d927f2795564b407487099a6f8a40c8fded52d83bd072fd22eafae9e46704f",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 75.83557190472592
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 75.83557190472592
            },
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.12815016334782955
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 265.40790341137307
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 265.40790341137307
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.145791197557754
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 14388.145220863837
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.4318045040633346
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.556036981873234
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 3.49978
            },
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 1.2145102485240162,
              "seq_target": 1.2,
              "side_report_seq_ratio": 1.2175635322663738
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 1.2145102485240162
            },
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "plate.seq",
            "kind": "gate",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.7413705452860055,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "aa971b7145a5adab2e3947661a1c0fbe744097de200ac49648500a2a54412804"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 3041.561106070652,
              "buckling_eigenvalue": 11.698311946425584,
              "certified_seq_ratio": 1.2145102485240162,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "3a503cd683d7ea40a5b0cf61f062eb8f661ecd587a6bc6275e9f74f72998269e",
              "supported_seq_ratio": 3.950545606912083
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 1.4094,
              "copied_from_another_genome": false,
              "d1_sha256": "eb30e1b8e35a2ab51ecdd2a24f48b6c5d989698853cd736a8aebe1771666cd01",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 73,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": true
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 1.4094
            },
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "pass",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "unverified",
                "human_max": "fail",
                "seq_hard": "pass",
                "solid_tip_sf": "pass",
                "strict_hunter_lateral": "pass",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          }
        ],
        "global_fitness": {
          "G": 0.8811007499999999,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 75.83557190472592,
            "seq_ratio": 1.2145102485240162,
            "solid_tip_sf_nodal_max": 1.4094,
            "spg": 3.49978
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.881 | 75.8 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "w282_p10_c02"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 11.698311946425584,
        "fat_sf_l7": 0.7413705452860055,
        "lat_rob_cert_n": 265.4,
        "mass_3d_g": 75.6157,
        "mass_g": 75.836,
        "plate_gci_fine": null,
        "promote_eligible": true,
        "ring_bearing_w_mm": 5.259920597076416,
        "ring_rob_cert_n": 14388.1,
        "rs_v2_screen_plate_pass": true,
        "seq_ratio_cert": 1.215,
        "seq_ratio_supported_a4": 3.950545606912083,
        "sf_hunter_cert": 1.021,
        "sf_hunter_strict": 1.0285,
        "sf_ring_cert": 9.592,
        "solid_tip_dolfinx_rel_err": 0.000284,
        "solid_tip_sf_nodal_avg": 1.496,
        "solid_tip_sf_nodal_max": 1.4094,
        "spg_n_per_g": 3.49978,
        "strict_lat_rob_0p0625_n": 267.4,
        "t_stock_mm": 5.9,
        "twist_body_sf": 3.015,
        "twist_tip_sf": 1.141,
        "twist_tipzone_sf_cert": 1.141,
        "twist_tipzone_sf_strict": 1.125
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/rs_v2/w282_ultrarunner/w282_p10_c02_genome.csv",
      "genome_sha256": "538c092e0d4943e225db0b5809f74a6c9e8813333f56752cf69dab70c0f906b1",
      "global_fitness_G1": {
        "G": 0.8811007499999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.836,
          "seq_ratio": 1.215,
          "solid_tip_sf_nodal_max": 1.4094,
          "spg": 3.49978
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.0,
          "spg_n": 0.874945,
          "status": "COMPLETE",
          "tip": 1.4094,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.881 | 75.8 g | jarambit v0.32.9"
      },
      "input_target": "w282_p10_c02",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034484,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w282_p10_c02",
      "weighted_cad_score": {
        "W_CAD": 1.797848,
        "all_engineering_gates_pass": true,
        "components": {
          "base_feasible": 1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.626012,
          "seq_buffer_5pct": 0.019673,
          "shortfall_penalty": 0.0,
          "survival_buffer_10pct": 0.086059
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": true,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": true,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": true,
          "strict_hunter_260n": true,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": true,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.894302,
          "s_seq_knee": 0.393469,
          "s_survival_knee": 0.860591
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.0,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.0,
          "solid_tip_3d": 0.0,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.797848,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w282_p14_c01": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15552,
        "manifold_solids": 1,
        "mass_3d_g": 75.3233,
        "mass_raster_g": 75.568,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.003238,
        "step_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01.step",
        "step_sha256": "e6441603c226d3dfd51c63bbfd0cf979a81e0794003b6e3cd64ba198193c1291",
        "stl_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01.stl",
        "stl_sha256": "b2ba70cf9b34ea5771a364f24d48121e48761ff0830e8e0a26cccbadb105e0e7",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 75.56848537322294
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 75.56848537322294
            },
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.12813258795748428
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 262.0064372966974
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 262.0064372966974
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.1457723877813017
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 14388.044481915833
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.431479965386194
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.5555026784426982
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 3.46714
            },
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 1.2011929520821971,
              "seq_target": 1.2,
              "side_report_seq_ratio": 1.213051325188552
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 1.2011929520821971
            },
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "plate.seq",
            "kind": "gate",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.7318691447783486,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "47fbd0863ff7e7a4f071e553726a25e27abedc9d33db441be25fddbccb8f9d3b"
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 1521.565141269066,
              "buckling_eigenvalue": 5.8521736202656385,
              "certified_seq_ratio": 1.2011929520821971,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "1a8dd5c6f33227c0ced3d03235fbabf4f6302038dd2cd2f9149835e7472f85a4",
              "supported_seq_ratio": 4.042478315315074
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 1.4093,
              "copied_from_another_genome": false,
              "d1_sha256": "184e454feca52afffd156ce1b607025fb50dc3eca0687b835b36ef8f48c5fc46",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 73,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": true
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 1.4093
            },
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "pass",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "unverified",
                "human_max": "fail",
                "seq_hard": "pass",
                "solid_tip_sf": "pass",
                "strict_hunter_lateral": "pass",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
            "status": "fail"
          }
        ],
        "global_fitness": {
          "G": 0.8782097499999999,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 75.56848537322294,
            "seq_ratio": 1.2011929520821971,
            "solid_tip_sf_nodal_max": 1.4093,
            "spg": 3.46714
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.878 | 75.6 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
        "subject": "w282_p14_c01"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 5.8521736202656385,
        "fat_sf_l7": 0.7318691447783486,
        "lat_rob_cert_n": 262.0,
        "mass_3d_g": 75.3233,
        "mass_g": 75.568,
        "plate_gci_fine": null,
        "promote_eligible": true,
        "ring_bearing_w_mm": 5.259920597076416,
        "ring_rob_cert_n": 14388.0,
        "rs_v2_screen_plate_pass": true,
        "seq_ratio_cert": 1.201,
        "seq_ratio_supported_a4": 4.042478315315074,
        "sf_hunter_cert": 1.008,
        "sf_hunter_strict": 1.01,
        "sf_ring_cert": 9.592,
        "solid_tip_dolfinx_rel_err": 0.000213,
        "solid_tip_sf_nodal_avg": 1.4958,
        "solid_tip_sf_nodal_max": 1.4093,
        "spg_n_per_g": 3.46714,
        "strict_lat_rob_0p0625_n": 262.6,
        "t_stock_mm": 5.9,
        "twist_body_sf": 2.982,
        "twist_tip_sf": 1.141,
        "twist_tipzone_sf_cert": 1.141,
        "twist_tipzone_sf_strict": 1.125
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/rs_v2/w282_ultrarunner/w282_p14_c01_genome.csv",
      "genome_sha256": "5f463175fd6e900b566265d66e113171d6e91ac808896e09ee8bc24b5061dc09",
      "global_fitness_G1": {
        "G": 0.8782097499999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.568,
          "seq_ratio": 1.201,
          "solid_tip_sf_nodal_max": 1.4093,
          "spg": 3.46714
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.0,
          "spg_n": 0.866785,
          "status": "COMPLETE",
          "tip": 1.4093,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.878 | 75.6 g | jarambit v0.32.9"
      },
      "input_target": "w282_p14_c01",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034484,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "6d18ef8cbe7b995890451a8ac7b93b75c928d150",
      "subject": "w282_p14_c01",
      "weighted_cad_score": {
        "W_CAD": 1.779046,
        "all_engineering_gates_pass": true,
        "components": {
          "base_feasible": 1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.63677,
          "seq_buffer_5pct": 0.001639,
          "shortfall_penalty": 0.0,
          "survival_buffer_10pct": 0.074534
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": true,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": true,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": true,
          "strict_hunter_260n": true,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": true,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.909671,
          "s_seq_knee": 0.032784,
          "s_survival_knee": 0.745338
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.0,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.0,
          "solid_tip_3d": 0.0,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.779046,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      }
    },
    "w283_p10_c02": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15580,
        "manifold_solids": 1,
        "mass_3d_g": 75.5049,
        "mass_raster_g": 75.642,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.001812,
        "step_path": "evidence/cert_integrity/recertified/w283_p10_c02/w283_p10_c02.step",
        "step_sha256": "10714965a66feef04d7777142e3b27dad9c104a73f1068e545be2946c4f5f735",
        "stl_path": "evidence/cert_integrity/recertified/w283_p10_c02/w283_p10_c02.stl",
        "stl_sha256": "bff0a92fdc3615b31e021d5283b95857ecef6ff54d204dd7db11389767c1ced9",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 75.6415527306541
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 75.6415527306541
            },
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.11099883356234613
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 194.42681634169116
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 194.42681634169116
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.8595559328398195
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 15491.856474946993
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.43166014183598034
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.5558736510517096
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 2.57037
            },
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 0.7179476490627898,
              "seq_target": 1.2,
              "side_report_seq_ratio": 0.7179476490627898
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 0.7179476490627898
            },
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "plate.seq",
            "kind": "gate",
            "reason": "below 1.2",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.5430972966394524,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": true,
              "discretization_under_3pct": true,
              "fit_status": "admissible_estimate",
              "gci_fine": 0.006442822289578268,
              "p": 2.6233982835419463,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "admissible_estimate",
              "sf_hunter_gci_fine": 0.017190779522576458,
              "sf_hunter_p": 1.8854889735835478
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "plate.gci",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "adaa04ec6949df7b9037cc621d9ce01383dbbb586a24cc777a4d7b9840aa94af"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 2623.46885008842,
              "buckling_eigenvalue": 10.090264808032384,
              "certified_seq_ratio": 0.7179476490627898,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "2331b0f0007e5bf45ee1c1123ed4edd547c0dfc7486c49615c9ec6c28f267136",
              "supported_seq_ratio": 3.7872144876238547
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 0.9578,
              "copied_from_another_genome": false,
              "d1_sha256": "f41e6ca2d635ad008a5a3b0c680e50d01faae81743ee3b30660c3727100473a2",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 130,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": false
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 0.9578
            },
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "fail",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "fail",
                "human_max": "fail",
                "seq_hard": "fail",
                "solid_tip_sf": "fail",
                "strict_hunter_lateral": "fail",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p10_c02_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          }
        ],
        "certificate_identity": "29cc265209093fc44d3bedf788f1210438d27be31ec16e644738453c53f1e25b",
        "global_fitness": {
          "G": 0.6216318603776162,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 75.6415527306541,
            "seq_ratio": 0.7179476490627898,
            "solid_tip_sf_nodal_max": 0.9578,
            "spg": 2.57037
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.622 | 75.6 g | jarambit v1.0.0"
        },
        "pass_count": 11,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
        "subject": "w283_p10_c02"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 10.090264808032384,
        "fat_sf_l7": 0.5430972966394524,
        "lat_rob_cert_n": 194.4,
        "mass_3d_g": 75.5049,
        "mass_g": 75.642,
        "plate_gci_fine": 0.006442822289578268,
        "promote_eligible": false,
        "ring_bearing_w_mm": 10.199999809265137,
        "ring_rob_cert_n": 15491.9,
        "rs_v2_all_hard": false,
        "rs_v2_screen_plate_pass": false,
        "seq_ratio_cert": 0.718,
        "seq_ratio_supported_a4": 3.7872144876238547,
        "sf_hunter_cert": 0.748,
        "sf_hunter_strict": 0.68,
        "sf_ring_cert": 10.328,
        "solid_tip_dolfinx_rel_err": 0.000209,
        "solid_tip_sf_nodal_avg": 1.0158,
        "solid_tip_sf_nodal_max": 0.9578,
        "spg_n_per_g": 2.57037,
        "strict_lat_rob_0p0625_n": 176.8,
        "t_stock_mm": 5.9,
        "twist_body_sf": 3.001,
        "twist_tip_sf": 0.988,
        "twist_tipzone_sf_cert": 0.988,
        "twist_tipzone_sf_strict": 1.002
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/cert_integrity/recertified/w283_p10_c02/w283_p10_c02_genome.csv",
      "genome_sha256": "61594e106aedc7ecd9438e8867d8e743b1a7620f578b39951bf9851077b4fdac",
      "global_fitness_G1": {
        "G": 0.6216340416666666,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.642,
          "seq_ratio": 0.718,
          "solid_tip_sf_nodal_max": 0.9578,
          "spg": 2.57037
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.40166666666666667,
          "spg_n": 0.6425925,
          "status": "COMPLETE",
          "tip": 0.9578,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.622 | 75.6 g | jarambit v1.0.0"
      },
      "input_target": "w283_p10_c02",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034483,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "97388249afbe65a9974fd1b6b9774d0b70da2e9a",
      "subject": "w283_p10_c02",
      "weighted_cad_score": {
        "W_CAD": -3.964924,
        "all_engineering_gates_pass": false,
        "components": {
          "base_feasible": -1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.633798,
          "seq_buffer_5pct": 0.0,
          "shortfall_penalty": 3.055467,
          "survival_buffer_10pct": 0.001612
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": false,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": false,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": false,
          "strict_hunter_260n": false,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": false,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.905425,
          "s_seq_knee": 0.0,
          "s_survival_knee": 0.016123
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.31999999999999995,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.40166666666666667,
          "solid_tip_3d": 0.042200000000000015,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.701514,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      },
      "rs_v2_gates": {
        "aesthetic_and_geometric": "pass",
        "fatigue": "fail",
        "floors_828_joint": "fail",
        "gate_s_sharp_law": "fail",
        "gate_s_tip_point_brep": "fail",
        "human_max": "fail",
        "seq_hard": "fail",
        "solid_tip_sf": "fail",
        "strict_hunter_lateral": "fail",
        "twist_floor_body_and_tipsub_strict": "pass"
      }
    },
    "w283_p14_c01": {
      "cad": {
        "boundary_edges": 0,
        "cad_valid": true,
        "closed_shells": 1,
        "facets": 15528,
        "manifold_solids": 1,
        "mass_3d_g": 75.1992,
        "mass_raster_g": 75.382,
        "max_directed_edge_repeat": 1,
        "non_manifold_edges": 0,
        "rel_mass_diff": 0.002425,
        "step_path": "evidence/cert_integrity/recertified/w283_p14_c01/w283_p14_c01.step",
        "step_sha256": "56709b6750cf33511f865a104dd5b74b7a0a145caba5228c1b3340f16cd6ae6f",
        "stl_path": "evidence/cert_integrity/recertified/w283_p14_c01/w283_p14_c01.stl",
        "stl_sha256": "ab25eb8b8d9c399e7e5f7f335b3afd05f5555f8524fe39ee2fa8ed38d93edee6",
        "units": "mm",
        "units_source": "project_convention",
        "watertight_exact": true
      },
      "cad_battery": {
        "cases": [
          {
            "evidence_path": "crates/physics/tests/invariance.rs",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.analytic",
            "kind": "software_contract",
            "solver": "analytic",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "fem_solid/tests/test_gci.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.gci_contracts",
            "kind": "software_contract",
            "solver": "gci",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "expected_unaided_exit": 2,
              "reason": "independent_caller_admission_unavailable"
            },
            "evidence_path": "ops/ci/check_claims.py",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.claims",
            "kind": "software_contract",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "all_within_2pct": true,
              "container_image": "ghcr.io/fenics/dolfinx/dolfinx:v0.9.0",
              "script": "fem_solid/cross_check.py",
              "subjects_checked": [
                "g3",
                "w271b_11772",
                "w271b_11860",
                "w271c_11522",
                "w272_s2902_c01",
                "w273_s2903_c04",
                "w273_s2904_c01",
                "w274_s2905_c01",
                "w275_s2909_c01",
                "w276_s2913_c01",
                "w277_s2919_c01",
                "w278_s2922_c01",
                "w279_s2928_c01",
                "w280_s2931_c01",
                "w281_s2932_c01",
                "w282_p10_c02",
                "w282_p14_c01",
                "w283_p10_c02",
                "w283_p14_c01"
              ],
              "tolerance_rel": 0.02
            },
            "evidence_path": "evidence/cad_battery/dolfinx_cross_check.json",
            "fresh_this_lane": false,
            "geometry_sha256": null,
            "id": "build.dolfinx",
            "kind": "software_contract",
            "solver": "solid_dolfinx",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "geom.unit",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "geom.one_solid",
            "kind": "gate",
            "reason": "no step bound",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "detail": {
              "note": "fem::certify mass, not a STEP",
              "plate_mass_g": 75.38207297021373
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "mass_g": 75.38207297021373
            },
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "geom.mass",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_genome.csv",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "geom.pin",
            "kind": "software_contract",
            "reason": "genome is not a native CAD pin",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "blocked"
          },
          {
            "detail": {
              "loads": {
                "chord_twist_human_max_sf": {
                  "compared_with": 16200,
                  "value_n_or_sf": 0.110986187014051
                },
                "hiker_lateral_n": {
                  "compared_with": 180,
                  "value_n_or_sf": 194.81115243123332
                },
                "hunter_lateral_n": {
                  "compared_with": 260,
                  "value_n_or_sf": 194.81115243123332
                },
                "pry": null,
                "ring_back_sf": {
                  "compared_with": 3000,
                  "value_n_or_sf": 2.859525228086611
                },
                "ring_pull_n": {
                  "compared_with": 1500,
                  "value_n_or_sf": 15491.915610772525
                },
                "spine_sf": {
                  "compared_with": 3427,
                  "value_n_or_sf": 0.431348153525743
                },
                "tip_inplane_sf_record_only": {
                  "compared_with": 1000,
                  "value_n_or_sf": 0.5553764837608903
                }
              },
              "note": "derived from the certify record"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "spg": 2.58432
            },
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "plate.loads",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "seq_ratio": 0.743647399006731,
              "seq_target": 1.2,
              "side_report_seq_ratio": 0.743647399006731
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "seq_ratio": 0.743647399006731
            },
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "plate.seq",
            "kind": "gate",
            "reason": "below 1.2",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "cycles": "1e7",
              "fat_sf": 0.5441708722663069,
              "label": "screening",
              "load": "0..180 N",
              "not": "spec \u00a77.3 L7"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "plate.l7",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "admitted": false,
              "discretization_under_3pct": false,
              "fit_status": "oscillatory",
              "gci_fine": null,
              "p": null,
              "primary_observable": "seq_ratio",
              "refinement_ratio_r": 2.0,
              "sf_hunter_fit_status": "oscillatory",
              "sf_hunter_gci_fine": null,
              "sf_hunter_p": null
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_plate_gci.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "plate.gci",
            "kind": "derived",
            "reason": "oscillatory_sequence",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "side.a1",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "side.a2",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "changes_seq_ratio": false,
              "mesh_mm2": 0.25,
              "sha256": "4cc376fe1bc87ad01b7c8c2e4e2c0414071a8e52d7f0731fff73ec339afb2c72"
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a1a3.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "side.a3",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "bore_support": "radial_pin_bearing_and_flange",
              "buckling_critical_load_n": 1198.0424514916424,
              "buckling_eigenvalue": 4.6078555826601635,
              "certified_seq_ratio": 0.743647399006731,
              "changes_seq_ratio": false,
              "default_clamp": "ring-hole nodes fixed in all 5 dofs inside run_b",
              "grip_foundation_stiffness": "winkler_grip_pad_foundation",
              "partial_solver_edit": false,
              "sha256": "78c627634c97598673724b3634ce1eed80e3e917cad489aed3e5ab3572203694",
              "supported_seq_ratio": 4.550625489280957
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_a4.json",
            "fresh_this_lane": false,
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "side.a4",
            "kind": "derived",
            "solver": "plate_cst_dkt",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "NL": 4,
              "NSUB": 1,
              "SF_nodal_max": 0.9577,
              "copied_from_another_genome": false,
              "d1_sha256": "efbfcb1ddd68f93349f5fe5ae8270599c0adda88efd13618042b39e09afc836b",
              "entered_as_solver_run": true,
              "genome_data_row_matches_csv": true,
              "ksp_its": 130,
              "missing": [],
              "n_dof": 416583,
              "n_tet": 91008,
              "solve_rerun": false
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_solid_tip.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "solid_tip_sf_nodal_max": 0.9577
            },
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "solid.tip",
            "kind": "solver_run",
            "solver": "solid_skfem_p2",
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "pass"
          },
          {
            "detail": {
              "all_hard": false,
              "gates": {
                "aesthetic_and_geometric": "pass",
                "fatigue": "fail",
                "floors_828_joint": "fail",
                "gate_s_sharp_law": "fail",
                "gate_s_tip_point_brep": "fail",
                "human_max": "fail",
                "seq_hard": "fail",
                "solid_tip_sf": "fail",
                "strict_hunter_lateral": "fail",
                "twist_floor_body_and_tipsub_strict": "pass"
              }
            },
            "evidence_path": "evidence/rs_v2/w283_ultrarunner/w283_p14_c01_rs.json",
            "fresh_this_lane": false,
            "g_inputs": {
              "aes_score": 0.4079
            },
            "geometry_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
            "id": "gate.rs_v2",
            "kind": "gate",
            "reason": "does not pass every hard gate",
            "solver": null,
            "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
            "status": "fail"
          }
        ],
        "certificate_identity": "17576c751c5c0157cac7d098fd821915e54e811c5b7b2f9883889a0d0f11f245",
        "global_fitness": {
          "G": 0.623888308291947,
          "inputs": {
            "aes_score": 0.4079,
            "mass_g": 75.38207297021373,
            "seq_ratio": 0.743647399006731,
            "solid_tip_sf_nodal_max": 0.9577,
            "spg": 2.58432
          },
          "metric": "G1",
          "missing": [],
          "provisional": false,
          "status": "COMPLETE",
          "title": "G 0.624 | 75.4 g | jarambit v1.0.0"
        },
        "pass_count": 10,
        "role": "genome",
        "schema": "jarambit.cad_battery.subject.v1",
        "search_fitness": "unchanged",
        "solver_run_count": 1,
        "solver_run_objects": 1,
        "source_commit": "4e54019dd016ddde71b416460b96e0a9b08a95de",
        "subject": "w283_p14_c01"
      },
      "engineering_metrics": {
        "aes_pass": true,
        "aes_score": 0.4079,
        "buckling_eigenvalue_a4": 4.6078555826601635,
        "fat_sf_l7": 0.5441708722663069,
        "lat_rob_cert_n": 194.8,
        "mass_3d_g": 75.1992,
        "mass_g": 75.382,
        "plate_gci_fine": null,
        "promote_eligible": false,
        "ring_bearing_w_mm": 10.199999809265137,
        "ring_rob_cert_n": 15491.9,
        "rs_v2_all_hard": false,
        "rs_v2_screen_plate_pass": false,
        "seq_ratio_cert": 0.744,
        "seq_ratio_supported_a4": 4.550625489280957,
        "sf_hunter_cert": 0.749,
        "sf_hunter_strict": 0.6796,
        "sf_ring_cert": 10.328,
        "solid_tip_dolfinx_rel_err": 0.000313,
        "solid_tip_sf_nodal_avg": 1.0156,
        "solid_tip_sf_nodal_max": 0.9577,
        "spg_n_per_g": 2.58432,
        "strict_lat_rob_0p0625_n": 176.7,
        "t_stock_mm": 5.9,
        "twist_body_sf": 2.982,
        "twist_tip_sf": 0.988,
        "twist_tipzone_sf_cert": 0.988,
        "twist_tipzone_sf_strict": 1.002
      },
      "evaluation_mode": "genome_and_3d_cad_full_battery",
      "genome_csv": "evidence/cert_integrity/recertified/w283_p14_c01/w283_p14_c01_genome.csv",
      "genome_sha256": "aa54fe9148fe303b2054bc5449f063f507d6feffa07556d144013aa9ded5b009",
      "global_fitness_G1": {
        "G": 0.6239029999999999,
        "inputs": {
          "aes_score": 0.4079,
          "mass_g": 75.382,
          "seq_ratio": 0.744,
          "solid_tip_sf_nodal_max": 0.9577,
          "spg": 2.58432
        },
        "metric": "G1",
        "provisional": false,
        "status": "COMPLETE",
        "terms": {
          "aes": 0.4079,
          "mass_pen": 0.0,
          "metric_version": "G1",
          "missing": [],
          "provisional": false,
          "seq_gap": 0.38,
          "spg_n": 0.64608,
          "status": "COMPLETE",
          "tip": 0.9577,
          "tip_missing": false,
          "tip_source": "solid_tip_sf_nodal_max"
        },
        "title": "G 0.624 | 75.4 g | jarambit v1.0.0"
      },
      "input_target": "w283_p14_c01",
      "karambit_archetype": {
        "blade_min_in": 3.436,
        "c6_ring_groove_w_mm": 2.000796830296464,
        "hook_drop_mm": 41.90475205408508,
        "inside_concave_fraction": 1.0,
        "karambit_valid": true,
        "ring_id_mm": 24.9,
        "ring_wall_mm": 5.200796830296465,
        "spine_convex_fraction": 0.9224137931034483,
        "teeth_in": 15,
        "teeth_sp": 14
      },
      "schema": "jarambit.master_cad_scorecard.v1",
      "source_commit": "97388249afbe65a9974fd1b6b9774d0b70da2e9a",
      "subject": "w283_p14_c01",
      "weighted_cad_score": {
        "W_CAD": -3.878704,
        "all_engineering_gates_pass": false,
        "components": {
          "base_feasible": -1.0,
          "carry_stability_3pct": 0.018,
          "karambit_authenticity_12pct": 0.048103,
          "mass_term_70pct": 0.644245,
          "seq_buffer_5pct": 0.0,
          "shortfall_penalty": 2.970738,
          "survival_buffer_10pct": 0.001612
        },
        "gates": {
          "c6_aesthetic_floor": true,
          "cad_watertight_and_single_solid": true,
          "cert_hiker_180n": true,
          "cert_hunter_260n": false,
          "cert_ring_pull_1500n": true,
          "dolfinx_cross_check_2pct": true,
          "failure_sequence_1p2": false,
          "geometric_and_holder_gates": true,
          "karambit_ring_and_curved_claw": true,
          "plate_gci_valid_3level": true,
          "side_a4_buckling_and_support": true,
          "solid_tip_3d_p2_1p82nm": false,
          "strict_hunter_260n": false,
          "twist_body_1p82nm": true,
          "twist_tip_only_1p82nm": false,
          "twist_tip_submodel_strict_1p82nm": true
        },
        "metric": "W_CAD_v2_ultrarunner_karambit",
        "normalized_subscores": {
          "s_carry_knee": 0.6,
          "s_karambit": 0.400862,
          "s_mass": 0.920349,
          "s_seq_knee": 0.0,
          "s_survival_knee": 0.016123
        },
        "shortfalls": {
          "aesthetic": 0.0,
          "cad": 0.0,
          "geometry_gates": 0.0,
          "hunter_strict": 0.32038461538461543,
          "karambit_archetype": 0.0,
          "ring_pull": 0.0,
          "seq_ratio": 0.38,
          "solid_tip_3d": 0.042300000000000004,
          "twist_body": 0.0,
          "twist_tip_strict": 0.0
        },
        "unpenalized_quality_score": 1.71196,
        "weights": {
          "w_carry_stability": 0.03,
          "w_karambit": 0.12,
          "w_mass": 0.7,
          "w_seq_buffer_knee_1p28": 0.05,
          "w_survival_buffer_knee_8pct": 0.1
        }
      },
      "rs_v2_gates": {
        "aesthetic_and_geometric": "pass",
        "fatigue": "fail",
        "floors_828_joint": "fail",
        "gate_s_sharp_law": "fail",
        "gate_s_tip_point_brep": "fail",
        "human_max": "fail",
        "seq_hard": "fail",
        "solid_tip_sf": "fail",
        "strict_hunter_lateral": "fail",
        "twist_floor_body_and_tipsub_strict": "pass"
      }
    }
  },
  "schema": "jarambit.web_studio_bundle.v1",
  "source_commit": "97388249afbe65a9974fd1b6b9774d0b70da2e9a",
  "subjects": [
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 13,
      "fail_count": 0,
      "g1_score": null,
      "g1_status": "INCOMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "c10a",
      "incomplete_count": 0,
      "kind": "native_step",
      "mass_3d_g": 64.8484,
      "mass_g": 64.9498,
      "pass_count": 5,
      "role": "native_cad",
      "solver_run_count": 1,
      "w_cad": -14.12419
    },
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 14,
      "fail_count": 0,
      "g1_score": null,
      "g1_status": "INCOMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "c93",
      "incomplete_count": 0,
      "kind": "native_step",
      "mass_3d_g": null,
      "mass_g": null,
      "pass_count": 4,
      "role": "native_cad",
      "solver_run_count": 0,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 14,
      "fail_count": 0,
      "g1_score": null,
      "g1_status": "INCOMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "c92",
      "incomplete_count": 0,
      "kind": "native_step",
      "mass_3d_g": null,
      "mass_g": null,
      "pass_count": 4,
      "role": "native_cad",
      "solver_run_count": 0,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 14,
      "fail_count": 0,
      "g1_score": null,
      "g1_status": "INCOMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "c10c",
      "incomplete_count": 0,
      "kind": "native_step",
      "mass_3d_g": null,
      "mass_g": null,
      "pass_count": 4,
      "role": "archived_not_on_main",
      "solver_run_count": 0,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 1.0036274369238576,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "g3",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 84.80209229536577,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 5,
      "g1_score": 0.7505798795492734,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w271b_11772",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 78.5515514925079,
      "pass_count": 10,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 5,
      "g1_score": 0.7216545740786019,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w271b_11860",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 73.32648797691759,
      "pass_count": 10,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.7597240123135478,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w271c_11522",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 79.96161512803431,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 5,
      "g1_score": 0.7523889572632168,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w272_s2902_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 79.42608648214917,
      "pass_count": 10,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.7602536724290332,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w273_s2904_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 78.58519741483542,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 5,
      "g1_score": 0.7509316126056635,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w273_s2903_c04",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 79.2991654804125,
      "pass_count": 10,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.7700579410566811,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w274_s2905_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 84.97916178866379,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8015439464550428,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w275_s2909_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 81.77438570899143,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8322600494582232,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w276_s2913_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 79.99479010835532,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8347854224687924,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w277_s2919_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 79.5094905062415,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8618220969105294,
      "g1_status": "COMPLETE",
      "has_scorecard": false,
      "has_stl": false,
      "id": "w278_s2922_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": null,
      "mass_g": 78.57968061789411,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": null
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8737531289757889,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": false,
      "id": "w279_s2928_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 77.4142,
      "mass_g": 77.535,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": 1.72123
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.87764625,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": false,
      "id": "w280_s2931_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 76.8339,
      "mass_g": 76.965,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": 1.743186
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8808251249999999,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "w281_s2932_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 75.6954,
      "mass_g": 75.911,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": 1.79165
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8782097499999999,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "w282_p14_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 75.3233,
      "mass_g": 75.568,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": 1.779046
    },
    {
      "all_engineering_gates_pass": true,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.8811007499999999,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "w282_p10_c02",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 75.6157,
      "mass_g": 75.836,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": 1.797848
    },
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 3,
      "fail_count": 5,
      "g1_score": 0.623888308291947,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "w283_p14_c01",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 75.1992,
      "mass_g": 75.382,
      "pass_count": 10,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": -3.878704
    },
    {
      "all_engineering_gates_pass": false,
      "blocked_count": 3,
      "fail_count": 4,
      "g1_score": 0.6216318603776162,
      "g1_status": "COMPLETE",
      "has_scorecard": true,
      "has_stl": true,
      "id": "w283_p10_c02",
      "incomplete_count": 0,
      "kind": "crescent_genome",
      "mass_3d_g": 75.5049,
      "mass_g": 75.642,
      "pass_count": 11,
      "role": "genome",
      "solver_run_count": 1,
      "w_cad": -3.964924
    }
  ],
  "wave_trajectory": [
    {
      "g1": 0.742,
      "mass_3d_g": 91.28,
      "mass_g": 91.5,
      "seq_ratio": 1.24,
      "sf_hunter_strict": 1.18,
      "solid_tip_sf": 1.38,
      "subject": "w20_baseline",
      "w_cad": 1.192,
      "wave": "w20"
    },
    {
      "g1": 0.785,
      "mass_3d_g": 86.19,
      "mass_g": 86.4,
      "seq_ratio": 1.235,
      "sf_hunter_strict": 1.14,
      "solid_tip_sf": 1.365,
      "subject": "w24_crescent",
      "w_cad": 1.391,
      "wave": "w24"
    },
    {
      "g1": 0.7912,
      "mass_3d_g": 81.91,
      "mass_g": 82.12,
      "seq_ratio": 0.959,
      "sf_hunter_strict": 1.085,
      "solid_tip_sf": 1.351,
      "subject": "g3",
      "w_cad": -1.72,
      "wave": "w27"
    },
    {
      "g1": 0.854,
      "mass_3d_g": 78.44,
      "mass_g": 78.663,
      "seq_ratio": 1.269,
      "sf_hunter_strict": 1.032,
      "solid_tip_sf": 1.416,
      "subject": "w271b_11772",
      "w_cad": 1.662,
      "wave": "w271b"
    },
    {
      "g1": 0.862,
      "mass_3d_g": 77.69,
      "mass_g": 77.912,
      "seq_ratio": 1.279,
      "sf_hunter_strict": 1.028,
      "solid_tip_sf": 1.412,
      "subject": "w272_s2902_c01",
      "w_cad": 1.694,
      "wave": "w272"
    },
    {
      "g1": 0.869,
      "mass_3d_g": 77.088,
      "mass_g": 77.31,
      "seq_ratio": 1.27,
      "sf_hunter_strict": 1.025,
      "solid_tip_sf": 1.411,
      "subject": "w273_s2904_c01",
      "w_cad": 1.718,
      "wave": "w273"
    },
    {
      "g1": 0.873,
      "mass_3d_g": 76.762,
      "mass_g": 76.985,
      "seq_ratio": 1.28,
      "sf_hunter_strict": 1.026,
      "solid_tip_sf": 1.41,
      "subject": "w274_s2905_c01",
      "w_cad": 1.734,
      "wave": "w274"
    },
    {
      "g1": 0.8748,
      "mass_3d_g": 76.556,
      "mass_g": 76.78,
      "seq_ratio": 1.273,
      "sf_hunter_strict": 1.027,
      "solid_tip_sf": 1.41,
      "subject": "w275_s2909_c01",
      "w_cad": 1.743,
      "wave": "w275"
    },
    {
      "g1": 0.8762,
      "mass_3d_g": 76.318,
      "mass_g": 76.54,
      "seq_ratio": 1.271,
      "sf_hunter_strict": 1.026,
      "solid_tip_sf": 1.41,
      "subject": "w276_s2913_c01",
      "w_cad": 1.753,
      "wave": "w276"
    },
    {
      "g1": 0.8771,
      "mass_3d_g": 76.188,
      "mass_g": 76.41,
      "seq_ratio": 1.269,
      "sf_hunter_strict": 1.027,
      "solid_tip_sf": 1.41,
      "subject": "w277_s2919_c01",
      "w_cad": 1.759,
      "wave": "w277"
    },
    {
      "g1": 0.8778,
      "mass_3d_g": 76.098,
      "mass_g": 76.32,
      "seq_ratio": 1.269,
      "sf_hunter_strict": 1.028,
      "solid_tip_sf": 1.41,
      "subject": "w278_s2922_c01",
      "w_cad": 1.763,
      "wave": "w278"
    },
    {
      "g1": 0.8784,
      "mass_3d_g": 76.026,
      "mass_g": 76.245,
      "seq_ratio": 1.269,
      "sf_hunter_strict": 1.028,
      "solid_tip_sf": 1.41,
      "subject": "w279_s2928_c01",
      "w_cad": 1.768,
      "wave": "w279"
    },
    {
      "g1": 0.8798,
      "mass_3d_g": 75.821,
      "mass_g": 76.04,
      "seq_ratio": 1.269,
      "sf_hunter_strict": 1.028,
      "solid_tip_sf": 1.41,
      "subject": "w280_s2931_c01",
      "w_cad": 1.785,
      "wave": "w280"
    },
    {
      "g1": 0.880512,
      "mass_3d_g": 75.695,
      "mass_g": 75.911,
      "seq_ratio": 1.215,
      "sf_hunter_strict": 1.0285,
      "solid_tip_sf": 1.4094,
      "subject": "w281_s2932_c01",
      "w_cad": 1.795131,
      "wave": "w281"
    },
    {
      "g1": 0.881101,
      "mass_3d_g": 75.616,
      "mass_g": 75.836,
      "seq_ratio": 1.215,
      "sf_hunter_strict": 1.0285,
      "solid_tip_sf": 1.4094,
      "subject": "w282_p10_c02",
      "w_cad": 1.797848,
      "wave": "w282 (peer)"
    },
    {
      "g1": 0.87821,
      "mass_3d_g": 75.323,
      "mass_g": 75.568,
      "seq_ratio": 1.201,
      "sf_hunter_strict": 1.01,
      "solid_tip_sf": 1.4093,
      "subject": "w282_p14_c01",
      "w_cad": 1.779046,
      "wave": "w282"
    },
    {
      "g1": 0.621634,
      "mass_3d_g": 75.505,
      "mass_g": 75.642,
      "seq_ratio": 0.718,
      "sf_hunter_strict": 0.68,
      "solid_tip_sf": 0.9578,
      "subject": "w283_p10_c02",
      "w_cad": -3.964924,
      "wave": "w283 (peer, failed)"
    },
    {
      "g1": 0.623903,
      "mass_3d_g": 75.199,
      "mass_g": 75.382,
      "seq_ratio": 0.744,
      "sf_hunter_strict": 0.6796,
      "solid_tip_sf": 0.9577,
      "subject": "w283_p14_c01",
      "w_cad": -3.878704,
      "wave": "w283 (failed)"
    }
  ]
} as unknown as WebStudioBundle;

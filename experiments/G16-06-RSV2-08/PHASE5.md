# G16-06-RSV2-08 Phase 5

Status: complete; 163 trials recorded; 215 five-mesh observations. Counts: {'geometry_legal': 43, 'gci_admitted': 6, 'fine_targets_pass': 0, 'all_five_targets_pass': 0, 'export_pass': 1, 'side_a4_pass': 10, 'scoped_joint_pass': 0}.

Result: **bounded negative joint closure (0 passes)**. All43 legal cases fail certification sequence>=1.20 and strict sequence>=1.25: maxima are1.0764636821846363 and1.2113216813430108, respectively. Actual export passes1/10 (Q7 control only); native A4 passes10/10. Native geometry rejects120/163 inputs, including24 CP08 domain violations. All failed and unselected evidence is retained; no global infeasibility claim.

The complete assigned Q7 CSV and section implementation are snapshotted in phase5_sources/. Only window S0/S1/width/fill, ring_wall and CP06..08 vary. Spine offset42 and every other gene remain parental. Native material, FEM operators/loads/masks, geometry gates, certification, GCI and support constants are unchanged. CPUs12,13, requested nice -n10 prefix (inherited effective nice recorded in receipt), Cargo jobs2, explicit faer.

All accepted geometry trials receive areas4/1/.25/.125/.0625 plus native three-level4/1/.25 GCI. Geometry rejects are recorded and cannot qualify. Full export/A4 selection retains every all-five numerical pass and representative fine-mesh/GCI tradeoffs. Unselected screens have export/A4=null and do not count as joint passes.

Native CP08 is bounded by2.0 despite the assignment saying up to3. Initial all-3 inputs triggered24 identity-guard assertions; the first runner/receipt/log are retained. Those requests are now explicitly classified as domain rejections, with requested and observed genes kept separate. No clamped input is accepted. Complete matching initial native artifacts are reused by hash; additional smaller-step hand probes obey every native bound.

Acceptance requires spine>=1, lateral>=358N, fatigue>=1 and hunter>=1.377 on every mesh; sequence>=1.20 on4/1/.25/.125 and>=1.25 on strict.0625. Actual native exported sharpness/point/bore/census and native A4 must pass. Admission is an estimate, with asymptotic validation not established. No gate or threshold is relaxed.

| Case | Legal | Cert spine | Strict spine | Cert lat | Strict lat | Cert seq | Strict seq | GCI | All-five targets | Export | A4 | Joint |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| w00_r6.3_h0 | True | 1.036124993 | 1.035988896 | 424.556980018 | 424.710434864 | 0.797077464 | 0.899040673 | False | False | None | None | False |
| q7_baseline | True | 1.038248515 | 1.007021689 | 474.635231835 | 396.501963522 | 0.972661735 | 1.211321681 | False | False | True | True | False |
| w00_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w00_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.3_h0 | True | 0.879084366 | 0.848402520 | 424.277435535 | 424.741330328 | 0.797432176 | 0.898938657 | False | False | None | None | False |
| w01_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w01_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.3_h0 | True | 0.645450371 | 0.629151425 | 424.557241393 | 424.899464452 | 0.797077677 | 0.898755196 | False | False | None | None | False |
| w02_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w02_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.3_h0 | True | 0.677865881 | 0.655511179 | 424.464637020 | 424.910565701 | 0.858043435 | 0.898757766 | False | False | None | None | False |
| w03_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w03_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.3_h0 | True | 0.535386008 | 0.539972678 | 424.366175861 | 424.728154408 | 0.857934253 | 0.913971245 | True | False | None | None | False |
| w04_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w04_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.3_h0 | True | 0.752994715 | 0.801101578 | 424.363899736 | 423.404758606 | 0.865614996 | 0.943994855 | False | False | None | None | False |
| w05_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w05_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.3_h0 | True | 0.790405124 | 0.911855057 | 403.193708532 | 424.713127465 | 0.825832947 | 0.869883479 | False | False | None | None | False |
| w06_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w06_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.3_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.3_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.3_h0 | True | 0.862359884 | 0.840909744 | 376.323623401 | 424.713967644 | 0.978870549 | 0.994959396 | False | False | None | None | False |
| w07_r6.3_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.8_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.8_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.8_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r6.8_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r7.0_h0 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r7.0_h1 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r7.0_h2 | False | None | None | None | None | None | None | None | None | None | None | False |
| w07_r7.0_h3 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p3 | True | 1.035580505 | 0.957760344 | 429.144663016 | 437.830347750 | 0.819107086 | 0.831213528 | False | False | None | None | False |
| gentle_w01_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w01_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w00_p5 | True | 1.027105578 | 1.005765047 | 396.610210226 | 426.938514153 | 0.855356539 | 0.795268170 | True | False | None | None | False |
| gentle_w01_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w01_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w01_p3 | True | 0.895430964 | 0.879711459 | 429.144609682 | 437.961049248 | 0.819106954 | 0.831081857 | False | False | None | None | False |
| gentle_w02_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w02_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w01_p5 | True | 0.898012953 | 0.919936071 | 396.537965903 | 426.758818550 | 0.855283102 | 0.795588528 | True | False | False | True | False |
| gentle_w02_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w02_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w02_p3 | True | 0.592055246 | 0.608593281 | 429.144607379 | 437.815500517 | 0.819107212 | 0.837177580 | False | False | None | None | False |
| gentle_w03_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w02_p5 | True | 0.617817000 | 0.633205191 | 396.968691398 | 426.754006024 | 0.854863066 | 0.795486609 | True | False | None | None | False |
| gentle_w03_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w03_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w03_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w03_p3 | True | 0.660761189 | 0.642835161 | 429.144640765 | 437.798150270 | 0.820457255 | 0.898410251 | False | False | None | None | False |
| gentle_w04_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w03_p5 | True | 0.661752315 | 0.646871222 | 396.788954858 | 426.738974265 | 0.812301146 | 0.791152796 | False | False | None | None | False |
| gentle_w04_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w04_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w04_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w04_p3 | True | 0.544579819 | 0.535949232 | 410.904640178 | 437.993076879 | 0.899049423 | 0.989602609 | False | False | None | None | False |
| gentle_w05_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w04_p5 | True | 0.515929016 | 0.568440316 | 397.520552640 | 426.758266921 | 0.866898251 | 0.899583167 | False | False | None | None | False |
| gentle_w05_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w05_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w05_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w05_p3 | True | 0.742364998 | 0.810665732 | 378.172049710 | 409.829908259 | 0.939153672 | 0.958643786 | False | False | False | True | False |
| gentle_w06_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w06_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w05_p5 | True | 0.753845553 | 0.789447636 | 413.403620313 | 398.198122953 | 0.911881348 | 1.072050535 | False | False | None | None | False |
| gentle_w06_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w06_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w06_p3 | True | 0.811595688 | 0.842376575 | 409.978725041 | 437.840731050 | 0.893856719 | 0.992940890 | False | False | None | None | False |
| gentle_w07_p0 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w07_p1 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w06_p5 | True | 0.779819970 | 0.825409664 | 388.867063095 | 426.753746106 | 0.941691473 | 0.883225477 | False | False | None | None | False |
| gentle_w07_p2 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w07_p4 | False | None | None | None | None | None | None | None | None | None | None | False |
| gentle_w07_p3 | True | 0.821112215 | 0.822616672 | 383.845031752 | 426.738955046 | 1.056704826 | 1.026015598 | False | False | False | True | False |
| gentle_w07_p5 | True | 0.879444696 | 0.749154742 | 406.661219254 | 426.734969794 | 1.076463682 | 0.962780786 | True | False | False | True | False |
| local0_g101_plus | True | 0.796844863 | 0.837676242 | 415.624492491 | 437.840933288 | 1.032529491 | 0.961553733 | False | False | None | None | False |
| local0_g100_plus | True | 0.760527963 | 0.854497882 | 419.081080659 | 427.434174466 | 1.005394261 | 1.024347054 | False | False | None | None | False |
| local0_g102_minus | True | 0.860964238 | 0.916210460 | 359.176910918 | 437.841140440 | 0.944066135 | 0.951052385 | False | False | False | True | False |
| local0_g102_plus | True | 0.809623984 | 0.865427809 | 405.950484885 | 420.465412722 | 1.057135107 | 1.041324479 | False | False | False | True | False |
| local0_g103_plus | True | 0.822274246 | 0.763162035 | 429.144472529 | 437.840863195 | 0.921399191 | 0.987018176 | False | False | None | None | False |
| local0_g103_minus | True | 0.811689615 | 0.918870651 | 416.531473981 | 437.435979825 | 1.030281190 | 1.000926934 | False | False | False | True | False |
| local1_g100_plus | True | 0.872103650 | 0.889729171 | 374.883015215 | 407.072865851 | 0.851961459 | 1.043336082 | False | False | None | None | False |
| local1_g101_plus | True | 0.832947693 | 0.881285663 | 373.647228333 | 384.315274769 | 0.915398792 | 1.105117937 | False | False | None | None | False |
| local1_g102_minus | True | 0.808059552 | 0.871216817 | 379.943601669 | 424.713880443 | 0.970160168 | 0.924063238 | False | False | None | None | False |
| local1_g102_plus | True | 0.862219724 | 0.825358606 | 376.338696934 | 381.067952780 | 0.970277358 | 1.114534978 | False | False | None | None | False |
| local1_g103_plus | True | 0.679086214 | 0.848609848 | 424.253048610 | 424.713065769 | 0.990972934 | 0.979657410 | False | False | None | None | False |
| local1_g103_minus | True | 0.822358356 | 0.854056006 | 400.343237375 | 424.865907018 | 0.965278981 | 0.920676660 | False | False | None | None | False |
| local2_g101_plus | True | 0.849670954 | 0.813729781 | 430.671499657 | 414.465458869 | 1.058897579 | 1.030038535 | False | False | False | True | False |
| local2_g100_plus | True | 0.758516083 | 0.818051401 | 401.023146403 | 426.735080401 | 1.047969593 | 0.990681289 | False | False | None | None | False |
| local2_g102_plus | True | 0.767897194 | 0.766912470 | 427.093084206 | 397.187462031 | 1.019788650 | 1.074391893 | False | False | None | None | False |
| local2_g102_minus | True | 0.791079605 | 0.839668900 | 406.078904520 | 420.486690377 | 0.969031890 | 1.014859876 | False | False | None | None | False |
| local2_g103_minus | True | 0.843774119 | 0.863000235 | 395.563911042 | 375.377986138 | 1.054687086 | 1.136865276 | True | False | False | True | False |
| local2_g103_plus | True | 0.757419700 | 0.832488972 | 395.948729580 | 425.001016232 | 1.062798071 | 1.004124211 | False | False | None | None | False |

Baseline reproduction: {'max_relative_scalar_error': 1.2215497689225644e-13, 'owner_bound': 1e-09, 'pass_': True}.

Export/A4 selection: [{'name': 'q7_baseline', 'reason': 'exact Q7 control'}, {'name': 'local2_g103_minus', 'reason': 'top fine_score / admitted filter=False'}, {'name': 'gentle_w07_p3', 'reason': 'top fine_score / admitted filter=False'}, {'name': 'local2_g101_plus', 'reason': 'top fine_score / admitted filter=False'}, {'name': 'local0_g102_plus', 'reason': 'top fine_score / admitted filter=False'}, {'name': 'local0_g103_minus', 'reason': 'top all_five_score / admitted filter=False'}, {'name': 'local0_g102_minus', 'reason': 'top all_five_score / admitted filter=False'}, {'name': 'gentle_w05_p3', 'reason': 'top all_five_score / admitted filter=False'}, {'name': 'gentle_w07_p5', 'reason': 'top fine_score / admitted filter=True'}, {'name': 'gentle_w01_p5', 'reason': 'top fine_score / admitted filter=True'}].

This bounded parameter study does not establish global infeasibility or an optimum. Full RS-v2/manufacturing qualification remains false. Native human-max twist and independent solid/cross-solver/manufacturing constraints are not waived. Raw measurements, canonical inputs, peak coordinates, mesh hashes and rejected geometry remain in target/G16-06-RSV2-08/phase5/.

Original invocation:
```sh
nice -n 10 taskset -c 12,13 env CARGO_BUILD_JOBS=2 python3 experiments/G16-06-RSV2-08/run_phase5.py
```

## Verified per-mesh measurements

Integrity audit PASS: {'status': 'pass', 'cases': 163, 'legal_five_mesh_cases': 43, 'mesh_observations': 215, 'full_exports_and_a4': 10, 'native_requirement_tests_passed': 1, 'native_gci_tests_passed': 2, 'parent_and_canonical_identity': True, 'only_assigned_genes_changed': True, 'native_fem_identity': True, 'source_artifact_hashes': True, 'gci_a4_mesh_identity': True, 'matched_export_fem_inputs': True, 'fatigue_identity': True, 'thresholds_unchanged': True}.

| Case | Area mm² | Spine SF | Lateral N | Fatigue SF | Hunter SF | Sequence | Spine peak x,y |
| --- | --- | --- | --- | --- | --- | --- | --- |
| w00_r6.3_h0 | 4.0 | 0.891773894 | 407.262234164 | 1.137615801 | 1.566393208 | 0.822004610 | [108.16976925178784, -12.088768196992806] |
| w00_r6.3_h0 | 1.0 | 1.061612398 | 447.940740931 | 1.251244093 | 1.722849004 | 0.842136984 | [120.13080211204635, -14.17059087962764] |
| w00_r6.3_h0 | 0.25 | 1.036124993 | 443.672518653 | 1.239321561 | 1.706432764 | 0.812572823 | [100.51466426636259, -12.322975014499171] |
| w00_r6.3_h0 | 0.125 | 1.067836429 | 424.556980018 | 1.185925648 | 1.632911462 | 0.797077464 | [164.46113359480137, -16.492902628845357] |
| w00_r6.3_h0 | 0.0625 | 1.035988896 | 424.710434864 | 1.186354297 | 1.633501673 | 0.899040673 | [147.73244581179222, -4.661934259127262] |
| q7_baseline | 4.0 | 0.904029182 | 480.804945428 | 1.343044499 | 1.849249790 | 0.994364630 | [113.53110603669126, -12.486231742226517] |
| q7_baseline | 1.0 | 0.728459841 | 484.343339813 | 1.352928384 | 1.862858999 | 0.924589549 | [113.53110603669126, -12.486231742226517] |
| q7_baseline | 0.25 | 1.038248515 | 474.635231835 | 1.325810483 | 1.825520122 | 1.003791887 | [100.51466426636259, -12.322975014499171] |
| q7_baseline | 0.125 | 1.063024520 | 479.695733997 | 1.339946111 | 1.844983592 | 0.972661735 | [107.83060263944975, -12.004190305608311] |
| q7_baseline | 0.0625 | 1.007021689 | 396.501963522 | 1.107558867 | 1.525007552 | 1.211321681 | [107.85295710791556, -12.002552918124557] |
| w01_r6.3_h0 | 4.0 | 0.864087535 | 407.262246131 | 1.137615834 | 1.566393254 | 0.822004185 | [164.38705826118974, -16.424884879175085] |
| w01_r6.3_h0 | 1.0 | 0.894025096 | 447.940742757 | 1.251244098 | 1.722849011 | 0.842136706 | [164.38705826118974, -16.424884879175085] |
| w01_r6.3_h0 | 0.25 | 0.890113180 | 443.672579084 | 1.239321730 | 1.706432996 | 0.812573101 | [164.44261476139846, -16.475898191427788] |
| w01_r6.3_h0 | 0.125 | 0.879084366 | 424.277435535 | 1.185144789 | 1.631836291 | 0.797432176 | [164.46113359480137, -16.492902628845357] |
| w01_r6.3_h0 | 0.0625 | 0.848402520 | 424.741330328 | 1.186440598 | 1.633620501 | 0.898938657 | [164.47039301150284, -16.50140484755414] |
| w02_r6.3_h0 | 4.0 | 0.778737191 | 407.262245519 | 1.137615833 | 1.566393252 | 0.822004197 | [164.38705826118974, -16.424884879175085] |
| w02_r6.3_h0 | 1.0 | 0.736393258 | 447.940728840 | 1.251244059 | 1.722848957 | 0.842137400 | [164.49817126160718, -16.52691150368049] |
| w02_r6.3_h0 | 0.25 | 0.645450371 | 443.672546295 | 1.239321638 | 1.706432870 | 0.812573039 | [164.44261476139846, -16.475898191427788] |
| w02_r6.3_h0 | 0.125 | 0.669714515 | 424.557241393 | 1.185926378 | 1.632912467 | 0.797077677 | [164.46113359480137, -16.492902628845357] |
| w02_r6.3_h0 | 0.0625 | 0.629151425 | 424.899464452 | 1.186882318 | 1.634228709 | 0.898755196 | [164.47039301150284, -16.50140484755414] |
| w03_r6.3_h0 | 4.0 | 0.568804184 | 407.262503028 | 1.137616552 | 1.566394242 | 0.832545670 | [147.25031321745394, -22.961688312491106] |
| w03_r6.3_h0 | 1.0 | 0.650176502 | 447.940743982 | 1.251244101 | 1.722849015 | 0.842110820 | [149.00840675865214, -6.047099673819465] |
| w03_r6.3_h0 | 0.25 | 0.711187097 | 443.672371954 | 1.239321151 | 1.706432200 | 0.858043435 | [148.94097614309345, -6.013743291272332] |
| w03_r6.3_h0 | 0.125 | 0.677865881 | 424.464637020 | 1.185667704 | 1.632556296 | 0.878849258 | [151.07690752921414, -7.409683658796578] |
| w03_r6.3_h0 | 0.0625 | 0.655511179 | 424.910565701 | 1.186913328 | 1.634271407 | 0.898757766 | [151.08932994261198, -7.421057224288393] |
| w04_r6.3_h0 | 4.0 | 0.523530393 | 407.262247454 | 1.137615838 | 1.566393259 | 0.823135542 | [153.25253433034936, -8.858045928866478] |
| w04_r6.3_h0 | 1.0 | 0.506077644 | 447.940748733 | 1.251244115 | 1.722849034 | 0.843400160 | [147.25031321745394, -22.961688312491106] |
| w04_r6.3_h0 | 0.25 | 0.614786731 | 443.672461469 | 1.239321401 | 1.706432544 | 0.857934253 | [153.30988561953845, -8.907050588413925] |
| w04_r6.3_h0 | 0.125 | 0.535386008 | 424.366175861 | 1.185392670 | 1.632177599 | 0.865756149 | [153.36723690872753, -8.956055247961373] |
| w04_r6.3_h0 | 0.0625 | 0.539972678 | 424.728154408 | 1.186403794 | 1.633569825 | 0.913971245 | [153.33856126413298, -8.931552918187649] |
| w05_r6.3_h0 | 4.0 | 0.910369495 | 407.262474907 | 1.137616473 | 1.566394134 | 0.949070073 | [129.22008171084696, -16.39382254336293] |
| w05_r6.3_h0 | 1.0 | 0.870194975 | 442.344193131 | 1.235611116 | 1.701323820 | 1.012651905 | [147.25031321745394, -22.961688312491106] |
| w05_r6.3_h0 | 0.25 | 0.752994715 | 443.672527358 | 1.239321585 | 1.706432798 | 0.956298197 | [138.90155790496908, -19.356948003787245] |
| w05_r6.3_h0 | 0.125 | 0.782897523 | 424.363899736 | 1.185386312 | 1.632168845 | 0.865614996 | [138.5120343775876, -19.40781183650096] |
| w05_r6.3_h0 | 0.0625 | 0.801101578 | 423.404758606 | 1.182707119 | 1.628479841 | 0.943994855 | [138.3181574196401, -19.33593638240425] |
| w06_r6.3_h0 | 4.0 | 0.890438192 | 407.262229519 | 1.137615788 | 1.566393190 | 0.861499389 | [118.12333726113872, -13.77286375118333] |
| w06_r6.3_h0 | 1.0 | 1.012727283 | 447.940830425 | 1.251244343 | 1.722849348 | 0.989639950 | [119.95543653668683, -13.533818896480524] |
| w06_r6.3_h0 | 0.25 | 0.790405124 | 443.672482811 | 1.239321461 | 1.706432626 | 0.877502900 | [119.85882372008427, -13.742478614296878] |
| w06_r6.3_h0 | 0.125 | 0.868897000 | 403.193708532 | 1.126251086 | 1.550745033 | 0.825832947 | [119.69078174134478, -13.942976652402372] |
| w06_r6.3_h0 | 0.0625 | 0.911855057 | 424.713127465 | 1.186361818 | 1.633512029 | 0.869883479 | [119.89645801771164, -14.028311898812014] |
| w07_r6.3_h0 | 4.0 | 0.745212529 | 407.262288975 | 1.137615954 | 1.566393419 | 0.877252752 | [128.54200022591553, -16.209765251659576] |
| w07_r6.3_h0 | 1.0 | 0.999319644 | 429.178984488 | 1.198836409 | 1.650688402 | 1.043715322 | [119.05297385006828, -13.753432055046883] |
| w07_r6.3_h0 | 0.25 | 0.862359884 | 443.672479397 | 1.239321451 | 1.706432613 | 0.978870549 | [118.90084564798681, -13.745430822381389] |
| w07_r6.3_h0 | 0.125 | 0.877788695 | 376.323623401 | 1.051194204 | 1.447398552 | 1.010952114 | [128.89238235322372, -16.261327095415236] |
| w07_r6.3_h0 | 0.0625 | 0.840909744 | 424.713967644 | 1.186364165 | 1.633515260 | 0.994959396 | [118.97690974902756, -13.749431438714137] |
| gentle_w00_p3 | 4.0 | 0.874507176 | 491.151281839 | 1.371945179 | 1.889043392 | 0.986249941 | [129.28003638515514, -16.522334651955855] |
| gentle_w00_p3 | 1.0 | 1.113482608 | 384.729430347 | 1.074674356 | 1.479728578 | 0.816953474 | [145.47702117105075, -3.234883404976661] |
| gentle_w00_p3 | 0.25 | 1.035580505 | 429.144663016 | 1.198740538 | 1.650556396 | 0.819107086 | [107.7829360472671, -11.996434686352984] |
| gentle_w00_p3 | 0.125 | 1.077336586 | 457.951894366 | 1.279208499 | 1.761353440 | 0.966806122 | [145.44917484155832, -3.2490587612132833] |
| gentle_w00_p3 | 0.0625 | 0.957760344 | 437.830347750 | 1.223002478 | 1.683962876 | 0.831213528 | [107.84998923202673, -11.991615574750924] |
| gentle_w00_p5 | 4.0 | 1.162100023 | 392.820483313 | 1.097275297 | 1.510848013 | 0.649796191 | [113.58204976735053, -12.494694220169805] |
| gentle_w00_p5 | 1.0 | 1.049098485 | 468.858049063 | 1.309672934 | 1.803300189 | 0.873225393 | [113.58204976735053, -12.494694220169805] |
| gentle_w00_p5 | 0.25 | 1.027105578 | 465.598432362 | 1.300567765 | 1.790763201 | 0.917463687 | [113.95135638036001, -12.28810602207152] |
| gentle_w00_p5 | 0.125 | 1.098245212 | 396.610210226 | 1.107861236 | 1.525423885 | 0.855356539 | [107.84710063568336, -12.070946410945407] |
| gentle_w00_p5 | 0.0625 | 1.005765047 | 426.938514153 | 1.192578047 | 1.642071208 | 0.795268170 | [107.93599857843148, -12.064115473823453] |
| gentle_w01_p3 | 4.0 | 0.873853585 | 491.151287452 | 1.371945195 | 1.889043413 | 0.986251718 | [129.28003638515514, -16.522334651955855] |
| gentle_w01_p3 | 1.0 | 1.020094188 | 384.729438754 | 1.074674379 | 1.479728611 | 0.817642906 | [164.49875007418402, -16.527305083243363] |
| gentle_w01_p3 | 0.25 | 0.985184607 | 429.144609682 | 1.198740389 | 1.650556191 | 0.819106954 | [164.44321140685707, -16.476268083587662] |
| gentle_w01_p3 | 0.125 | 0.895430964 | 457.650684930 | 1.278367123 | 1.760194942 | 0.967237029 | [164.46172429596606, -16.49328041680623] |
| gentle_w01_p3 | 0.0625 | 0.879711459 | 437.961049248 | 1.223367570 | 1.684465574 | 0.831081857 | [164.47098074052053, -16.501786583415512] |
| gentle_w01_p5 | 4.0 | 0.794740902 | 392.820471632 | 1.097275264 | 1.510847968 | 0.649795935 | [164.39246497732103, -16.4279233694199] |
| gentle_w01_p5 | 1.0 | 0.956157551 | 468.858060378 | 1.309672965 | 1.803300232 | 0.873224735 | [164.39246497732103, -16.4279233694199] |
| gentle_w01_p5 | 0.25 | 0.971494500 | 466.069741642 | 1.301884285 | 1.792575929 | 0.916899495 | [164.44786460340015, -16.479145059961994] |
| gentle_w01_p5 | 0.125 | 0.898012953 | 396.537965903 | 1.107659434 | 1.525146023 | 0.855283102 | [164.46633114542652, -16.496218956809354] |
| gentle_w01_p5 | 0.0625 | 0.919936071 | 426.758818550 | 1.192076099 | 1.641380071 | 0.795588528 | [164.47556441643968, -16.504755905233036] |
| gentle_w02_p3 | 4.0 | 0.731775904 | 491.151270021 | 1.371945146 | 1.889043346 | 0.986249128 | [164.3876727395301, -16.425231083931962] |
| gentle_w02_p3 | 1.0 | 0.721678077 | 384.729419743 | 1.074674326 | 1.479728537 | 0.816466105 | [164.49875007418402, -16.527305083243363] |
| gentle_w02_p3 | 0.25 | 0.670413643 | 429.144607379 | 1.198740382 | 1.650556182 | 0.819107212 | [164.44321140685707, -16.476268083587662] |
| gentle_w02_p3 | 0.125 | 0.592055246 | 457.951791514 | 1.279208211 | 1.761353044 | 0.966806030 | [164.49875007418402, -16.527305083243363] |
| gentle_w02_p3 | 0.0625 | 0.608593281 | 437.815500517 | 1.222961005 | 1.683905771 | 0.837177580 | [164.49875007418402, -16.527305083243363] |
| gentle_w02_p5 | 4.0 | 0.762603866 | 392.820477392 | 1.097275280 | 1.510847990 | 0.659311649 | [164.39246497732103, -16.4279233694199] |
| gentle_w02_p5 | 1.0 | 0.681838305 | 468.858073543 | 1.309673002 | 1.803300283 | 0.873225228 | [164.50326422947924, -16.530366750504083] |
| gentle_w02_p5 | 0.25 | 0.658838390 | 465.598367182 | 1.300567583 | 1.790762951 | 0.917464200 | [164.44786460340015, -16.479145059961994] |
| gentle_w02_p5 | 0.125 | 0.617817000 | 396.968691398 | 1.108862590 | 1.526802659 | 0.854863066 | [164.46633114542652, -16.496218956809354] |
| gentle_w02_p5 | 0.0625 | 0.633205191 | 426.754006024 | 1.192062656 | 1.641361562 | 0.795486609 | [164.47556441643968, -16.504755905233036] |
| gentle_w03_p3 | 4.0 | 0.566702461 | 488.479018104 | 1.364480678 | 1.878765454 | 1.005470597 | [147.25588552733004, -22.951189357646392] |
| gentle_w03_p3 | 1.0 | 0.607452099 | 384.729344789 | 1.074674117 | 1.479728249 | 0.814443811 | [146.5841195368231, -22.849836965517547] |
| gentle_w03_p3 | 0.25 | 0.679100639 | 429.144640765 | 1.198740475 | 1.650556311 | 0.820457255 | [151.21923033372525, -7.497880052810986] |
| gentle_w03_p3 | 0.125 | 0.660761189 | 457.853169638 | 1.278932728 | 1.760973729 | 0.928807068 | [151.077699257753, -7.410192398013972] |
| gentle_w03_p3 | 0.0625 | 0.642835161 | 437.798150270 | 1.222912540 | 1.683839040 | 0.898410251 | [151.09011955376388, -7.421568717095186] |
| gentle_w03_p5 | 4.0 | 0.687433509 | 392.820502684 | 1.097275351 | 1.510848087 | 0.649925386 | [148.88077809978213, -5.98503865599389] |
| gentle_w03_p5 | 1.0 | 0.738069019 | 423.069792795 | 1.181771450 | 1.627191511 | 1.108228660 | [149.01549433827742, -6.051943326105814] |
| gentle_w03_p5 | 0.25 | 0.704168238 | 465.598281634 | 1.300567344 | 1.790762622 | 0.812301146 | [151.2252322124304, -7.5020637388434235] |
| gentle_w03_p5 | 0.125 | 0.661752315 | 396.788954858 | 1.108360527 | 1.526111365 | 0.867536997 | [151.08386977972845, -7.414155388979868] |
| gentle_w03_p5 | 0.0625 | 0.646871222 | 426.738974265 | 1.192020667 | 1.641303747 | 0.791152796 | [151.09627357124327, -7.425553171084587] |
| gentle_w04_p3 | 4.0 | 0.524765105 | 491.151303396 | 1.371945240 | 1.889043475 | 0.996700543 | [153.25330244278305, -8.858522615672427] |
| gentle_w04_p3 | 1.0 | 0.599881981 | 384.729414395 | 1.074674311 | 1.479728517 | 0.997068254 | [150.97833688966625, -7.3191818453642625] |
| gentle_w04_p3 | 0.25 | 0.544579819 | 410.904640178 | 1.147790225 | 1.580402462 | 1.044389678 | [147.76652988640842, -22.81864946755269] |
| gentle_w04_p3 | 0.125 | 0.584456318 | 457.952037318 | 1.279208898 | 1.761353990 | 0.899049423 | [153.32975652944742, -8.923880741501083] |
| gentle_w04_p3 | 0.0625 | 0.535949232 | 437.993076879 | 1.223457034 | 1.684588757 | 0.989602609 | [153.33931329028047, -8.932050507229665] |
| gentle_w04_p5 | 4.0 | 0.595110684 | 392.820475594 | 1.097275275 | 1.510847983 | 0.898216274 | [153.25928932448855, -8.86223526492346] |
| gentle_w04_p5 | 1.0 | 0.523523926 | 468.858061803 | 1.309672969 | 1.803300238 | 0.873212704 | [146.58151888953984, -22.870144798058142] |
| gentle_w04_p5 | 0.25 | 0.571012144 | 465.598341804 | 1.300567512 | 1.790762853 | 0.929050291 | [147.3531240584427, -22.76428675167402] |
| gentle_w04_p5 | 0.125 | 0.515929016 | 397.520552640 | 1.110404118 | 1.528925202 | 0.866898251 | [153.37380328282717, -8.960489673035948] |
| gentle_w04_p5 | 0.0625 | 0.568440316 | 426.758266921 | 1.192074558 | 1.641377950 | 0.899583167 | [153.3451747932425, -8.935926071007826] |
| gentle_w05_p3 | 4.0 | 0.800510005 | 491.151289858 | 1.371945202 | 1.889043423 | 0.987242287 | [138.1408350662822, -19.31525746180698] |
| gentle_w05_p3 | 1.0 | 0.783993639 | 366.095391095 | 1.022623427 | 1.408059197 | 1.050899174 | [146.5841195368231, -22.849836965517547] |
| gentle_w05_p3 | 0.25 | 0.742364998 | 429.144596767 | 1.198740353 | 1.650556141 | 0.939153672 | [138.515574073503, -19.15975791540042] |
| gentle_w05_p3 | 0.125 | 0.824430227 | 378.172049710 | 1.056357460 | 1.454507884 | 1.209893335 | [138.59225109612424, -19.4573576216329] |
| gentle_w05_p3 | 0.0625 | 0.810665732 | 409.829908259 | 1.144788149 | 1.576268878 | 0.958643786 | [138.40015095038552, -19.380034651818946] |
| gentle_w05_p5 | 4.0 | 0.709738123 | 392.820459425 | 1.097275230 | 1.510847921 | 0.701652899 | [138.16181064446653, -19.30764794568919] |
| gentle_w05_p5 | 1.0 | 0.667383424 | 403.783229924 | 1.127897811 | 1.553012423 | 1.161162836 | [138.16181064446653, -19.30764794568919] |
| gentle_w05_p5 | 0.25 | 0.753845553 | 465.598300976 | 1.300567398 | 1.790762696 | 0.911881348 | [138.33455209286583, -19.35097248646384] |
| gentle_w05_p5 | 0.125 | 0.817250767 | 413.403620313 | 1.154770689 | 1.590013924 | 0.940710611 | [138.39213257566558, -19.365414000055384] |
| gentle_w05_p5 | 0.0625 | 0.789447636 | 398.198122953 | 1.112296792 | 1.531531242 | 1.072050535 | [144.1912310400241, -3.082704109975916] |
| gentle_w06_p3 | 4.0 | 0.807480200 | 461.151724900 | 1.288146665 | 1.773660480 | 1.065053049 | [118.0662486727953, -13.814143286946997] |
| gentle_w06_p3 | 1.0 | 0.828911292 | 370.624962529 | 1.035275992 | 1.425480625 | 1.038054258 | [129.28003638515514, -16.522334651955855] |
| gentle_w06_p3 | 0.25 | 0.811595688 | 409.978725041 | 1.145203843 | 1.576841250 | 1.046748043 | [128.70647590129875, -16.02724235183053] |
| gentle_w06_p3 | 0.125 | 0.897308261 | 457.657318917 | 1.278385654 | 1.760220457 | 0.893856719 | [113.74297306149435, -12.496457056965355] |
| gentle_w06_p3 | 0.0625 | 0.842376575 | 437.840731050 | 1.223031482 | 1.684002812 | 0.992940890 | [118.87233503168703, -13.784245680282195] |
| gentle_w06_p5 | 4.0 | 0.955050933 | 392.820563758 | 1.097275521 | 1.510848322 | 0.905614471 | [118.18183007734916, -13.747899536504248] |
| gentle_w06_p5 | 1.0 | 0.784072366 | 460.365813142 | 1.285951358 | 1.770637743 | 1.018446700 | [113.58204976735053, -12.494694220169805] |
| gentle_w06_p5 | 0.25 | 0.779819970 | 388.867063095 | 1.086232109 | 1.495642550 | 0.996799164 | [129.1146900181932, -16.044254818220356] |
| gentle_w06_p5 | 0.125 | 0.881765366 | 456.252991033 | 1.274462909 | 1.754819196 | 0.941691473 | [129.10827772251116, -16.3439738204245] |
| gentle_w06_p5 | 0.0625 | 0.825409664 | 426.753746106 | 1.192061930 | 1.641360562 | 0.883225477 | [119.82736467797007, -13.957133931267016] |
| gentle_w07_p3 | 4.0 | 0.726167616 | 467.838098169 | 1.306823879 | 1.799377301 | 1.049831027 | [129.28003638515514, -16.522334651955855] |
| gentle_w07_p3 | 1.0 | 0.596330782 | 384.729420689 | 1.074674329 | 1.479728541 | 0.849312015 | [113.53218393315974, -12.487528273870058] |
| gentle_w07_p3 | 0.25 | 0.821112215 | 383.845031752 | 1.072203943 | 1.476327045 | 1.118015011 | [128.75682141313672, -16.347262092539662] |
| gentle_w07_p3 | 0.125 | 0.876650203 | 433.098566755 | 1.209785076 | 1.665763718 | 1.056704826 | [113.74297306149435, -12.496457056965355] |
| gentle_w07_p3 | 0.0625 | 0.822616672 | 426.738955046 | 1.192020613 | 1.641303673 | 1.026015598 | [118.87233503168703, -13.784245680282195] |
| gentle_w07_p5 | 4.0 | 0.583299832 | 392.820533377 | 1.097275436 | 1.510848205 | 0.850150030 | [128.65253083919598, -16.23430769971703] |
| gentle_w07_p5 | 1.0 | 0.755412581 | 468.858071178 | 1.309672996 | 1.803300274 | 0.983703090 | [113.58204976735053, -12.494694220169805] |
| gentle_w07_p5 | 0.25 | 0.879444696 | 432.525630643 | 1.208184679 | 1.663560118 | 1.076463682 | [128.82630899607614, -16.265325122549175] |
| gentle_w07_p5 | 0.125 | 0.887153209 | 406.661219254 | 1.135936971 | 1.564081613 | 1.121417776 | [119.6993793876199, -13.920130448069472] |
| gentle_w07_p5 | 0.0625 | 0.749154742 | 426.734969794 | 1.192009481 | 1.641288345 | 0.962780786 | [119.05945293586903, -13.735113032081742] |
| local0_g101_plus | 4.0 | 0.870952292 | 432.649719341 | 1.208531299 | 1.664037382 | 1.135216369 | [119.33381998565764, -13.89097809870017] |
| local0_g101_plus | 1.0 | 0.577158699 | 374.528936063 | 1.046181059 | 1.440495908 | 1.027235487 | [113.53218393315974, -12.487528273870058] |
| local0_g101_plus | 0.25 | 0.796844863 | 415.624492491 | 1.160974307 | 1.598555740 | 1.032529491 | [119.52622001023389, -13.943254734656973] |
| local0_g101_plus | 0.125 | 0.817771876 | 441.797703217 | 1.234084592 | 1.699221935 | 1.035898285 | [113.84836762566167, -12.500921448513004] |
| local0_g101_plus | 0.0625 | 0.837676242 | 437.840933288 | 1.223032047 | 1.684003590 | 0.961553733 | [113.84836762566167, -12.500921448513004] |
| local0_g100_plus | 4.0 | 0.741968006 | 491.151226787 | 1.371945026 | 1.889043180 | 0.990176966 | [113.70809310500039, -12.073596597494308] |
| local0_g100_plus | 1.0 | 0.595547139 | 384.729319320 | 1.074674045 | 1.479728151 | 0.948052467 | [129.28003638515514, -16.522334651955855] |
| local0_g100_plus | 0.25 | 0.760527963 | 419.081080659 | 1.170629681 | 1.611850310 | 1.024013041 | [113.69027577941071, -12.494224861191531] |
| local0_g100_plus | 0.125 | 0.884092376 | 455.202062491 | 1.271527324 | 1.750777163 | 1.005394261 | [119.077286619556, -13.821275917424433] |
| local0_g100_plus | 0.0625 | 0.854497882 | 427.434174466 | 1.193962587 | 1.643977594 | 1.024347054 | [119.622420022522, -13.969393052635375] |
| local0_g102_minus | 4.0 | 0.815234659 | 452.713954016 | 1.264577229 | 1.741207515 | 1.084903489 | [129.28003638515514, -16.522334651955855] |
| local0_g102_minus | 1.0 | 0.814480386 | 366.211127361 | 1.022946716 | 1.408504336 | 1.050567060 | [118.0662486727953, -13.814143286946997] |
| local0_g102_minus | 0.25 | 0.860964238 | 429.144570586 | 1.198740279 | 1.650556041 | 0.944066135 | [128.75682141313672, -16.347262092539662] |
| local0_g102_minus | 0.125 | 0.916279105 | 359.176910918 | 1.003297863 | 1.381449657 | 1.120675543 | [119.59035335175929, -13.960680279975907] |
| local0_g102_minus | 0.0625 | 0.916210460 | 437.841140440 | 1.223032626 | 1.684004386 | 0.951052385 | [119.622420022522, -13.969393052635375] |
| local0_g102_plus | 4.0 | 0.765589063 | 441.016203949 | 1.231901610 | 1.696216169 | 1.113680077 | [118.0662486727953, -13.814143286946997] |
| local0_g102_plus | 1.0 | 0.731167514 | 384.729354419 | 1.074674143 | 1.479728286 | 0.881973446 | [118.0662486727953, -13.814143286946997] |
| local0_g102_plus | 0.25 | 0.809623984 | 405.950484885 | 1.133951659 | 1.561348019 | 1.057135107 | [128.75682141313672, -16.347262092539662] |
| local0_g102_plus | 0.125 | 0.870666149 | 412.570775764 | 1.152444283 | 1.586810676 | 1.109281786 | [119.18167449202807, -13.690547844942838] |
| local0_g102_plus | 0.0625 | 0.865427809 | 420.465412722 | 1.174496570 | 1.617174664 | 1.041324479 | [119.622420022522, -13.969393052635375] |
| local0_g103_plus | 4.0 | 0.789779643 | 430.140149823 | 1.201521256 | 1.654385192 | 1.141839570 | [118.14281792061112, -13.445187753471437] |
| local0_g103_plus | 1.0 | 0.750068167 | 384.728996023 | 1.074673142 | 1.479726908 | 0.908903692 | [118.0662486727953, -13.814143286946997] |
| local0_g103_plus | 0.25 | 0.854649979 | 429.144472529 | 1.198740006 | 1.650555664 | 0.921399191 | [119.52622001023389, -13.943254734656973] |
| local0_g103_plus | 0.125 | 0.822274246 | 438.269103500 | 1.224228066 | 1.685650398 | 1.044238289 | [113.84836762566167, -12.500921448513004] |
| local0_g103_plus | 0.0625 | 0.763162035 | 437.840863195 | 1.223031851 | 1.684003320 | 0.987018176 | [119.71862003481012, -13.995531370613776] |
| local0_g103_minus | 4.0 | 0.878824272 | 491.151222809 | 1.371945014 | 1.889043165 | 0.971548478 | [118.14281792061112, -13.445187753471437] |
| local0_g103_minus | 1.0 | 0.775080584 | 384.729144674 | 1.074673558 | 1.479727480 | 0.970724138 | [118.0662486727953, -13.814143286946997] |
| local0_g103_minus | 0.25 | 0.811689615 | 416.531473981 | 1.163507801 | 1.602044131 | 1.030281190 | [114.24593359450611, -12.390102794616524] |
| local0_g103_minus | 0.125 | 0.898686717 | 437.745044096 | 1.222764198 | 1.683634785 | 1.045488682 | [119.59035335175929, -13.960680279975907] |
| local0_g103_minus | 0.0625 | 0.918870651 | 437.435979825 | 1.221900881 | 1.682446076 | 1.000926934 | [118.87233503168703, -13.784245680282195] |
| local1_g100_plus | 4.0 | 0.599573584 | 407.262237553 | 1.137615810 | 1.566393221 | 0.872450528 | [128.54200022591553, -16.209765251659576] |
| local1_g100_plus | 1.0 | 0.568608114 | 447.940714745 | 1.251244020 | 1.722848903 | 0.889950268 | [118.74871744590533, -13.737429589715894] |
| local1_g100_plus | 0.25 | 0.872103650 | 442.064937949 | 1.234831065 | 1.700249761 | 1.003636357 | [114.43800061518935, -12.666932877242651] |
| local1_g100_plus | 0.125 | 0.884816912 | 374.883015215 | 1.047170118 | 1.441857751 | 0.851961459 | [119.56796586380386, -13.728231495961044] |
| local1_g100_plus | 0.0625 | 0.889729171 | 407.072865851 | 1.137086834 | 1.565664869 | 1.043336082 | [118.97690974902756, -13.749431438714137] |
| local1_g101_plus | 4.0 | 0.831146318 | 407.262232947 | 1.137615798 | 1.566393204 | 0.863530881 | [118.12333726113872, -13.77286375118333] |
| local1_g101_plus | 1.0 | 0.655960367 | 437.080522020 | 1.220907971 | 1.681078931 | 1.024847038 | [113.53110603669126, -12.486231742226517] |
| local1_g101_plus | 0.25 | 0.832947693 | 443.672471240 | 1.239321428 | 1.706432582 | 0.915398792 | [118.90084564798681, -13.745430822381389] |
| local1_g101_plus | 0.125 | 0.884321948 | 373.647228333 | 1.043718163 | 1.437104724 | 0.925134326 | [118.95155504868063, -13.748097899936553] |
| local1_g101_plus | 0.0625 | 0.881285663 | 384.315274769 | 1.073517484 | 1.478135672 | 1.105117937 | [118.97690974902756, -13.749431438714137] |
| local1_g102_minus | 4.0 | 0.857863070 | 403.734663330 | 1.127762149 | 1.552825628 | 1.008737286 | [112.87796851490492, -12.537599151944823] |
| local1_g102_minus | 1.0 | 0.707164119 | 428.095088505 | 1.195808735 | 1.646519571 | 1.046357922 | [120.13080211204635, -14.17059087962764] |
| local1_g102_minus | 0.25 | 0.808059552 | 443.672483785 | 1.239321463 | 1.706432630 | 0.970160168 | [118.87772612803033, -13.238069345948048] |
| local1_g102_minus | 0.125 | 0.865114495 | 379.943601669 | 1.061305980 | 1.461321545 | 1.116620947 | [118.95155504868063, -13.748097899936553] |
| local1_g102_minus | 0.0625 | 0.871216817 | 424.713880443 | 1.186363922 | 1.633514925 | 0.924063238 | [128.89238235322372, -16.261327095415236] |
| local1_g102_plus | 4.0 | 0.718385355 | 373.752256549 | 1.044011541 | 1.437508679 | 1.089658209 | [119.05297385006828, -13.753432055046883] |
| local1_g102_plus | 1.0 | 0.978177887 | 419.615038929 | 1.172121200 | 1.613903996 | 1.067503859 | [128.54200022591553, -16.209765251659576] |
| local1_g102_plus | 0.25 | 0.862219724 | 427.535965014 | 1.194246921 | 1.644369096 | 1.037742975 | [113.6889961273753, -12.491883763280274] |
| local1_g102_plus | 0.125 | 0.877668738 | 376.338696934 | 1.051236310 | 1.447456527 | 0.970277358 | [113.74162615760332, -12.493767770298192] |
| local1_g102_plus | 0.0625 | 0.825358606 | 381.067952780 | 1.064446659 | 1.465645972 | 1.114534978 | [119.7226721359086, -13.952453882270147] |
| local1_g103_plus | 4.0 | 0.756778517 | 403.580369795 | 1.127331157 | 1.552232192 | 1.009122859 | [118.12333726113872, -13.77286375118333] |
| local1_g103_plus | 1.0 | 0.945346698 | 403.589052256 | 1.127355410 | 1.552265586 | 1.109892460 | [128.54200022591553, -16.209765251659576] |
| local1_g103_plus | 0.25 | 0.679086214 | 439.716179502 | 1.228270219 | 1.691216075 | 1.008997358 | [128.36680916226143, -16.183984329781747] |
| local1_g103_plus | 0.125 | 0.850360489 | 424.253048610 | 1.185076669 | 1.631742495 | 0.990972934 | [119.18053542832358, -13.791340974517981] |
| local1_g103_plus | 0.0625 | 0.848609848 | 424.713065769 | 1.186361646 | 1.633511791 | 0.979657410 | [118.97690974902756, -13.749431438714137] |
| local1_g103_minus | 4.0 | 0.711319746 | 407.262269048 | 1.137615898 | 1.566393342 | 0.859296373 | [113.53110603669126, -12.486231742226517] |
| local1_g103_minus | 1.0 | 0.851206176 | 447.940726932 | 1.251244054 | 1.722848950 | 0.987786688 | [127.81758694044899, -16.19590242983476] |
| local1_g103_minus | 0.25 | 0.840331190 | 443.672478495 | 1.239321449 | 1.706432610 | 0.965278981 | [114.43800061518935, -12.666932877242651] |
| local1_g103_minus | 0.125 | 0.822358356 | 400.343237375 | 1.118288794 | 1.539781682 | 0.992681768 | [119.05297385006828, -13.753432055046883] |
| local1_g103_minus | 0.0625 | 0.854056006 | 424.865907018 | 1.186788581 | 1.634099642 | 0.920676660 | [119.05297385006828, -13.753432055046883] |
| local2_g101_plus | 4.0 | 0.807412228 | 392.820097812 | 1.097274220 | 1.510846530 | 0.833008261 | [128.65253083919598, -16.23430769971703] |
| local2_g101_plus | 1.0 | 0.835308833 | 382.757388385 | 1.069165801 | 1.472143801 | 1.224948358 | [118.18183007734916, -13.747899536504248] |
| local2_g101_plus | 0.25 | 0.876296343 | 433.417364592 | 1.210675582 | 1.666989864 | 1.074249091 | [119.05945293586903, -13.735113032081742] |
| local2_g101_plus | 0.125 | 0.849670954 | 430.671499657 | 1.203005489 | 1.656428845 | 1.058897579 | [118.95769739472576, -13.730574160067542] |
| local2_g101_plus | 0.0625 | 0.813729781 | 414.465458869 | 1.157736749 | 1.594097919 | 1.030038535 | [119.62150816547756, -13.505500583784034] |
| local2_g100_plus | 4.0 | 0.615863288 | 392.820367832 | 1.097274974 | 1.510847569 | 0.840696516 | [128.65253083919598, -16.23430769971703] |
| local2_g100_plus | 1.0 | 0.686819493 | 335.740575584 | 0.937832560 | 1.291309906 | 1.396489189 | [128.65253083919598, -16.23430769971703] |
| local2_g100_plus | 0.25 | 0.758516083 | 444.285913159 | 1.241034971 | 1.708791974 | 1.047969593 | [118.99125885669821, -13.54961139137388] |
| local2_g100_plus | 0.125 | 0.852657705 | 401.023146403 | 1.120188001 | 1.542396717 | 1.137183690 | [118.95769739472576, -13.730574160067542] |
| local2_g100_plus | 0.0625 | 0.818051401 | 426.735080401 | 1.192009790 | 1.641288771 | 0.990681289 | [119.82736467797007, -13.957133931267016] |
| local2_g102_plus | 4.0 | 0.561959307 | 392.820396776 | 1.097275055 | 1.510847680 | 0.979832301 | [118.75418631243923, -13.721496416039143] |
| local2_g102_plus | 1.0 | 0.786039844 | 356.096642874 | 0.994693673 | 1.369602473 | 1.316659605 | [113.58204976735053, -12.494694220169805] |
| local2_g102_plus | 0.25 | 0.870425187 | 427.093084206 | 1.193009811 | 1.642665708 | 1.090156130 | [119.14474603840988, -13.406320764919363] |
| local2_g102_plus | 0.125 | 0.767897194 | 447.187837419 | 1.249140989 | 1.719953221 | 1.019788650 | [118.95769739472576, -13.730574160067542] |
| local2_g102_plus | 0.0625 | 0.766912470 | 397.187462031 | 1.109473688 | 1.527644085 | 1.074391893 | [119.82736467797007, -13.957133931267016] |
| local2_g102_minus | 4.0 | 0.586620926 | 392.820103125 | 1.097274235 | 1.510846550 | 0.843412328 | [118.75418631243923, -13.721496416039143] |
| local2_g102_minus | 1.0 | 0.710949764 | 468.858026875 | 1.309672872 | 1.803300103 | 0.875738692 | [128.65253083919598, -16.23430769971703] |
| local2_g102_minus | 0.25 | 0.791079605 | 406.078904520 | 1.134310377 | 1.561841940 | 1.146570786 | [119.05945293586903, -13.735113032081742] |
| local2_g102_minus | 0.125 | 0.864658791 | 456.037214139 | 1.273860174 | 1.753989285 | 0.969031890 | [114.0793491968929, -12.549857447461566] |
| local2_g102_minus | 0.0625 | 0.839668900 | 420.486690377 | 1.174556006 | 1.617256501 | 1.014859876 | [118.98313628001158, -13.731708878071093] |
| local2_g103_minus | 4.0 | 0.847038028 | 392.820326090 | 1.097274857 | 1.510847408 | 0.759939153 | [118.18183007734916, -13.747899536504248] |
| local2_g103_minus | 1.0 | 0.925197337 | 465.536727214 | 1.300395402 | 1.790525874 | 1.007134429 | [118.44891968900941, -13.707879799996544] |
| local2_g103_minus | 0.25 | 0.897453382 | 395.563911042 | 1.104938582 | 1.521399658 | 1.177049228 | [114.5039041051915, -12.378122087831116] |
| local2_g103_minus | 0.125 | 0.843774119 | 432.390945235 | 1.207808459 | 1.663042097 | 1.054687086 | [119.82736467797007, -13.957133931267016] |
| local2_g103_minus | 0.0625 | 0.863000235 | 375.377986138 | 1.048552732 | 1.443761485 | 1.136865276 | [113.94882655940685, -12.509377900248733] |
| local2_g103_plus | 4.0 | 0.624752542 | 392.820579088 | 1.097275564 | 1.510848381 | 0.905176946 | [118.92278678191224, -13.3703420467446] |
| local2_g103_plus | 1.0 | 0.797887709 | 468.858007422 | 1.309672817 | 1.803300029 | 0.956164687 | [113.58204976735053, -12.494694220169805] |
| local2_g103_plus | 0.25 | 0.757419700 | 438.086993970 | 1.223719375 | 1.684949977 | 1.062798071 | [114.3225125995896, -12.342471534340405] |
| local2_g103_plus | 0.125 | 0.813035558 | 395.948729580 | 1.106013505 | 1.522879729 | 1.151757773 | [119.6993793876199, -13.920130448069472] |
| local2_g103_plus | 0.0625 | 0.832488972 | 425.001016232 | 1.187165985 | 1.634619293 | 1.004124211 | [114.0467185375214, -12.539737560658356] |

| Export/A4 case | GCI fit / p | Half-angle ° | Apex mm | Point | Bore mm | Census | A4 eigenvalue | Supported / cert seq |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| q7_baseline | oscillatory / None | 24.324224296 | 0.195144636 | True | 1.600000000 | [0, 0, 1] | 10.366098143 | 2.942390034 / 0.972661735 |
| gentle_w01_p5 | admissible_estimate / 2.35494345438922 | 89.596611450 | 0.195142117 | True | 1.575000000 | [0, 0, 1] | 12.139721541 | 4.048930763 / 0.855283102 |
| gentle_w05_p3 | oscillatory / None | 26.833950452 | 0.195135284 | True | 1.575000000 | [0, 0, 1] | 10.918024182 | 4.007615129 / 0.939153672 |
| gentle_w07_p3 | oscillatory / None | 26.833950452 | 0.195135284 | True | 1.575000000 | [0, 0, 1] | 9.844538481 | 2.999379215 / 1.056704826 |
| gentle_w07_p5 | admissible_estimate / 0.5258291127900484 | 89.596611450 | 0.195142117 | True | 1.575000000 | [0, 0, 1] | 9.117642308 | 2.649174268 / 1.076463682 |
| local0_g102_minus | nonpositive_order / -1.6330540577130794 | 26.833950452 | 0.195135284 | True | 1.575000000 | [0, 0, 1] | 9.844590148 | 2.594312350 / 0.944066135 |
| local0_g102_plus | oscillatory / None | 26.833950452 | 0.195135284 | True | 1.575000000 | [0, 0, 1] | 9.844520387 | 2.700495705 / 1.057135107 |
| local0_g103_minus | oscillatory / None | 26.833950452 | 0.195135284 | True | 1.575000000 | [0, 0, 1] | 8.602527934 | 2.731763463 / 1.030281190 |
| local2_g101_plus | oscillatory / None | 89.596611450 | 0.195142117 | True | 1.575000000 | [0, 0, 1] | 11.285586653 | 2.661260088 / 1.058897579 |
| local2_g103_minus | admissible_estimate / 0.540839651997014 | 89.596611450 | 0.195142117 | True | 1.575000000 | [0, 0, 1] | 9.117811591 | 2.574256096 / 1.054687086 |

Geometry rejects (retained, never accepted):
- w00_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w00_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w00_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w00_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w00_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w00_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w00_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w00_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w00_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w00_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w00_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w01_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w01_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w01_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w01_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w01_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w01_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w02_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w02_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w02_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w02_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w02_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w03_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w03_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w03_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w03_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w03_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w03_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w03_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w03_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w03_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w03_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w03_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w04_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w04_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w04_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w04_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w04_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w04_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w05_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w05_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w05_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w05_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w05_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w06_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w06_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w06_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w06_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w06_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w06_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w06_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w06_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w06_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w06_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w06_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r6.3_h1: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r6.3_h2: []; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w07_r6.3_h3: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r6.8_h1: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r6.8_h0: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; []
- w07_r6.8_h2: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w07_r6.8_h3: ['GripOutOfBand 59.8', 'Holder envelope: best oriented box 200.0 x 98.8 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r7.0_h0: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; []
- w07_r7.0_h1: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- w07_r7.0_h2: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.87 < C.4 1.00-0.03)']
- w07_r7.0_h3: ['GripOutOfBand 59.6', 'Holder envelope: best oriented box 200.2 x 99.0 > 200 x 100']; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w00_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w00_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w00_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w00_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w01_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w01_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w01_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w01_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w02_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w02_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w02_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w02_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w03_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w03_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w03_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w03_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w04_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w04_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w04_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w04_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w05_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w05_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w05_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w05_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w06_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w06_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w06_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w06_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']
- gentle_w07_p0: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w07_p1: []; ['hawkbill edge not continuously concave (0.97 < C.4 1.00-0.03)']
- gentle_w07_p2: []; ['hawkbill edge not continuously concave (0.96 < C.4 1.00-0.03)']
- gentle_w07_p4: []; ['hawkbill edge not continuously concave (0.94 < C.4 1.00-0.03)']

Delivery: WIP commit with required p5 Agent-Session/Agent-Model/Agent-Task trailers was attempted; staging failed because shared Git index.lock is read-only. Snapshot HEAD `ca3ce9dd802d3c0bb282751ac0b6552f617671fe` contains complete matching Phase-5 data=False; no agent-created commit is claimed. Complete report, receipt, source snapshots,163 exact CSVs and audit remain local for captain checkpointing. Source/raw archives and manifest live under target/G16-06-RSV2-08/. Lane COMMS HANDOFF is recorded under shared flock; shared main COMMS requires captain relay. No PR/push/stacking/deletes; await captain's next milestone.

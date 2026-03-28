struct RmmFeatures {
  pub max_ipa_width: UInt64,
  pub feat_lpa2: RmmFeature,
  pub feat_sve: RmmFeature,
  pub max_sve_vl: UInt64,
  pub num_bps: UInt64,
  pub num_wps: UInt64,
  pub feat_pmu: RmmFeature,
  pub pmu_num_ctrs: UInt64,
  pub feat_sha_256: RmmFeature,
  pub feat_sha_512: RmmFeature,
  pub max_recs_order: UInt64,
}

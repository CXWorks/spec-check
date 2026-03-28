struct RmmRealm {
  pub feat_lpa2: RmmFeature,
  pub ipa_width: UInt8,
  pub measurements: [RmmRealmMeasurement; 5],
  pub hash_algo: RmmHashAlgorithm,
  pub rec_index: UInt64,
  pub rtt_base: Address,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt64,
  pub state: RmmRealmState,
  pub vmid: UInt16,
  pub rpv: [UInt64; 8],
  pub num_recs: UInt64,
}

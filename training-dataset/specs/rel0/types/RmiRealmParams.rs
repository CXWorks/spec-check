struct RmiRealmParams {
  pub flags: RmiRealmFlags,
  pub s2sz: UInt8,
  pub sve_vl: UInt8,
  pub num_bps: UInt8,
  pub num_wps: UInt8,
  pub pmu_num_ctrs: UInt8,
  pub hash_algo: RmiHashAlgorithm,
  pub rpv: [UInt64; 8],
  pub vmid: UInt16,
  pub rtt_base: Address,
  pub rtt_level_start: Int64,
  pub rtt_num_start: UInt32,
}

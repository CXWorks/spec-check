struct RsiRealmConfig {
  pub ipa_width: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub num_aux_planes: UInt64,
  pub gicv3_vtr: UInt64,
  pub rpv: [UInt64; 8],
}

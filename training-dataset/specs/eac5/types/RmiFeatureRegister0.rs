struct RmiFeatureRegister0 {
  pub S2SZ: UInt8,
  pub LPA2: RmiFeature,
  pub SVE_EN: RmiFeature,
  pub SVE_VL: UInt4,
  pub NUM_BPS: UInt4,
  pub NUM_WPS: UInt4,
  pub PMU_EN: RmiFeature,
  pub PMU_NUM_CTRS: UInt5,
  pub HASH_SHA_256: RmiFeature,
  pub HASH_SHA_512: RmiFeature,
}

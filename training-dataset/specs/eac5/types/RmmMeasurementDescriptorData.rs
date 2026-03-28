struct RmmMeasurementDescriptorData {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub ipa: Address,
  pub flags: RmmDataFlags,
  pub content: RmmRealmMeasurement,
}

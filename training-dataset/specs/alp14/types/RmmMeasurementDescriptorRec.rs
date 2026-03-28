struct RmmMeasurementDescriptorRec {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub content: RmmRealmMeasurement,
}

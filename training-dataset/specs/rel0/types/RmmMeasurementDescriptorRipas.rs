struct RmmMeasurementDescriptorRipas {
  pub desc_type: UInt8,
  pub len: UInt64,
  pub rim: RmmRealmMeasurement,
  pub base: Address,
  pub top: Address,
}

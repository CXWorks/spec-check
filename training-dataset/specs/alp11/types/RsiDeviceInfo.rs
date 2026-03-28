struct RsiDeviceInfo {
  pub attest_type: RsiDeviceAttestationType,
  pub cert_id: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub cert_digest: [UInt64; 8],
  pub meas_digest: [UInt64; 8],
  pub report_digest: [UInt64; 8],
}

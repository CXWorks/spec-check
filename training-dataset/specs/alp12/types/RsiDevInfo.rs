struct RsiDevInfo {
  pub flags: RsiDevFlags,
  pub attest_type: RsiDevAttestType,
  pub cert_id: UInt64,
  pub hash_algo: RsiHashAlgorithm,
  pub cert_digest: [UInt64; 8],
  pub meas_digest: [UInt64; 8],
  pub report_digest: [UInt64; 8],
}

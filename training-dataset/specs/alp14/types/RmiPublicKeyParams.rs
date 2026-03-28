struct RmiPublicKeyParams {
  pub key_len: UInt64,
  pub metadata_len: UInt64,
  pub algo: RmiSignatureAlgorithm,
  pub key: [UInt8; 1024],
  pub metadata: [UInt8; 1024],
}

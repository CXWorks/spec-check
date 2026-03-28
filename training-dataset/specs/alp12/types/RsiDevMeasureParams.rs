struct RsiDevMeasureParams {
  pub flags: RsiDevMeasureFlags,
  pub indices: [UInt64; 4],
  pub nonce: [UInt64; 4],
}

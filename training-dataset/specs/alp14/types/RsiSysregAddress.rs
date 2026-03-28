struct RsiSysregAddress {
  pub Op2: UInt3,
  pub CRm: UInt4,
  pub CRn: UInt4,
  pub Op1: UInt3,
  pub Op0: UInt2,
  pub d128: RsiBoolean,
}

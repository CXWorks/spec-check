struct RsiPlaneEnter {
  pub flags: RsiPlaneEnterFlags,
  pub pc: UInt64,
  pub gicv3_hcr: UInt64,
  pub gprs: [UInt64; 31],
  pub gicv3_lrs: [UInt64; 16],
}

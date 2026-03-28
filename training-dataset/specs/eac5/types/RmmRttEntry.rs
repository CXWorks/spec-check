struct RmmRttEntry {
  pub addr: Address,
  pub ripas: RmmRipas,
  pub state: RmmRttEntryState,
  pub MemAttr: UInt3,
  pub S2AP: UInt2,
  pub SH: UInt2,
}

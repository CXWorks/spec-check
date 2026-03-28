struct RmmRttEntry {
  pub addr: Address,
  pub ripas: RmmRipas,
  pub state: RmmRttEntryState,
  pub attr_prot: RmmRttMemAttr,
  pub attr_unprot: UInt3,
  pub sh: RmmRttShareability,
  pub s2ap_direct: RmmRttS2APDirect,
  pub s2ap_indirect: RmmRttS2APIndirect,
}

struct RmiVsmmuParams {
  pub flags: RmiVsmmuFlags,
  pub reg_base: Address,
  pub reg_top: Address,
  pub aidr: UInt64,
  pub idr: [UInt64; 7],
}

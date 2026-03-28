struct RsiPlaneEnterFlags {
  pub trap_wfi: RsiTrap,
  pub trap_wfe: RsiTrap,
  pub trap_hc: RsiTrap,
  pub gic_owner: RsiGicOwner,
}

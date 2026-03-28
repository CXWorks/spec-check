struct RmiRecEnterFlags {
  pub emul_mmio: RmiEmulatedMmio,
  pub inject_sea: RmiInjectSea,
  pub trap_wfi: RmiTrap,
  pub trap_wfe: RmiTrap,
  pub ripas_response: RmiResponse,
  pub s2ap_response: RmiResponse,
  pub dev_mem_response: RmiResponse,
}

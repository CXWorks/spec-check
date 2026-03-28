struct RmiPdevFlags {
  pub spdm: RmiPdevSpdm,
  pub ncoh_ide: RmiPdevIde,
  pub ncoh_addr: RmiFeature,
  pub coh_ide: RmiPdevIde,
  pub coh_addr: RmiFeature,
  pub p2p: RmiFeature,
}

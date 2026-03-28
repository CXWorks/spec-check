struct RmmVdev {
  pub vdev_id: UInt64,
  pub tdi_id: UInt64,
  pub inst_id: UInt64,
  pub pdev: Address,
  pub realm: Address,
  pub state: RmmVdevState,
  pub comm_state: RmmDevCommState,
  pub aux: [Address; 32],
  pub num_aux: UInt64,
  pub vsmmu: RmmFeature,
  pub vsmmu_addr: Address,
  pub vsid: UInt64,
}

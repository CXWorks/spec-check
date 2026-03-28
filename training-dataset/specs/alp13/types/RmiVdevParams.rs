struct RmiVdevParams {
  pub flags: RmiVdevFlags,
  pub vdev_id: UInt64,
  pub tdi_id: UInt64,
  pub num_aux: UInt64,
  pub vsmmu_addr: Address,
  pub vsid: UInt64,
  pub aux: [Address; 32],
}

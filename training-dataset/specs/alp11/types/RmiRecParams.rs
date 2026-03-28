struct RmiRecParams {
  pub flags: RmiRecCreateFlags,
  pub mpidr: RmiRecMpidr,
  pub pc: UInt64,
  pub num_aux: UInt64,
  pub gprs: [UInt64; 8],
  pub aux: [Address; 16],
}

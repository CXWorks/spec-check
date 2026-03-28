struct RmiDevCommEnter {
  pub status: RmiDevCommStatus,
  pub req_addr: Address,
  pub resp_addr: Address,
  pub resp_len: UInt64,
}

struct RmiDevCommExit {
  pub flags: RmiDevCommExitFlags,
  pub cache_offset: UInt64,
  pub cache_len: UInt64,
  pub protocol: RmiDevCommProtocol,
  pub req_len: UInt64,
}

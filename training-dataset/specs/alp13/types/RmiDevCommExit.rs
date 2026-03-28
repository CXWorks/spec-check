struct RmiDevCommExit {
  pub flags: RmiDevCommExitFlags,
  pub cache_req_offset: UInt64,
  pub cache_req_len: UInt64,
  pub cache_rsp_offset: UInt64,
  pub cache_rsp_len: UInt64,
  pub cache_obj_id: RmiDevCommObject,
  pub protocol: RmiDevCommProtocol,
  pub req_len: UInt64,
  pub timeout: UInt64,
}

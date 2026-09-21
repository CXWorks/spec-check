pub open spec fn 3.10.3.12_powercap_cai_set_spec(domain_id: UInt32, flags: UInt32, cai: UInt32, cpli: UInt32, result: Result<int, ScmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SCMI_SUCCESS ==> cai != 0)
  && ((!(result == SCMI_SUCCESS))
    ==> cai == 0)
}
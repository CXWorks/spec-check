pub open spec fn 3.5.6.19_performance_domain_name_get_spec(domain_id: UInt32, result: int32, flags: UInt32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
  (result == SCMI_SUCCESS ==> flags == 0)
  && (result != SCMI_SUCCESS ==> flags == 0)
  && (result == SCMI_SUCCESS ==> name[0] == 0)
  && ((!(result == SCMI_SUCCESS) &&
       !(result == SCMI_NOT_FOUND))
    ==> result == SCMI_SUCCESS)
}
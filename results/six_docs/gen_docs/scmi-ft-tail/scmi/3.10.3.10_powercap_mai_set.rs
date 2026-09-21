pub open spec fn 3.10.3.10_powercap_mai_set_spec(domain_id: UInt32, flags: UInt32, mai: UInt32, result: int, old_s: S, new_s: S) -> bool {
  (result != SCMI_SUCCESS ==> flags == 0)
  && (result == SCMI_SUCCESS ==> mai != 0)
  && ((!(result == SCMI_SUCCESS) &&
       result != SCMI_NOT_FOUND &&
       result != SCMI_NOT_SUPPORTED &&
       result != SCMI_INVALID_PARAMETERS &&
       result != SCMI_DENIED)
    ==> flags == 0)
}
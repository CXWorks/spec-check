pub open spec fn 3.10.3.9_powercap_mai_get_spec(result: int32, mai: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> mai == old_s.powercap_mai)
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_NOT_SUPPORTED ==> true)
}
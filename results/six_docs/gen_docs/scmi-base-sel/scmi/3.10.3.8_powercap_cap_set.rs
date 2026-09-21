pub open spec fn 3.10.3.8_powercap_cap_set_spec(result: int, old_s: S, new_s: S, domain_id: UInt32, cpli: UInt32, flags: UInt32, power_cap: UInt32) -> bool {
    (result == SCMI_SUCCESS)
    || (result == SCMI_NOT_FOUND)
    || (result == SCMI_NOT_SUPPORTED)
    || (result == SCMI_INVALID_PARAMETERS)
    || (result == SCMI_DENIED)
}
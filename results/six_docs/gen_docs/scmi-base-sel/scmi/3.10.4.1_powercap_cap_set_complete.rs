pub open spec fn 3.10.4.1_powercap_cap_set_complete_spec(result: int, domain_id: UInt32, power_cap: UInt32, cpli: UInt32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> true)
    && (result != SCMI_SUCCESS ==> true)
}
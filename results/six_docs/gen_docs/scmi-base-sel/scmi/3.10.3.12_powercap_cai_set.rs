pub open spec fn 3.10.3.12_powercap_cai_set_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (old_s.domain_id == new_s.domain_id && old_s.cai == new_s.cai && old_s.flags == new_s.flags && old_s.cpli == new_s.cpli))
    && (result == SCMI_NOT_FOUND ==> (old_s.domain_id != new_s.domain_id || old_s.cpli != new_s.cpli))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags != 0 || new_s.cai == 0))
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_GENERIC_ERROR ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
    && (result == SCMI_COMMS_ERROR ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_IN_USE ==> true)
}
pub open spec fn 3_10_3_11_powercap_cai_get_spec(result: int32, cai: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> cai >= 0)
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_BUSY ==> true)
    && (result == SCMI_COMMS_ERROR ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_GENERIC_ERROR ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_IN_USE ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
}
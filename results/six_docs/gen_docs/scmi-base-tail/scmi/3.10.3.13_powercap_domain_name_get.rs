pub open spec fn 3.10.3.13_powercap_domain_name_get_spec(result: int32, flags: UInt32, name: [u8; 64], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> flags == 0 && name[0] == 0)
    && (result == SCMI_NOT_FOUND ==> flags == 0 && name[0] == 0)
    && (result != SCMI_SUCCESS ==> flags == 0 && name[0] == 0)
    && (result != SCMI_SUCCESS ==> result == SCMI_INVALID_PARAMETERS || result == SCMI_NOT_FOUND || result == SCMI_HARDWARE_ERROR || result == SCMI_PROTOCOL_ERROR || result == SCMI_OUT_OF_RANGE || result == SCMI_PARTIAL_ERROR || result == SCMI_DENIED || result == SCMI_IN_USE || result == SCMI_NOT_SUPPORTED || result == SCMI_GENERIC_ERROR || result == SCMI_BUSY || result == SCMI_COMMS_ERROR)
}
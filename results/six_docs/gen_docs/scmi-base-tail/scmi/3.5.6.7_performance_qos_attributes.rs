pub open spec fn 3.5.6.7_performance_qos_attributes_spec(result: (int32, uint32, [uint8; 16]), old_s: S, new_s: S) -> bool {
    (result.0 == SCMI_INVALID_PARAMETERS ==> (result.1 == 0 && result.2[0] == 0))
    && (result.0 == SCMI_NOT_FOUND ==> (result.1 == 0 && result.2[0] == 0))
    && (result.0 == SCMI_SUCCESS ==> (result.1 != 0 || result.2[0] != 0))
    && true
}
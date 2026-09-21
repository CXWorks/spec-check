pub open spec fn 3.5.6.7_performance_qos_attributes_spec(result: int, qos_attribute_1: uint32, name: [uint8; 16], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (qos_attribute_1 == old_s.performance_qos_attributes[old_s.domain_id as int][old_s.capability as int].attribute_1 && name == old_s.performance_qos_attributes[old_s.domain_id as int][old_s.capability as int].name))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_INVALID_PARAMETERS ==> true)
}
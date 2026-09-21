pub open spec fn performance_qos_config_complete_spec(
    status: int,
    domain_id: UInt32,
    capability: UInt32,
    flags: UInt32,
    qos_value: UInt32,
    old_s: S,
    new_s: S
) -> bool {
    (status == SCMI_SUCCESS)
    && (flags == 0)
    && (qos_value == 0)
}
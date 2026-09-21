pub open spec fn 3.5.7.1_performance_qos_config_complete_spec(
    result: int,
    old_s: S,
    new_s: S,
    domain_id: uint32,
    capability: uint32,
    flags: uint32,
    qos_value: uint32,
) -> bool {
    (result == SCMI_SUCCESS)
    && (flags & 0x1F == 0)
    && (flags & 0x10 == 0)
    && (flags & 0x08 == 0)
    && (flags & 0x04 == 0)
    && (flags & 0x02 == 0)
    && (flags & 0x01 == 0)
}
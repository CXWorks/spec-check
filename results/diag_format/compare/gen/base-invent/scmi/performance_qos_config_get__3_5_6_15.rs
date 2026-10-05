pub open spec fn performance_qos_config_get__3_5_6_15_spec(result: int32, qos_value: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (domain_id(old_s) != valid_domain_id(old_s) || capability(old_s) != valid_capability(old_s)))
    && (result == INVALID_PARAMETERS ==> (multiple_type_bits_set(old_s) || multiple_subtype_bits_set(old_s)))
    && (result == SUCCESS ==> (qos_value(new_s) == qos_value(old_s)))
}
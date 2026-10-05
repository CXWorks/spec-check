pub open spec fn voltage_config_get__3_9_2_8_spec(result: int32, config: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> domain_id_not_valid(old_s, domain_id))
    && (result == NOT_SUPPORTED ==> request_not_supported(old_s))
    && (result == DENIED ==> agent_not_allowed(old_s, domain_id))
    && (result == SUCCESS ==> (config_bits_3_0_mode_valid(config) && config_bits_31_4_zero(config)))
}

fn domain_id_not_valid(old_s: S, domain_id: uint32) -> bool {
    true
}

fn request_not_supported(old_s: S) -> bool {
    true
}

fn agent_not_allowed(old_s: S, domain_id: uint32) -> bool {
    true
}

fn config_bits_3_0_mode_valid(config: uint32) -> bool {
    true
}

fn config_bits_31_4_zero(config: uint32) -> bool {
    (config >> 4) == 0
}
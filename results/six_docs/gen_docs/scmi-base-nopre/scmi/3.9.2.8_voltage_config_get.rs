pub open spec fn 3.9.2.8_voltage_config_get_spec(result: int32, config: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> domain_id_not_found(old_s, domain_id))
    && (result == NOT_SUPPORTED ==> request_not_supported(old_s))
    && (result == DENIED ==> agent_not_allowed(old_s, domain_id))
    && (result == SUCCESS ==> (config_mode_valid(old_s, config) && config_reserved_zero(old_s, config)))
}

pub open spec fn domain_id_not_found(old_s: S, domain_id: uint32) -> bool {
    true
}

pub open spec fn request_not_supported(old_s: S) -> bool {
    true
}

pub open spec fn agent_not_allowed(old_s: S, domain_id: uint32) -> bool {
    true
}

pub open spec fn config_mode_valid(old_s: S, config: uint32) -> bool {
    (config & 0xFFFF_FFFF_FFFF_FFFF) == 0
}

pub open spec fn config_reserved_zero(old_s: S, config: uint32) -> bool {
    (config & 0xFFFF_FFFF_FFFF_FFFF) == 0
}
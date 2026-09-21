pub open spec fn clock_config_get_spec(result: int32, attributes: uint32, config: uint32, extended_config_val: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_not_found(old_s, clock_id))
    && (result == INVALID_PARAMETERS ==> (flags_nonzero(old_s, flags) || extended_config_type_invalid(old_s, flags)))
    && (result == SUCCESS ==> (attributes == old_s.clock_config.attributes && config == old_s.clock_config.config && extended_config_val == old_s.clock_config.extended_config_val))
}

fn clock_id_not_found(old_s: S, clock_id: uint32) -> bool {
    true
}

fn flags_nonzero(old_s: S, flags: uint32) -> bool {
    (flags & 0xFF00) != 0
}

fn extended_config_type_invalid(old_s: S, flags: uint32) -> bool {
    (flags & 0xFF) != 0
}
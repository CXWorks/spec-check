pub open spec fn 3.9.2.9_voltage_level_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.voltage_domains[old_s.domain_id] == new_s.voltage_domains[new_s.domain_id]))
    && (result == 0 ==> (new_s.voltage_domains[new_s.domain_id] == voltage_level))
    && (result != 0 ==> (result == NOT_FOUND || result == INVALID_PARAMETERS || result == NOT_SUPPORTED || result == DENIED))
}
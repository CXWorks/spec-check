pub open spec fn voltage_config_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.voltage_domains.contains(old_s.domain_id) && old_s.voltage_domains[old_s.domain_id].config == new_s.voltage_domains[old_s.domain_id].config))
    && (result == -1 ==> !old_s.voltage_domains.contains(old_s.domain_id))
    && (result == -2 ==> (old_s.voltage_domains.contains(old_s.domain_id) && (new_s.voltage_domains[old_s.domain_id].config & 0xF) != 0 || new_s.voltage_domains[old_s.domain_id].config & 0xF0000000 != 0))
    && (result == -3 ==> !old_s.voltage_domains.contains(old_s.domain_id))
    && (result == -4 ==> !old_s.voltage_domains.contains(old_s.domain_id))
    && (result == -5 ==> !old_s.voltage_domains.contains(old_s.domain_id))
}
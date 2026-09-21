pub open spec fn 3.10.3.14_powercap_measurements_get_spec(result: int32, power: uint32, mai: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (power == old_s.powercap_measurements[old_s.domain_id].power && mai == old_s.powercap_measurements[old_s.domain_id].mai))
    && (result != 0 ==> (old_s.domain_id >= old_s.num_powercap_domains || old_s.domain_id < 0))
}
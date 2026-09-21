pub open spec fn 3.3.2.5_power_domain_attributes_spec(result: int32, attributes: uint32, name: [16]uint8, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (attributes == old_s.power_domain_attributes[old_s.domain_id] && name == old_s.power_domain_name[old_s.domain_id]))
    && (result != 0 ==> (old_s.domain_id >= 0 && old_s.domain_id < 65536))
}
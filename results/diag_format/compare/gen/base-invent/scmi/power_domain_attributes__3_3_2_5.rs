pub open spec fn power_domain_attributes__3_3_2_5_spec(result: int32, attributes: uint32, name: [uint8; 16], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (attributes == 0 && name == [0u8; 16]))
    && (result == SUCCESS ==> (attributes >= 0 && attributes <= 0x07FFFFFF))
    && (result == SUCCESS ==> (name[0] != 0 || name[16] == 0))
    && (result == SUCCESS ==> (name[16] == 0))
    && (result == SUCCESS ==> (name[0..16] == old_s.power_domain_attributes__3_3_2_5_name[old_s.domain_id]))
    && (result == SUCCESS ==> (attributes == old_s.power_domain_attributes__3_3_2_5_attrs[old_s.domain_id]))
    && (result == SUCCESS ==> (new_s.power_domain_attributes__3_3_2_5_name == old_s.power_domain_attributes__3_3_2_5_name))
    && (result == SUCCESS ==> (new_s.power_domain_attributes__3_3_2_5_attrs == old_s.power_domain_attributes__3_3_2_5_attrs))
}
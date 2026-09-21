pub open spec fn 3.9.2.5_voltage_domain_attributes_spec(result: int32, attributes: uint32, name: [16]uint8, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (attributes >= 0 && attributes <= 0xFFFFFFFF && name[0] == 0))
    && (result == -1 ==> true)
    && (result != 0 && result != -1 ==> true)
    && (old_s == new_s)
}
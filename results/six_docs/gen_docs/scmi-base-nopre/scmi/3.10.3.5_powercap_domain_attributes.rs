pub open spec fn 3.10.3.5_powercap_domain_attributes_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.powercap_domain_attributes_valid(old_s.domain_id)))
    && (result != 0 ==> (new_s.powercap_domain_attributes_invalid(old_s.domain_id)))
}
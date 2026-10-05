pub open spec fn reset_domain_attributes__3_8_2_5_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.reset_domains.contains(old_s.domain_id) && new_s.reset_domains.contains(new_s.domain_id)))
    && (result != 0 ==> !old_s.reset_domains.contains(old_s.domain_id))
}
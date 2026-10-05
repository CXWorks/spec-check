pub open spec fn powercap_domain_attributes__3_10_3_5_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, new_s.domain_id as u32) ==> ResultEqual(result, RMI_ERROR_NOT_FOUND))
    && (ResultEqual(result, RMI_SUCCESS) ==> (
        ValidPowercapDomainAttributes(old_s, new_s)
        && (new_s.cap_config == 1 || new_s.power_monitor == 1)
        && (new_s.power_unit == 0 || new_s.power_unit == 1 || new_s.power_unit == 2)
        && (new_s.cap_config == 1 ==> new_s.num_limits > 0)
        && (new_s.reserved == 0)
        && (new_s.min_mai == new_s.max_mai ==> !MaiIsConfigurable(old_s, new_s.domain_id as u32))
        && (new_s.min_mai != new_s.max_mai ==> new_s.mai_step != 0)
        && (new_s.min_power_cap != 0 && new_s.max_power_cap != 0)
        && (new_s.min_power_cap == new_s.max_power_cap ==> !PowerCapIsConfigurable(old_s, new_s.domain_id as u32))
        && (new_s.min_power_cap != new_s.max_power_cap ==> new_s.power_cap_step != 0)
        && (new_s.min_cai == new_s.max_cai ==> !CaiIsConfigurable(old_s, new_s.domain_id as u32))
        && (new_s.min_cai != new_s.max_cai ==> new_s.cai_step != 0)
    ))
}
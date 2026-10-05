pub open spec fn powercap_cpc_attributes__3_10_3_6_spec(
    result: Int32,
    num_returned: UInt16,
    num_remaining: UInt16,
    desc: [CPLi_DESC],
    old_s: S,
    new_s: S
) -> bool {
    // Failure conditions
    (!PowercapDomainExists(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpliDescIndex(domain_id(old_s), desc_index(old_s)) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!IsRequestSupported() || !DomainSupportsCpc(domain_id(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    // Success conditions
    && (ResultEqual(result, SUCCESS) ==> (
        num_returned == desc.len as UInt16
        && num_remaining == (total_cpl_count(old_s) - num_returned)
        && desc[0] == CpliDescriptor(domain_id(old_s), desc_index(old_s))
        && IsAscending(|i: UInt16; i < num_returned; i => desc[i].cpli)
        && (forall i: UInt16; i < num_returned => desc[i].flags[31:1] == 0)
        && (forall i: UInt16; i < num_returned => desc[i].min_power_cap != 0)
        && (forall i: UInt16; i < num_returned => desc[i].max_power_cap != 0)
        && (forall i: UInt16; i < num_returned => (desc[i].min_power_cap == desc[i].max_power_cap || desc[i].power_cap_step != 0))
        && (forall i: UInt16; i < num_returned => (desc[i].min_cai == desc[i].max_cai || desc[i].cai_step != 0))
    ))
    // Unchanged state
    && (old_s == new_s)
}
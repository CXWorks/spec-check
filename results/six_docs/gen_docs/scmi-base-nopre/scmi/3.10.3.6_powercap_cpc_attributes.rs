pub open spec fn 3.10.3.6_powercap_cpc_attributes_spec(
    result: int32,
    num_cpl: uint32,
    desc: [CPLi_DESC],
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == OUT_OF_RANGE ==> desc_index >= num_cpl_descriptors(old_s, domain_id))
    && (result == NOT_SUPPORTED ==> !DomainSupportsCpc(old_s, domain_id))
    // Success conditions
    && (result == SUCCESS ==>
        // Input validation
        (domain_id < MAX_DOMAIN_ID)
        && (desc_index >= 0)
        && (desc_index < num_cpl_descriptors(old_s, domain_id))
        // Output consistency
        && (num_cpl == num_cpl_descriptors(old_s, domain_id) - desc_index)
        && (forall i: int | 0 <= i && i < len(desc) ==>
            // CPLi Descriptor constraints
            (desc[i].flags & 0x1) == 0 || (desc[i].flags & 0x1) == 1
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].min_power_cap != 0)
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].max_power_cap != 0)
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].min_power_cap <= desc[i].max_power_cap)
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].min_cai == 0 || desc[i].max_cai == 0 || desc[i].min_cai <= desc[i].max_cai)
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].min_cai == 0 || desc[i].max_cai == 0 || desc[i].cai_step != 0)
        )
        && (forall i: int | 0 <= i && i < len(desc) ==>
            (desc[i].name[0] != 0 || desc[i].name[0] == 0) // Null-terminated ASCII
        )
        // State transition: domain attributes unchanged
        && (forall i: int | 0 <= i && i < len(desc) ==>
            CPLiDescriptorEqual(old_s, new_s, domain_id, i)
        )
    )
}
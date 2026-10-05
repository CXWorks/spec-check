pub open spec fn powercap_cpc_attributes__3_10_3_6_spec(result: int32, num_cpl: uint32, desc: [CPLi_DESC], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (desc_index(old_s) >= num_cpl_descriptors(old_s)))
    && (result == OUT_OF_RANGE ==> (desc_index(old_s) >= num_cpl_descriptors(old_s)))
    && (result == NOT_SUPPORTED ==> (cpc_supported(old_s) == false))
    && (result == SUCCESS ==> (desc_index(old_s) < num_cpl_descriptors(old_s) && cpc_supported(old_s) == true))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].flags == 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].min_power_cap != 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].max_power_cap != 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].min_cai >= 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].max_cai >= 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (desc[i].name[15] == 0)))
    && (result == SUCCESS ==> (forall i: int | i < num_cpl ==> (forall j: int | i < j && j < num_cpl ==> (desc[i].cpli < desc[j].cpli))))
    && (result == SUCCESS ==> (new_s == old_s))
}
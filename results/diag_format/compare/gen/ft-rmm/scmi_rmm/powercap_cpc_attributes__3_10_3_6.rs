pub open spec fn powercap_cpc_attributes__3_10_3_6_spec(domain_id: UInt32, desc_index: UInt32, status: Int32, num_cpl: UInt16, desc: [CPLi_DESC; 1], old_s: S, new_s: S) -> bool {
  (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpliDescIndex(old_s, domain_id, desc_index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!IsRequestSupported(old_s) || !DomainSupportsCpc(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> num_cpl & 0xFFFF == desc.len())
  && (ResultEqual(status, SUCCESS) ==> num_cpl >> 16 == 0)
  && (ResultEqual(status, SUCCESS) ==> desc[0] == CpliDescriptor(old_s, domain_id, desc_index))
  && (ResultEqual(status, SUCCESS) ==> IsAscending(desc[0..desc.len()].cpli))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].flags[31:1] == 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].min_power_cap != 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].max_power_cap != 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].min_power_cap == desc[i].max_power_cap))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].min_power_cap != desc[i].max_power_cap implies desc[i].power_cap_step != 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].min_cai == desc[i].max_cai))
  && (ResultEqual(status, SUCCESS) ==> (forall i: int where i < desc.len(), desc[i].min_cai != desc[i].max_cai implies desc[i].cai_step != 0))
  && ((PowercapDomainExists(old_s, domain_id) &&
       IsValidCpliDescIndex(old_s, domain_id, desc_index) &&
       (IsRequestSupported(old_s) && DomainSupportsCpc(old_s, domain_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> num_cpl & 0xFFFF == 0)
  && (result != SUCCESS
    ==> num_cpl >> 16 == 0)
  && (result != SUCCESS
    ==> desc[0] == CpliDescriptor(old_s, domain_id, desc_index))
  && (result != SUCCESS
    ==> IsAscending(desc[0..desc.len()].cpli))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].flags[31:1] == 0))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].min_power_cap != 0))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].max_power_cap != 0))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].min_power_cap == desc[i].max_power_cap))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].min_power_cap != desc[i].max_power_cap implies desc[i].power_cap_step != 0))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].min_cai == desc[i].max_cai))
  && (result != SUCCESS
    ==> (forall i: int where i < desc.len(), desc[i].min_cai != desc[i].max_cai implies desc[i].cai_step != 0))
}
pub open spec fn powercap_cpc_attributes__3_10_3_6_spec(domain_id: UInt32, desc_index: UInt32, status: Int32, num_cpl: UInt32, desc: [CPLi_DESC; 4], old_s: S, new_s: S) -> bool {
  (!IsCommandSupported(old_s, 0x18, 0xD) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!DomainSupportsCpc(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidCpliDescIndex(old_s, domain_id, desc_index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_cpl, 15, 0) == 4)
  && (ResultEqual(status, SUCCESS) ==> Bits(num_cpl, 15, 0) <= MaxCpliDescsPerTransport(old_s))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_cpl, 31, 16) == NumCpliDescs(old_s, domain_id) - desc_index - Bits(num_cpl, 15, 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i + 1 < 4 ==> desc[i].cpli < desc[i + 1].cpli))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> Bits(desc[i].flags, 31, 1) == 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (Bits(desc[i].flags, 0, 0) == 1) == CpliSupportsPowerCapChange(old_s, domain_id, desc[i].cpli)))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> desc[i].min_power_cap != 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> desc[i].max_power_cap != 0))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (desc[i].min_power_cap == desc[i].max_power_cap ==> !IsPowerCapConfigurable(old_s, domain_id, desc[i].cpli))))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (desc[i].min_power_cap != desc[i].max_power_cap ==> desc[i].power_cap_step != 0)))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (desc[i].min_cai == desc[i].max_cai ==> !IsCaiConfigurable(old_s, domain_id, desc[i].cpli))))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (desc[i].min_cai != desc[i].max_cai ==> desc[i].cai_step != 0)))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> (!PlatformReportsCai(old_s, domain_id, desc[i].cpli) ==> desc[i].min_cai == 0 && desc[i].max_cai == 0)))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32, i < 4 ==> IsNullTerminatedAscii(desc[i].name, 16)))
  && ((IsCommandSupported(old_s, 0x18, 0xD) &&
       IsValidPowercapDomain(old_s, domain_id) &&
       DomainSupportsCpc(old_s, domain_id) &&
       IsValidCpliDescIndex(old_s, domain_id, desc_index))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> Bits(num_cpl, 15, 0) == 0)
  && (result != SUCCESS
    ==> Bits(num_cpl, 31, 16) == NumCpliDescs(old_s, domain_id) - desc_index)
  && (result != SUCCESS
    ==> desc[0] == CpliDescriptor(old_s, domain_id, desc_index))
  && (result != SUCCESS
    ==> desc[1] == CpliDescriptor(old_s, domain_id, desc_index + 1))
  && (result != SUCCESS
    ==> desc[2] == CpliDescriptor(old_s, domain_id, desc_index + 2))
  && (result != SUCCESS
    ==> desc[3] == CpliDescriptor(old_s, domain_id, desc_index + 3))
  && (!(result == SUCCESS &&
       (Bits(num_cpl, 15, 0) == 4))
    ==> (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
  && (!(result == SUCCESS &&
       (Bits(num_cpl, 15, 0) <= MaxCpliDescsPerTransport(old_s)))
    ==> (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
  && (!(result == SUCCESS &&
       (Bits(num_cpl, 31, 16) == NumCpliDescs(old_s, domain_id) - desc_index - Bits(num_cpl, 15, 0)))
    ==> (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
  && (!(result == SUCCESS &&
       (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
    ==> (forall i: UInt32, i < 4 ==> desc[i] == CpliDescriptor(old_s, domain_id, desc_index + i)))
  && (!(result == SUCCESS &&
       (forall i: UInt32, i + 1 < 4 ==> desc[i].cpli < desc[i + 1].cpli))
    ==> (forall i: UInt32, i + 1 < 4 ==> desc[i].cpli < desc[i + 1].cpli))
  && (!(result == SUCCESS &&
       (forall i: UInt32, i < 4 ==> Bits(desc[i].flags, 31, 1) == 0))
    ==> (forall i: UInt32, i < 4 ==> Bits(desc[i].flags, 31, 1) == 0)))
}
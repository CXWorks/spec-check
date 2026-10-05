pub open spec fn powercap_cai_set__3_10_3_12_spec(domain_id: UInt32, flags: UInt32, cai: UInt32, cpli: UInt32, result: Result<Int32, PowercapCaiSetReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
  && (!IsCaiSetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsSupportedCai(old_s, domain_id, cai) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidCaiSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AgentMaySetCai(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> PowercapDomain(new_s, domain_id).cai == cai)
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsCaiSetSupported(old_s, domain_id) &&
       IsSupportedCai(old_s, domain_id, cai) &&
       AreValidCaiSetFlags(old_s, flags) &&
       AgentMaySetCai(old_s, calling_agent, domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> PowercapDomain(new_s, domain_id).cai == PowercapDomain(old_s, domain_id).cai)
}
pub open spec fn powercap_cai_set__3_10_3_12_spec(domain_id: UInt32, flags: UInt32, cai: UInt32, cpli: UInt32, result: Result<Int32, PowercapCaiSetReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomainId(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
  && (!IsCaiSetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsSupportedCai(old_s, domain_id, cai) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AgentMaySetCai(old_s, CallingAgent(), domain_id) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> PowercapCai(new_s, domain_id, cpli) == cai)
  && ((IsValidPowercapDomainId(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsCaiSetSupported(old_s, domain_id) &&
       IsSupportedCai(old_s, domain_id, cai) &&
       !(flags != 0) &&
       AgentMaySetCai(old_s, CallingAgent(), domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> PowercapCai(new_s, domain_id, cpli) == PowercapCai(old_s, domain_id, cpli))
}
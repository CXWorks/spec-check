pub open spec fn powercap_cai_get__3_10_3_11_spec(domain_id: UInt32, cpli: UInt32, result: Result<Int32, UInt32>, cai: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
  && (!DomainSupportsCpc(old_s, domain_id) && cpli != 0 ==> ResultEqual(result, NOT_FOUND))
  && (!IsCaiGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> cai == CurrentCai(new_s, domain_id, cpli))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       !(DomainSupportsCpc(old_s, domain_id) && cpli != 0) &&
       IsCaiGetSupported(old_s, domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> cai == CurrentCai(new_s, domain_id, cpli))
}
pub open spec fn powercap_cai_get__3_10_3_11_spec(domain_id: UInt32, cpli: UInt32, status: Int32, cai: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> cai == EnforcedCai(new_s, domain_id, cpli))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidCpli(old_s, domain_id, cpli) &&
       IsRequestSupported(old_s, domain_id, cpli))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> cai == EnforcedCai(old_s, domain_id, cpli))
}
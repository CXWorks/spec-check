pub open spec fn powercap_mai_get__3_10_3_9_spec(domain_id: UInt32, status: Int32, mai: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, POWERCAP_MAI_GET) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> mai == EnforcedMai(new_s, domain_id))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsRequestSupported(old_s, POWERCAP_MAI_GET))
    ==> ResultEqual(status, SUCCESS))
}
pub open spec fn powercap_domain_name_get__3_10_3_13_spec(domain_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> name == PowercapDomainExtendedName(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && ((PowercapDomainExists(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}
pub open spec fn power_domain_name_get__3_3_2_10_spec(domain_id: UInt32, status: Int32, flags: UInt32, ext_name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!PowerDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> ext_name == PowerDomainExtendedName(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAscii(new_s, ext_name, 64))
  && ((PowerDomainExists(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}
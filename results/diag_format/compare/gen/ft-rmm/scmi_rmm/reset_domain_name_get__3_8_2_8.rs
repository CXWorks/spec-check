pub open spec fn reset_domain_name_get__3_8_2_8_spec(domain_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!ResetDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> name == ResetDomainExtendedName(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAscii(name, 64))
  && ((ResetDomainExists(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}
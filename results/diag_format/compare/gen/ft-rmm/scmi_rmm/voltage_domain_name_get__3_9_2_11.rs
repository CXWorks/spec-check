pub open spec fn voltage_domain_name_get__3_9_2_11_spec(domain_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> name == VoltageDomainExtendedName(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAscii(name, 64))
  && ((VoltageDomainExists(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
}
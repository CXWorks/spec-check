pub open spec fn performance_domain_name_get__3_5_6_19_spec(domain_id: UInt32, status: Int32, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (!PerformanceDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags == 0)
  && (ResultEqual(status, SUCCESS) ==> IsNullTerminatedAsciiString(name, 64) && name == PerformanceDomainExtendedName(old_s, domain_id))
  && ((PerformanceDomainExists(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result == RSI_SUCCESS
    ==> flags == 0)
  && (result == RSI_SUCCESS
    ==> IsNullTerminatedAsciiString(name, 64) && name == PerformanceDomainExtendedName(old_s, domain_id))
}
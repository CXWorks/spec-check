pub open spec fn power_domain_attributes__3_3_2_5_spec(domain_id: UInt32, status: Int32, attributes: UInt32, name: [UInt8; 16], result: Result<Int32, PowerDomainAttributesReturn>, old_s: S, new_s: S) -> bool {
  (!PowerDomainExists(old_s, Bits(domain_id, 15, 0)) ==> ResultEqual(result, NOT_FOUND))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> (Bits(attributes, 31, 31) == 1) == PowerStateChangeNotifySupported(old_s, Bits(domain_id, 15, 0)))
  && (result == SUCCESS ==> (Bits(attributes, 30, 30) == 1) == PowerStateAsyncSetSupported(old_s, Bits(domain_id, 15, 0)))
  && (result == SUCCESS ==> (Bits(attributes, 29, 29) == 1) == PowerStateSyncSetSupported(old_s, Bits(domain_id, 15, 0)))
  && (result == SUCCESS ==> (Bits(attributes, 28, 28) == 1) == PowerStateChangeRequestedNotifySupported(old_s, Bits(domain_id, 15, 0)))
  && (result == SUCCESS ==> (Bits(attributes, 27, 27) == 1) == (PowerDomainNameLength(old_s, Bits(domain_id, 15, 0)) > 16))
  && (result == SUCCESS ==> Bits(attributes, 26, 0) == 0)
  && (result == SUCCESS ==> Bits(attributes, 27, 27) == 0 ==> (IsNullTerminatedAscii(name, 16) && name == PowerDomainName(old_s, Bits(domain_id, 15, 0))))
  && (result == SUCCESS ==> Bits(attributes, 27, 27) == 1 ==> name == NullTerminatedPrefix(PowerDomainName(old_s, Bits(domain_id, 15, 0)), 15))
  && ((PowerDomainExists(old_s, Bits(domain_id, 15, 0)))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> (Bits(attributes, 31, 31) == 1) == PowerStateChangeNotifySupported(old_s, Bits(domain_id, 15, 0)))
  && (result != SUCCESS
    ==> (Bits(attributes, 30, 30) == 1) == PowerStateAsyncSetSupported(old_s, Bits(domain_id, 15, 0)))
  && (result != SUCCESS
    ==> (Bits(attributes, 29, 29) == 1) == PowerStateSyncSetSupported(old_s, Bits(domain_id, 15, 0)))
  && (result != SUCCESS
    ==> (Bits(attributes, 28, 28) == 1) == PowerStateChangeRequestedNotifySupported(old_s, Bits(domain_id, 15, 0)))
  && (result != SUCCESS
    ==> (Bits(attributes, 27, 27) == 1) == (PowerDomainNameLength(old_s, Bits(domain_id, 15, 0)) > 16))
  && (result != SUCCESS
    ==> Bits(attributes, 26, 0) == 0)
  && (result != SUCCESS
    ==> Bits(attributes, 27, 27) == 0 ==> (IsNullTerminatedAscii(name, 16) && name == PowerDomainName(old_s, Bits(domain_id, 15, 0))))
  && (result != SUCCESS
    ==> Bits(attributes, 27, 27) == 1 ==> name == NullTerminatedPrefix(PowerDomainName(old_s, Bits(domain_id, 15, 0)), 15))
}
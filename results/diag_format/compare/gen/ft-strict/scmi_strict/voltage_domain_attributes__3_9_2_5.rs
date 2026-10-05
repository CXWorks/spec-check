pub open spec fn voltage_domain_attributes__3_9_2_5_spec(domain_id: UInt32, status: Int32, attributes: UInt32, name: [UInt8; 16], result: Result<VoltageDomainAttributes, RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, Bits(domain_id, 15, 0)) ==> ResultEqual(result, NOT_FOUND))
  && (result == RSI_SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == RSI_SUCCESS ==> (Bit(attributes, 31) == 1) == VoltageDomainSupportsAsyncLevelSet(old_s, Bits(domain_id, 15, 0)))
  && (result == RSI_SUCCESS ==> (Bit(attributes, 30) == 1) == (VoltageDomainNameLength(old_s, Bits(domain_id, 15, 0)) > 16))
  && (result == RSI_SUCCESS ==> Bits(attributes, 29, 0) == 0)
  && (result == RSI_SUCCESS ==> Bit(attributes, 30) == 0 ==> NameEquals(name, VoltageDomainName(old_s, Bits(domain_id, 15, 0))))
  && (result == RSI_SUCCESS ==> Bit(attributes, 30) == 1 ==> NameEqualsLowerBytes(name, VoltageDomainName(old_s, Bits(domain_id, 15, 0)), 15))
  && ((VoltageDomainExists(old_s, Bits(domain_id, 15, 0)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> status == 0)
  && (result != RSI_SUCCESS
    ==> attributes == 0)
  && (result != RSI_SUCCESS
    ==> name[0] == 0)
}
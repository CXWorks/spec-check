pub open spec fn voltage_domain_attributes__3_9_2_5_spec(domain_id: UInt32, status: Int32, attributes: UInt32, name: [UInt8; 16], result: Result<VoltageDomainAttributes, RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!VoltageDomainExists(old_s, domain_id[15:0]) ==> ResultEqual(result, NOT_FOUND))
  && (result == RSI_SUCCESS ==> ResultEqual(result.status, SUCCESS))
  && (result == RSI_SUCCESS ==> attributes[31] == (VoltageDomainSupportsAsyncLevelSet(old_s, domain_id[15:0]) ? 1 : 0))
  && (result == RSI_SUCCESS ==> attributes[30] == (VoltageDomainNameLength(old_s, domain_id[15:0]) > 16 ? 1 : 0))
  && (result == RSI_SUCCESS ==> attributes[29:0] == 0)
  && (result == RSI_SUCCESS ==> IsNullTerminatedAscii(new_s, name, 16))
  && (result == RSI_SUCCESS ==> attributes[30] == 1 implies name == LowerBytes(new_s, VoltageDomainName(old_s, domain_id[15:0]), 15))
  && ((!(VoltageDomainExists(old_s, domain_id[15:0])))
    ==> result == RSI_SUCCESS)
}
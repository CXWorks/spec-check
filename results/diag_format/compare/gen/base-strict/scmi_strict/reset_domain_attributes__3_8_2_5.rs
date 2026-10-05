pub open spec fn reset_domain_attributes__3_8_2_5_spec(result: Int32, attributes: UInt32, latency: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!IsValidResetDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (IsValidResetDomain(domain_id(old_s)) ==> ResultEqual(result, SUCCESS))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 31, 31) == 1) == ResetDomainSupportsAsyncReset(domain_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 30, 30) == 1) == ResetDomainSupportsResetNotifications(domain_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 29, 29) == 1) ==> ResetDomainNameLength(domain_id(old_s)) > 16)
    && (ResultEqual(result, SUCCESS) ==> Bits(attributes, 28, 0) == 0)
    && (ResultEqual(result, SUCCESS) ==> ResetLatencySupported(domain_id(old_s)) ==> latency == MaxResetLatencyUs(domain_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> !ResetLatencySupported(domain_id(old_s)) ==> latency == 0xFFFFFFFF)
    && (ResultEqual(result, SUCCESS) ==> IsNullTerminatedAsciiString(name, 16))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 29, 29) == 0) ==> name == ResetDomainName(domain_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 29, 29) == 1) ==> name == NullTerminatedPrefix(ResetDomainName(domain_id(old_s)), 15))
    && (old_s == new_s)
}
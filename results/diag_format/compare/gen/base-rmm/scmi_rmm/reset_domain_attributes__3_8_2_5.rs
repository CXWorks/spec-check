pub open spec fn reset_domain_attributes__3_8_2_5_spec(result: Int32, attributes: UInt32, latency: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!ResetDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        attributes[28:0] == 0
        && attributes[31] == (ResetDomain(old_s, domain_id).supports_async_reset ? 1 : 0)
        && attributes[30] == (ResetDomain(old_s, domain_id).supports_reset_notifications ? 1 : 0)
        && attributes[29] == (ResetDomain(old_s, domain_id).name_longer_than_16_bytes ? 1 : 0)
        && (latency == ResetDomain(old_s, domain_id).max_reset_latency_us || latency == 0xFFFFFFFF)
        && (attributes[29] == 0 ? name == ResetDomain(old_s, domain_id).name : name == LowerBytes(ResetDomain(old_s, domain_id).name, 15))
    ))
    && (old_s == new_s)
}
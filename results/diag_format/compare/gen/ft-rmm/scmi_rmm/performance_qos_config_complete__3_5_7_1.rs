pub open spec fn performance_qos_config_complete__3_5_7_1_spec(status: int, domain_id: uint32, capability: uint32, flags: uint32, qos_value: uint32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> flags & 0x1F == 0 && flags & 3 == 0)
  && (result.is_Ok() && flags & 16 == 16 ==> domain_id == 0xFFFFFFFF && AllDomainsQosAtPlatformDefault(new_s, capability))
  && (result.is_Ok() && flags & 8 == 8 ==> DomainAndSiblingsQosAtPlatformDefault(new_s, domain_id, capability))
  && (result.is_Ok() && flags & 4 == 4 ==> QosAtPlatformDefault(new_s, domain_id, capability))
  && (result.is_Ok() && flags & 16 == 0 && flags & 8 == 0 && flags & 4 == 0 ==> QosValue(new_s, domain_id, capability) == qos_value)
  && ((!(result.is_Ok() && ResultEqual(result, SUCCESS)) &&
       !(result.is_Ok() && (flags & 0x1F == 0 && flags & 3 == 0)))
    ==> (DomainAndSiblingsQosAtPlatformDefault(new_s, domain_id, capability) == DomainAndSiblingsQosAtPlatformDefault(old_s, domain_id, capability))
    && (QosAtPlatformDefault(new_s, domain_id, capability) == QosAtPlatformDefault(old_s, domain_id, capability))
    && (QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability)))
}
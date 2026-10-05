pub open spec fn performance_qos_config_complete__3_5_7_1_spec(status: Int32, domain_id: UInt32, capability: UInt32, flags: UInt32, qos_value: UInt32, old_s: S, new_s: S) -> bool {
  (ResultEqual(status, SUCCESS))
  && (Bits(flags, 31, 5) == 0)
  && (Bits(flags, 1, 0) == 0)
  && (Bit(flags, 4) == 1 ==> domain_id == 0xFFFFFFFF)
  && (Bit(flags, 4) == 1 ==> (forall d: PerfDomainId| QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability)))
  && (Bit(flags, 3) == 1 ==> (forall d: PerfDomainId| (d == domain_id || IsSiblingDomain(new_s, d, domain_id)) ==> QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability)))
  && (Bit(flags, 2) == 1 ==> QosValue(new_s, domain_id, capability) == PlatformDefaultQos(new_s, domain_id, capability))
  && ((Bit(flags, 2) == 0 && Bit(flags, 3) == 0 && Bit(flags, 4) == 0) ==> QosValue(new_s, domain_id, capability) == qos_value)
  && ((!(ResultEqual(status, SUCCESS)) ||
       !(Bits(flags, 31, 5) == 0) ||
       !(Bits(flags, 1, 0) == 0) ||
       !(Bit(flags, 4) == 1 ==> domain_id == 0xFFFFFFFF) ||
       !(Bit(flags, 4) == 1 ==> (forall d: PerfDomainId| QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability))) ||
       !(Bit(flags, 3) == 1 ==> (forall d: PerfDomainId| (d == domain_id || IsSiblingDomain(new_s, d, domain_id)) ==> QosValue(new_s, d, capability) == PlatformDefaultQos(new_s, d, capability))) ||
       !(Bit(flags, 2) == 1 ==> QosValue(new_s, domain_id, capability) == PlatformDefaultQos(new_s, domain_id, capability)) ||
       !((Bit(flags, 2) == 0 && Bit(flags, 3) == 0 && Bit(flags, 4) == 0) ==> QosValue(new_s, domain_id, capability) == qos_value))
    ==>
    QosValue(new_s, domain_id, capability) == QosValue(old_s, domain_id, capability))
}
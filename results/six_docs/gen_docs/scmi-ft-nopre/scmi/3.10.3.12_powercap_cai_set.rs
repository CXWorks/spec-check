pub open spec fn 3.10.3.12_powercap_cai_set_spec(domain_id: UInt32, flags: UInt32, cai: UInt32, cpli: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> (PowerCappingDomainAt(new_s, domain_id).cai == cai))
  && (result == RSI_SUCCESS ==> (PowerCappingDomainAt(new_s, domain_id).cpli == cpli))
  && ((!(flags == 0)) ==> result == RSI_ERROR_INVALID_PARAMETERS)
  && (cai == 0 ==> result == RSI_ERROR_INVALID_PARAMETERS)
  && ((result != RSI_SUCCESS)
    ==> (PowerCappingDomainAt(new_s, domain_id).cai == PowerCappingDomainAt(old_s, domain_id).cai))
  && ((result != RSI_SUCCESS)
    ==> (PowerCappingDomainAt(new_s, domain_id).cpli == PowerCappingDomainAt(old_s, domain_id).cpli))
}
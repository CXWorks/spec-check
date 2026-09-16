pub open spec fn rmi_dpt_l1_create_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (DptL0(old_s).state != DPT_L0_VALID ==> result.status == RMI_ERROR_GLOBAL)
  && ((addr) >= Rmm(old_s).static.dptps ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, addr, Rmm(old_s).static.l0dptsz as int) ==> result.status == RMI_ERROR_INPUT)
  && (result.status == DPT_L0_ENTRY_TABLE ==> result.status == RMI_ERROR_DPT)
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> DptL0Walk(new_s, addr).state == DPT_L0_ENTRY_TABLE)
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       !(DptL0(old_s).state != DPT_L0_VALID) &&
       !((addr) >= Rmm(old_s).static.dptps) &&
       AddrIsAligned(old_s, addr, Rmm(old_s).static.l0dptsz as int))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> DptL0Walk(new_s, addr).state == DptL0Walk(old_s, addr).state)
}
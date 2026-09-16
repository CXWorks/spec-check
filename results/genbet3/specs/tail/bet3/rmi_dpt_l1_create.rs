pub open spec fn rmi_dpt_l1_create_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (DptL0(old_s).state != DPT_L0_VALID ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && ((addr) >= DptL0s(old_s).dptps ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, DptL0s(old_s).l0dptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (DptL0Walk(old_s, addr).state == DPT_L0_ENTRY_TABLE ==> ResultEqual(result, RMI_ERROR_DPT))
  && (result.is_Ok() ==> result.get_Ok_0().status == RMI_SUCCESS)
  && (result.is_Ok() ==> DptL0Walk(new_s, addr).state == DPT_L0_ENTRY_TABLE)
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       !(DptL0(old_s).state != DPT_L0_VALID) &&
       !((addr) >= DptL0s(old_s).dptps) &&
       AddrIsAligned(old_s, addr, DptL0s(old_s).l0dptsz as int) &&
       !(DptL0Walk(old_s, addr).state == DPT_L0_ENTRY_TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> DptL0Walk(new_s, addr).state == DptL0Walk(old_s, addr).state)
}
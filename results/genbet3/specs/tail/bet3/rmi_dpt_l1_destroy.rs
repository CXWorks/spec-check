pub open spec fn rmi_dpt_l1_destroy_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (DptL0(old_s, addr).state != DPT_L0_VALID ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && ((addr) >= (Rmm(old_s).static.dptps) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, Rmm(old_s).static.l0dptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (DptL0Walk(old_s, addr).state == DPT_L0_ENTRY_BLOCK ==> (ResultEqual(result, RMI_ERROR_DPT) && (ResultEqual(result, RMI_ERROR_DPT).get_Err_0().level.level == 0)))
  && (!DptL1IsHomogeneous(old_s, addr) ==> (ResultEqual(result, RMI_ERROR_DPT) && (ResultEqual(result, RMI_ERROR_DPT).get_Err_0().level.level == 1)))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> DptL0Walk(new_s, addr).state == DPT_L0_ENTRY_BLOCK)
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       DptL0(old_s, addr).state == DPT_L0_VALID &&
       !((addr) >= (Rmm(old_s).static.dptps)) &&
       AddrIsAligned(old_s, addr, Rmm(old_s).static.l0dptsz as int) &&
       !(DptL0Walk(old_s, addr).state == DPT_L0_ENTRY_BLOCK) &&
       DptL1IsHomogeneous(old_s, addr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> DptL0Walk(new_s, addr).state == DptL0Walk(old_s, addr).state)
}
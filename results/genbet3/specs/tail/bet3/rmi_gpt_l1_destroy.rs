pub open spec fn rmi_gpt_l1_destroy_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((addr) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GptParState(old_s, addr) IN { GPT_PAR_RESERVED, GPT_PAR_PLAT} ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (GptParState(old_s, addr) == GPT_PAR_HOST_NOT_CREATED ==> (result.status == RMI_ERROR_GPT && ResultEqual(result, RMI_ERROR_GPT) && RttWalk(new_s, RealmAt(new_s, 0), addr, 0 as int, 0 as int).level.level == 0))
  && (!GptL1IsHomogeneous(old_s, addr) ==> (result.status == RMI_ERROR_GPT && ResultEqual(result, RMI_ERROR_GPT) && RttWalk(new_s, RealmAt(new_s, 0), addr, 0 as int, 0 as int).level.level == 1))
  && (result.status == RMI_SUCCESS ==> result.status == RMI_SUCCESS)
  && (result.status == RMI_SUCCESS ==> GptParState(new_s, addr) == GPT_PAR_HOST_NOT_CREATED)
  && ((!(addr) >= Rmm().static_.pasz &&
       AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) &&
       !(GptParState(old_s, addr) IN { GPT_PAR_RESERVED, GPT_PAR_PLAT}) &&
       !(GptParState(old_s, addr) == GPT_PAR_HOST_NOT_CREATED) &&
       GptL1IsHomogeneous(old_s, addr))
    ==> result.status == RMI_SUCCESS)
  && (result.is_Err()
    ==> GptParState(new_s, addr) == GptParState(old_s, addr))
}
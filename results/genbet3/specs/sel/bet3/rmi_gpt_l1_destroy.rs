pub open spec fn rmi_gpt_l1_destroy_spec(addr: Address, result: Result<(RmiResult), (RmiResult)>, old_s: S, new_s: S) -> bool {
  ((addr) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GptParState(old_s, addr) IN { GPT_PAR_RESERVED, GPT_PAR_PLAT} ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (GptParState(old_s, addr) == GPT_PAR_HOST_NOT_CREATED ==> (ResultEqual(result, RMI_ERROR_GPT(0)) && ResultEqual(result, RMI_ERROR_GPT(1))))
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() ==> GptParState(new_s, addr) == GPT_PAR_HOST_NOT_CREATED)
  && ((!(addr >= Rmm().static_.pasz) &&
       AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) &&
       !(GptParState(old_s, addr) IN { GPT_PAR_RESERVED, GPT_PAR_PLAT}))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GptParState(new_s, addr) == GptParState(old_s, addr))
}
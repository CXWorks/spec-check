pub open spec fn rmi_gpt_l1_create_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((addr) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GptParState(old_s, addr) == GPT_PAR_RESERVED || GptParState(old_s, addr) == GPT_PAR_PLAT ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (GptParState(old_s, addr) == GPT_PAR_HOST_CREATED ==> ResultEqual(result, RMI_ERROR_GPT))
  && (The RMM encountered a Granule within the memory donated for the L1GPT whose PAS is not NS or whose category is not DRAM. ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() ==> GptParState(new_s, addr) == GPT_PAR_HOST_CREATED)
  && ((!(addr) >= Rmm().static_.pasz &&
       AddrIsAligned(old_s, addr, Rmm().static_.l0gptsz as int) &&
       !(GptParState(old_s, addr) == GPT_PAR_RESERVED || GptParState(old_s, addr) == GPT_PAR_PLAT) &&
       !(GptParState(old_s, addr) == GPT_PAR_HOST_CREATED) &&
       !(The RMM encountered a Granule within the memory donated for the L1GPT whose PAS is not NS or whose category is not DRAM.))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GptParState(new_s, addr) == GptParState(old_s, addr))
}
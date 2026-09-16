pub open spec fn rmi_gpt_info_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, out_state: RmiGptParState, old_s: S, new_s: S) -> bool {
  ((base) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, base, Rmm().static_.l0gptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, top, Rmm().static_.l0gptsz as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((result).is_Ok() ==> out_top == top)
  && ((result).is_Ok() ==> out_state == RmiGptParState::RMM_GPT_PAR_STATE)
  && (((!(base) >= Rmm().static_.pasz) &&
       !(top) >= Rmm().static_.pasz &&
       AddrIsAligned(old_s, base, Rmm().static_.l0gptsz as int) &&
       AddrIsAligned(old_s, top, Rmm().static_.l0gptsz as int) &&
       !((top) <= (base)))
    ==> result.is_Ok())
}
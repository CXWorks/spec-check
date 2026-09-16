pub open spec fn rmi_gpt_info_spec(base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, state: RmiGptParState, old_s: S, new_s: S) -> bool {
  ((base) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) >= Rmm().static_.pasz ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, base, Rmm().static_.l0gptsz) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, top, Rmm().static_.l0gptsz) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((result).is_Ok() ==> out_top == top)
  && ((result).is_Ok() ==> state == GPT_PAR_STATE_UNIFORM)
  && (((!(base) >= Rmm().static_.pasz) &&
       !(top) >= Rmm().static_.pasz &&
       AddrIsAligned(old_s, base, Rmm().static_.l0gptsz) &&
       AddrIsAligned(old_s, top, Rmm().static_.l0gptsz) &&
       !((top) <= (base)))
    ==> result.is_Ok())
}
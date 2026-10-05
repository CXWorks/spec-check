pub open spec fn ffa_rxtx_map_spec(buffer_pair: BufferPair, callee_translation_regime: CalleeTranslationRegime, result: FfaReturnCode, old_s: S, new_s: S) -> bool {
  (result == FFA_SUCCESS ==> RxTxBufferPairIsMapped(new_s, callee_translation_regime, buffer_pair))
  && ((!(result == FFA_SUCCESS)) ==> RxTxBufferPairIsNotMapped(old_s, callee_translation_regime, buffer_pair))
}
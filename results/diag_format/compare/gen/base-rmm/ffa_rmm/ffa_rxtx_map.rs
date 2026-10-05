pub open spec fn ffa_rxtx_map_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> !RxTxBufferPairIsMapped(callee_translation_regime, buffer_pair))
    && (result == RSI_SUCCESS ==> RxTxBufferPairIsMapped(callee_translation_regime, buffer_pair))
    && (result != RSI_SUCCESS ==> !RxTxBufferPairIsMapped(callee_translation_regime, buffer_pair))
}
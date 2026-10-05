pub open spec fn ffa_rxtx_map_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> RxTxBufferPairMappedInCalleeRegime(new_s, rx_buffer, tx_buffer))
    && (RxTxBufferPairMappedInCalleeRegime(old_s, rx_buffer, tx_buffer) ==> RxTxBufferPairMappedInCalleeRegime(new_s, rx_buffer, tx_buffer))
}
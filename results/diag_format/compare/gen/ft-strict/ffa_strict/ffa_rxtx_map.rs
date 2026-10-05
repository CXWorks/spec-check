pub open spec fn ffa_rxtx_map_spec(rx_buffer: RxTxBuffer, tx_buffer: RxTxBuffer, old_s: S, new_s: S) -> bool {
  (RxTxBufferPairMappedInCalleeRegime(new_s, rx_buffer, tx_buffer))
}
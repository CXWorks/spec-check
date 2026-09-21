pub open spec fn ffa_rxtx_map_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.tx_addr as int) % 4096 != 0 ||
        (old_s.rx_addr as int) % 4096 != 0 ||
        (old_s.page_count as int) < 1 ||
        (old_s.page_count as int) > 0xFFFF ||
        (old_s.page_count as int) & 0x3F != 0 ||
        (old_s.tx_addr as int) % 4096 != 0 ||
        (old_s.rx_addr as int) % 4096 != 0
    ))
    && (result == FFA_NO_MEMORY ==> true)
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}
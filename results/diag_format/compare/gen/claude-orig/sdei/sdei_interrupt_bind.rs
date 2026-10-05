pub open spec fn sdei_interrupt_bind_spec(result: i64, interrupt: u32, old_s: S, new_s: S) -> bool {
    ((result >= 0) ==> ((result as int) < 0x1_0000_0000))
    && ((result < 0) ==> (result == -1 || result == -2 || result == -3 || result == -10))
}

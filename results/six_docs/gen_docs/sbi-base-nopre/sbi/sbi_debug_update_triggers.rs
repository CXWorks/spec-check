pub open spec fn sbi_debug_update_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let trig_count = result.arg0 as int;
    (trig_count >= 0)
    && (trig_count <= old_s.shared_memory.len() / 4)
    && (forall i: int | i >= 0 && i < trig_count ==> {
        let offset = i * 2; // XLEN / 2 where XLEN = 64
        let trig_idx = old_s.shared_memory[offset] as int;
        let trig_tdata1 = old_s.shared_memory[offset + 1] as int;
        let trig_tdata2 = old_s.shared_memory[offset + 2] as int;
        let trig_tdata3 = old_s.shared_memory[offset + 3] as int;
        // Check if trig_idx is a valid installed debug trigger index
        (trig_idx >= 0 && trig_idx < old_s.debug_triggers.len())
        // Check if trig_tdata1.type matches the original installed debug trigger
        && (trig_tdata1 & 0xFFFF_FFFF == old_s.debug_triggers[trig_idx].tdata1.type as int)
        // Check if trig_tdata1.chain matches the original installed debug trigger
        && ((trig_tdata1 >> 32) as int == old_s.debug_triggers[trig_idx].tdata1.chain as int)
    })
    && (result == 0)
}
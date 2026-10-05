pub open spec fn sbi_debug_update_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let trig_count = result.arg0 as int;
    (trig_count >= 0)
    && (trig_count <= old_s.shared_memory.len() / 4)
    && (forall i: int | i < trig_count ==> {
        let offset = (i * 2) as int;
        let trig_idx = old_s.shared_memory[offset];
        let trig_tdata1 = old_s.shared_memory[offset + 1];
        let trig_tdata2 = old_s.shared_memory[offset + 2];
        let trig_tdata3 = old_s.shared_memory[offset + 3];
        (trig_idx >= 0)
        && (trig_idx < old_s.debug_triggers.len())
        && (trig_tdata1.type == old_s.debug_triggers[trig_idx].type)
        && (trig_tdata1.chain == old_s.debug_triggers[trig_idx].chain)
    })
    && (result == 0)
}
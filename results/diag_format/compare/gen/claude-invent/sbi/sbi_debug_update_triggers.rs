pub open spec fn sbi_debug_update_triggers_spec(result: SbiRet, trig_count: UInt64, old_s: S, new_s: S) -> bool {
    (exists|i: int|
        0 <= i < trig_count as int
        && !(
            IsInstalledDebugTrigger(old_s, ShmemTrigIdx(old_s, i))
            && Tdata1Type(ShmemTrigTdata1(old_s, i))
                == Tdata1Type(InstalledTriggerTdata1(old_s, ShmemTrigIdx(old_s, i)))
            && Tdata1Chain(ShmemTrigTdata1(old_s, i))
                == Tdata1Chain(InstalledTriggerTdata1(old_s, ShmemTrigIdx(old_s, i)))
        )
    ) ==> result.error != SBI_SUCCESS
}

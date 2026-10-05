pub open spec fn telemetry_list_shmti__3_12_4_5_spec(index: u32, status: ScmiStatus, num_shmti: u32, shmti_desc: Seq<(u32, u32, u32, u32, u32)>, old_s: S, new_s: S) -> bool {
    (!IsShmtiSupported(old_s) ==> status == NOT_SUPPORTED)
    && ((IsShmtiSupported(old_s) && ShmtiCount(old_s) == 0) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        IsShmtiSupported(old_s)
        && ShmtiCount(old_s) > 0
        && shmti_desc.len() == ((num_shmti as int) % 0x1_0000)
        && (index as int) + shmti_desc.len() + ((num_shmti as int) / 0x1_0000) == ShmtiCount(old_s)
        && (forall|i: int| 0 <= i < shmti_desc.len() ==> (
            shmti_desc[i] == ShmtiDescAt(old_s, (index as int) + i)
            && shmti_desc[i].4 == 0
            && (((shmti_desc[i].2 as int) * 0x1_0000_0000 + (shmti_desc[i].1 as int)) % 8 == 0)
            && AddrInAgentMemoryMap(old_s, (shmti_desc[i].2 as int) * 0x1_0000_0000 + (shmti_desc[i].1 as int))
        ))
    ))
    && new_s == old_s
}

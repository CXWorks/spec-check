pub open spec fn pinctrl_name_get__3_11_2_11_spec(identifier: UInt32, flags: UInt32, status: Int32, ret_flags: UInt32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!PinctrlIdentifierExists(old_s, identifier & 0xFFFFu32, flags & 0x3u32) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        PinctrlIdentifierExists(old_s, identifier & 0xFFFFu32, flags & 0x3u32)
        && ret_flags == 0u32
        && name.len() == 64
        && (exists |i: int| 0 <= i < 64 && name[i] == 0u8
            && (forall |j: int| 0 <= j < i ==> (name[j] as int) < 128))
    ))
    && (new_s == old_s)
}

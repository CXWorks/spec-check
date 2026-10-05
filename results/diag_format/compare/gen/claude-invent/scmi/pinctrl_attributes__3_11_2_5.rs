pub open spec fn pinctrl_attributes__3_11_2_5_spec(identifier: UInt32, flags: UInt32, status: ScmiStatus, attributes: UInt32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!PinctrlExists(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int) ==> status == NOT_FOUND)
    && (new_s == old_s)
    && ((((flags & 3u32) == 0u32) || ((flags & 3u32) == 1u32) || ((flags & 3u32) == 2u32))
        && PinctrlExists(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int)
        ==> (
            status == SUCCESS
            && (((attributes >> 31u32) & 1u32) == 1u32
                <==> PinctrlNameIsExtended(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int))
            && ((attributes >> 18u32) & 0x1FFFu32) == 0u32
            && ((flags & 3u32) == 2u32 ==>
                ((((attributes >> 17u32) & 1u32) == 1u32)
                    <==> PinctrlFunctionSupportsGpio(old_s, (identifier & 0xFFFFu32) as int)))
            && ((flags & 3u32) == 2u32 ==>
                ((((attributes >> 16u32) & 1u32) == 1u32)
                    <==> PinctrlFunctionIsPinOnly(old_s, (identifier & 0xFFFFu32) as int)))
            && ((flags & 3u32) == 0u32 ==> (attributes & 0xFFFFu32) == 1u32)
            && ((flags & 3u32) == 1u32 ==>
                (attributes & 0xFFFFu32) as int == PinctrlGroupPinCount(old_s, (identifier & 0xFFFFu32) as int))
            && (((flags & 3u32) == 2u32 && ((attributes >> 16u32) & 1u32) == 1u32) ==>
                (attributes & 0xFFFFu32) as int == PinctrlFunctionPinCount(old_s, (identifier & 0xFFFFu32) as int))
            && (((flags & 3u32) == 2u32 && ((attributes >> 16u32) & 1u32) == 0u32) ==>
                (attributes & 0xFFFFu32) as int == PinctrlFunctionGroupCount(old_s, (identifier & 0xFFFFu32) as int))
            && name.len() == 16
            && (!PinctrlNameIsExtended(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int) ==> (
                PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int).len() < 16
                && (forall|i: int| 0 <= i < PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int).len() ==>
                    name[i] == PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int)[i])
                && name[PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int).len() as int] == 0u8))
            && (PinctrlNameIsExtended(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int) ==> (
                PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int).len() >= 15
                && (forall|i: int| 0 <= i < 15 ==>
                    name[i] == PinctrlName(old_s, (flags & 3u32) as int, (identifier & 0xFFFFu32) as int)[i])
                && name[15] == 0u8))
        ))
}

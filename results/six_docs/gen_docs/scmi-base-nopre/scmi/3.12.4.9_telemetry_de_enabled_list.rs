pub open spec fn 3.12.4.9_telemetry_de_enabled_list_spec(result: int32, flags: uint32, array: {uint32, uint32}, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (flags & 0xFFFF == 0 && (flags >> 16) == 0))
    && (result == 0 ==> (array.len() as int == (flags & 0xFFFF) as int))
    && (result == 0 ==> (forall i: int | 0 <= i && i < array.len() ==> (array[i].1 & 0x3) == 1 || (array[i].1 & 0x3) == 2))
    && (result == 0 ==> (forall i: int | 0 <= i && i < array.len() ==> (array[i].1 & 0xFFFFFFFC) == 0))
    && (result == 0 ==> (forall i: int | 0 <= i && i < array.len() ==> (array[i].0 >= 0)))
    && (result != 0 ==> (result == -1 || result == -2))
    && (result != 0 ==> (array.len() == 0))
    && (result != 0 ==> (flags == 0))
    && (result != 0 ==> (forall i: int | 0 <= i && i < array.len() ==> (array[i].1 == 0)))
    && (result != 0 ==> (forall i: int | 0 <= i && i < array.len() ==> (array[i].0 == 0)))
    && (result == 0 ==> (new_s == old_s))
}
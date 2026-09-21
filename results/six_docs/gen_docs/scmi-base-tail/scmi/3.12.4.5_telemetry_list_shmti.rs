pub open spec fn telemetry_list_shmti_spec(result: int32, num_SHMTI: uint32, SHMTI_desc: array<array<uint32>, 5>, index: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (num_SHMTI >= 0 && SHMTI_desc.len() == num_SHMTI as int))
    && (result == SCMI_NOT_FOUND ==> (num_SHMTI == 0 && SHMTI_desc.len() == 0))
    && (result == SCMI_NOT_SUPPORTED ==> (num_SHMTI == 0 && SHMTI_desc.len() == 0))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_NOT_SUPPORTED ==> (num_SHMTI == 0 && SHMTI_desc.len() == 0))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][0] as int >= 0)
        && (SHMTI_desc[i][1] as int >= 0)
        && (SHMTI_desc[i][2] as int >= 0)
        && (SHMTI_desc[i][3] as int >= 0)
        && (SHMTI_desc[i][4] == 0))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) % 8 == 0)
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) < (old_s as int) || (SHMTI_desc[i][1] as int) >= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][2] as int) < (old_s as int) || (SHMTI_desc[i][2] as int) >= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][3] as int) > 0)
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMTI_desc[i][3] as int) <= (old_s as int))
    && (forall i: int | 0 <= i && i < num_SHMTI as int ==>
        (SHMTI_desc[i][1] as int) + (SHMT
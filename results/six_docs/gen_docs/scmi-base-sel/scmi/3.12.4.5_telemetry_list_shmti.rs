pub open spec fn telemetry_list_shmti_spec(result: int, num_SHMTI: uint32, SHMTI_desc: [uint32; 5], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (num_SHMTI == 0 || (num_SHMTI as int) <= 65535 && (num_SHMTI as int) >= 0))
    && (result == SCMI_NOT_FOUND ==> (num_SHMTI == 0))
    && (result == SCMI_NOT_SUPPORTED ==> (num_SHMTI == 0))
    && (result == SCMI_SUCCESS ==> (SHMTI_desc.len() == 5))
    && (result == SCMI_SUCCESS ==> (SHMTI_desc[4] == 0))
    && (result != SCMI_SUCCESS ==> (num_SHMTI == 0))
    && (result != SCMI_SUCCESS ==> (SHMTI_desc[0] == 0))
    && (result != SCMI_SUCCESS ==> (SHMTI_desc[1] == 0))
    && (result != SCMI_SUCCESS ==> (SHMTI_desc[2] == 0))
    && (result != SCMI_SUCCESS ==> (SHMTI_desc[3] == 0))
    && (result != SCMI_SUCCESS ==> (SHMTI_desc[4] == 0))
}
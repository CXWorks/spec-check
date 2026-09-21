pub open spec fn psci_set_suspend_mode_spec(mode: int, result: int, old_s: S, new_s: S) -> bool {
  (result == PSCI_INVALID_PARAMETERS ==> result == PSCI_INVALID_PARAMETERS)
  && (result == PSCI_DENIED ==> result == PSCI_DENIED)
  && ((!( (cpu_state[0] == 0 || cpu_state[0] == 1 || cpu_state[0] == 2) &&
        (cpu_state[1] == 0 || cpu_state[1] == 1 || cpu_state[1] == 2) &&
        (cpu_state[2] == 0 || cpu_state[2] == 1 || cpu_state[2] == 2) &&
        (cpu_state[3] == 0 || cpu_state[3] == 1 || cpu_state[3] == 2) &&
        (cpu_state[4] == 0 || cpu_state[4] == 1 || cpu_state[4] == 2) &&
        (cpu_state[5] == 0 || cpu_state[5] == 1 || cpu_state[5] == 2) &&
        (cpu_state[6] == 0 || cpu_state[6] == 1 || cpu_state[6] == 2) &&
        (cpu_state[7] == 0 || cpu_state[7] == 1 || cpu_state[7] == 2) &&
        (cpu_state[8] == 0 || cpu_state[8] == 1 || cpu_state[8] == 2) &&
        (cpu_state[9] == 0 || cpu_state[9] == 1 || cpu_state[9] == 2) &&
        (cpu_state[10] == 0 || cpu_state[10] == 1 || cpu_state[10] == 2) &&
        (cpu_state[11] == 0 || cpu_state[11] == 1 || cpu_state[11] == 2) &&
        (cpu_state[12] == 0 || cpu_state[12] == 1 || cpu_state[12] == 2) &&
        (cpu_state[13] == 0 || cpu_state[13] == 1 || cpu_state[13] == 2) &&
        (cpu_state[14] == 0 || cpu_state[14] == 1 || cpu_state[14] == 2) &&
        (cpu_state[15] == 0 || cpu_state[15] == 1 || cpu_state[15] == 2) &&
        (cpu_state[16] == 0 || cpu_state[16] == 1 || cpu_state[16] == 2) &&
        (cpu_state[17] == 0 || cpu_state[17] == 1 || cpu_state[17] == 2) &&
        (cpu_state[18] == 0 || cpu_state[18] == 1 || cpu_state[18] == 2) &&
        (cpu_state[19] == 0 || cpu_state[19] == 1 || cpu_state[19] == 2) &&
        (cpu_state[20] == 0 || cpu_state[20] == 1 || cpu_state[20] == 2) &&
        (cpu_state[21] == 0 || cpu_state[21] == 1 || cpu_state[21] == 2) &&
        (cpu_state[22] == 0 || cpu_state[22] == 1 || cpu_state[22] == 2) &&
        (cpu_state[23] == 0 || cpu_state[23] == 1 || cpu_state[23] == 2) &&
        (cpu_state[24] == 0 || cpu_state[24] == 1 || cpu_state[24] == 2) &&
        (cpu_state[25] == 0 || cpu_state[25] == 1 || cpu_state[25] == 2) &&
        (cpu_state[26] == 0 || cpu_state[26] == 1 || cpu_state[26] == 2) &&
        (cpu_state[27] == 0 || cpu_state[27] == 1 || cpu_state[27] == 2) &&
        (cpu_state[28] == 0 || cpu_state[28] == 1 || cpu_state[28] == 2) &&
        (cpu_state[29] == 0 || cpu_state[29] == 1 || cpu_state[29] == 2) &&
        (cpu_state[30] == 0 || cpu_state[30] == 1 || cpu_state[30] == 2) &&
        (cpu_state[31] == 0 || cpu_state[31] == 1 || cpu_state[31] == 2) &&
        (cpu_state[32] == 0 || cpu_state[32] == 1 || cpu_state[32] == 2) &&
        (cpu_state[33] == 0 || cpu_state[33] == 1 || cpu_state[33] == 2) &&
        (cpu_state[34] == 0 || cpu_state[34] == 1 || cpu_state[34] == 2) &&
        (cpu_state[35] == 0 || cpu_state[35] == 1 || cpu_state[35] == 2) &&
        (cpu_state[36] == 0 || cpu_state[36] == 1 || cpu_state[36] == 2) &&
        (cpu_state[37] == 0 || cpu_state[37] == 1 || cpu_state[37] == 2) &&
        (cpu_state[38] == 0 || cpu_state[38] == 1 || cpu_state[38] == 2) &&
        (cpu_state[39] == 0 || cpu_state[39] == 1 || cpu_state[39] == 2) &&
        (cpu_state[40] == 0 || cpu_state[40] == 1 || cpu_state[40] == 2) &&
        (cpu_state[41] == 0 || cpu_state[41] == 1 || cpu_state[41] == 2) &&
        (cpu_state[42] == 0 || cpu_state[42] == 1 || cpu_state[42] == 2) &&
        (cpu_state[43] == 0 || cpu_state[43] == 1 || cpu_state[43] == 2) &&
        (cpu_state[44] == 0 || cpu_state[44] == 1 || cpu_state[44] == 2) &&
        (cpu_state[45] == 0 || cpu_state[45] == 1 || cpu_state[45] == 2) &&
        (cpu_state[46] == 0 || cpu_state[46] == 1 || cpu_state[46] == 2) &&
        (cpu_state[47] == 0 || cpu_state[47] == 1 || cpu_state[47] == 2) &&
        (cpu_state[48] == 0 || cpu_state[48] == 1 || cpu_state[48] == 2) &&
        (cpu_state[49] == 0 || cpu_state[49] == 1 || cpu_state[49] == 2) &&
        (cpu_state[50] == 0 || cpu_state[50] == 1 || cpu_state[50] == 2) &&
        (cpu_state[51] == 0 || cpu_state[51] == 1 || cpu_state[51] == 2) &&
        (cpu_state[52] == 0 || cpu_state[52] == 1 || cpu_state[52] == 2) &&
        (cpu_state[53] == 0 || cpu_state[53] == 1 || cpu_state[53] == 2) &&
        (cpu_state[54] == 0 || cpu_state[54] == 1 || cpu_state[54] == 2) &&
        (cpu_state[55] == 0 || cpu_state[55] == 1 || cpu_state[55] == 2) &&
        (cpu_state[56] == 0 || cpu_state[56] == 1 || cpu_state[56] == 2) &&
        (cpu_state[57] == 0 || cpu_state[57] == 1 || cpu_state[57] == 2) &&
        (cpu_state[58] == 0 || cpu_state[58] == 1 || cpu_state[58] == 2) &&
        (cpu_state[59] == 0 || cpu_state[59] == 1 || cpu_state[59] == 2) &&
        (cpu_state[60] == 0 || cpu_state[60] == 1 || cpu_state[60] == 2) &&
        (cpu_state[61] == 0 || cpu_state[61] == 1 || cpu_state[61] == 2) &&
        (cpu_state[62] == 0 || cpu_state[62] == 1 || cpu_state[62] == 2) &&
        (cpu_state[63] == 0 || cpu_state[63] == 1 || cpu_state[63] == 2))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS && result != PSCI_INVALID_PARAMETERS && result != PSCI_DENIED
    ==> cpu_state == cpu_state)
}
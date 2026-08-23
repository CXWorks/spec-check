pub open spec fn psci_set_suspend_mode_spec(mode: UInt32, result: int, old_s: S, new_s: S) -> bool {
  (result != PSCI_SUCCESS ==> result == PSCI_NOT_SUPPORTED)
  && (result != PSCI_SUCCESS ==> result == PSCI_INVALID_PARAMETERS)
  && (result != PSCI_SUCCESS ==> result == PSCI_DENIED)
  && ((!(CpuIsValid(old_s, 0) && (CpuIsOn(old_s, 0) || CpuIsOnPending(old_s, 0) || (cpu_state[0] == 0))) &&
       !(CpuIsValid(old_s, 1) && (CpuIsOn(old_s, 1) || CpuIsOnPending(old_s, 1) || (cpu_state[1] == 0))) &&
       !(CpuIsValid(old_s, 2) && (CpuIsOn(old_s, 2) || CpuIsOnPending(old_s, 2) || (cpu_state[2] == 0))) &&
       !(CpuIsValid(old_s, 3) && (CpuIsOn(old_s, 3) || CpuIsOnPending(old_s, 3) || (cpu_state[3] == 0))) &&
       !(CpuIsValid(old_s, 4) && (CpuIsOn(old_s, 4) || CpuIsOnPending(old_s, 4) || (cpu_state[4] == 0))) &&
       !(CpuIsValid(old_s, 5) && (CpuIsOn(old_s, 5) || CpuIsOnPending(old_s, 5) || (cpu_state[5] == 0))) &&
       !(CpuIsValid(old_s, 6) && (CpuIsOn(old_s, 6) || CpuIsOnPending(old_s, 6) || (cpu_state[6] == 0))) &&
       !(CpuIsValid(old_s, 7) && (CpuIsOn(old_s, 7) || CpuIsOnPending(old_s, 7) || (cpu_state[7] == 0))) &&
       !(CpuIsValid(old_s, 8) && (CpuIsOn(old_s, 8) || CpuIsOnPending(old_s, 8) || (cpu_state[8] == 0))) &&
       !(CpuIsValid(old_s, 9) && (CpuIsOn(old_s, 9) || CpuIsOnPending(old_s, 9) || (cpu_state[9] == 0))) &&
       !(CpuIsValid(old_s, 10) && (CpuIsOn(old_s, 10) || CpuIsOnPending(old_s, 10) || (cpu_state[10] == 0))) &&
       !(CpuIsValid(old_s, 11) && (CpuIsOn(old_s, 11) || CpuIsOnPending(old_s, 11) || (cpu_state[11] == 0))) &&
       !(CpuIsValid(old_s, 12) && (CpuIsOn(old_s, 12) || CpuIsOnPending(old_s, 12) || (cpu_state[12] == 0))) &&
       !(CpuIsValid(old_s, 13) && (CpuIsOn(old_s, 13) || CpuIsOnPending(old_s, 13) || (cpu_state[13] == 0))) &&
       !(CpuIsValid(old_s, 14) && (CpuIsOn(old_s, 14) || CpuIsOnPending(old_s, 14) || (cpu_state[14] == 0))) &&
       !(CpuIsValid(old_s, 15) && (CpuIsOn(old_s, 15) || CpuIsOnPending(old_s, 15) || (cpu_state[15] == 0))) &&
       !(CpuIsValid(old_s, 16) && (CpuIsOn(old_s, 16) || CpuIsOnPending(old_s, 16) || (cpu_state[16] == 0))) &&
       !(CpuIsValid(old_s, 17) && (CpuIsOn(old_s, 17) || CpuIsOnPending(old_s, 17) || (cpu_state[17] == 0))) &&
       !(CpuIsValid(old_s, 18) && (CpuIsOn(old_s, 18) || CpuIsOnPending(old_s, 18) || (cpu_state[18] == 0))) &&
       !(CpuIsValid(old_s, 19) && (CpuIsOn(old_s, 19) || CpuIsOnPending(old_s, 19) || (cpu_state[19] == 0))) &&
       !(CpuIsValid(old_s, 20) && (CpuIsOn(old_s, 20) || CpuIsOnPending(old_s, 20) || (cpu_state[20] == 0))) &&
       !(CpuIsValid(old_s, 21) && (CpuIsOn(old_s, 21) || CpuIsOnPending(old_s, 21) || (cpu_state[21] == 0))) &&
       !(CpuIsValid(old_s, 22) && (CpuIsOn(old_s, 22) || CpuIsOnPending(old_s, 22) || (cpu_state[22] == 0))) &&
       !(CpuIsValid(old_s, 23) && (CpuIsOn(old_s, 23) || CpuIsOnPending(old_s, 23) || (cpu_state[23] == 0))) &&
       !(CpuIsValid(old_s, 24) && (CpuIsOn(old_s, 24) || CpuIsOnPending(old_s, 24) || (cpu_state[24] == 0))) &&
       !(CpuIsValid(old_s, 25) && (CpuIsOn(old_s, 25) || CpuIsOnPending(old_s, 25) || (cpu_state[25] == 0))) &&
       !(CpuIsValid(old_s, 26) && (CpuIsOn(old_s, 26) || CpuIsOnPending(old_s, 26) || (cpu_state[26] == 0))) &&
       !(CpuIsValid(old_s, 27) && (CpuIsOn(old_s, 27) || CpuIsOnPending(old_s, 27) || (cpu_state[27] == 0))) &&
       !(CpuIsValid(old_s, 28) && (CpuIsOn(old_s, 28) || CpuIsOnPending(old_s, 28) || (cpu_state[28] == 0))) &&
       !(CpuIsValid(old_s, 29) && (CpuIsOn(old_s, 29) || CpuIsOnPending(old_s, 29) || (cpu_state[29] == 0))) &&
       !(CpuIsValid(old_s, 30) && (CpuIsOn(old_s, 30) || CpuIsOnPending(old_s, 30) || (cpu_state[30] == 0))) &&
       !(CpuIsValid(old_s, 31) && (CpuIsOn(old_s, 31) || CpuIsOnPending(old_s, 31) || (cpu_state[31] == 0))) &&
       !(CpuIsValid(old_s, 32) && (CpuIsOn(old_s, 32) || CpuIsOnPending(old_s, 32) || (cpu_state[32] == 0))) &&
       !(CpuIsValid(old_s, 33) && (CpuIsOn(old_s, 33) || CpuIsOnPending(old_s, 33) || (cpu_state[33] == 0))) &&
       !(CpuIsValid(old_s, 34) && (CpuIsOn(old_s, 34) || CpuIsOnPending(old_s, 34) || (cpu_state[34] == 0))) &&
       !(CpuIsValid(old_s, 35) && (CpuIsOn(old_s, 35) || CpuIsOnPending(old_s, 35) || (cpu_state[35] == 0))) &&
       !(CpuIsValid(old_s, 36) && (CpuIsOn(old_s, 36) || CpuIsOnPending(old_s, 36) || (cpu_state[36] == 0))) &&
       !(CpuIsValid(old_s, 37) && (CpuIsOn(old_s, 37) || CpuIsOnPending(old_s, 37) || (cpu_state[37] == 0))) &&
       !(CpuIsValid(old_s, 38) && (CpuIsOn(old_s, 38) || CpuIsOnPending(old_s, 38) || (cpu_state[38] == 0))) &&
       !(CpuIsValid(old_s, 39) && (CpuIsOn(old_s, 39) || CpuIsOnPending(old_s, 39) || (cpu_state[39] == 0))) &&
       !(CpuIsValid(old_s, 40) && (CpuIsOn(old_s, 40) || CpuIsOnPending(old_s, 40) || (cpu_state[40] == 0))) &&
       !(CpuIsValid(old_s, 41) && (CpuIsOn(old_s, 41) || CpuIsOnPending(old_s, 41) || (cpu_state[41] == 0))) &&
       !(CpuIsValid(old_s, 42) && (CpuIsOn(old_s, 42) || CpuIsOnPending(old_s, 42) || (cpu_state[42] == 0))) &&
       !(CpuIsValid(old_s, 43) && (CpuIsOn(old_s, 43) || CpuIsOnPending(old_s, 43) || (cpu_state[43] == 0))) &&
       !(CpuIsValid(old_s, 44) && (CpuIsOn(old_s, 44) || CpuIsOnPending(old_s, 44) || (cpu_state[44] == 0))) &&
       !(CpuIsValid(old_s, 45) && (CpuIsOn(old_s, 45) || CpuIsOnPending(old_s, 45) || (cpu_state[45] == 0))) &&
       !(CpuIsValid(old_s, 46) && (CpuIsOn(old_s, 46) || CpuIsOnPending(old_s, 46) || (cpu_state[46] == 0))) &&
       !(CpuIsValid(old_s, 47) && (CpuIsOn(old_s, 47) || CpuIsOnPending(old_s, 47) || (cpu_state[47] == 0))) &&
       !(CpuIsValid(old_s, 48) && (CpuIsOn(old_s, 48) || CpuIsOnPending(old_s, 48) || (cpu_state[48] == 0))) &&
       !(CpuIsValid(old_s, 49) && (CpuIsOn(old_s, 49) || CpuIsOnPending(old_s, 49) || (cpu_state[49] == 0))) &&
       !(CpuIsValid(old_s, 50) && (CpuIsOn(old_s, 50) || CpuIsOnPending(old_s, 50) || (cpu_state[50] == 0))) &&
       !(CpuIsValid(old_s, 51) && (CpuIsOn(old_s, 51) || CpuIsOnPending(old_s, 51) || (cpu_state[51] == 0))) &&
       !(CpuIsValid(old_s, 52) && (CpuIsOn(old_s, 52) || CpuIsOnPending(old_s, 52) || (cpu_state[52] == 0))) &&
       !(CpuIsValid(old_s, 53) && (CpuIsOn(old_s, 53) || CpuIsOnPending(old_s, 53) || (cpu_state[53] == 0))) &&
       !(CpuIsValid(old_s, 54) && (CpuIsOn(old_s, 54) || CpuIsOnPending(old_s, 54) || (cpu_state[54] == 0))) &&
       !(CpuIsValid(old_s, 55) && (CpuIsOn(old_s, 55) || CpuIsOnPending(old_s, 55) || (cpu_state[55] == 0))) &&
       !(CpuIsValid(old_s, 56) && (CpuIsOn(old_s, 56) || CpuIsOnPending(old_s, 56) || (cpu_state[56] == 0))) &&
       !(CpuIsValid(old_s, 57) && (CpuIsOn(old_s, 57) || CpuIsOnPending(old_s, 57) || (cpu_state[57] == 0))) &&
       !(CpuIsValid(old_s, 58) && (CpuIsOn(old_s, 58) || CpuIsOnPending(old_s, 58) || (cpu_state[58] == 0))) &&
       !(CpuIsValid(old_s, 59) && (CpuIsOn(old_s, 59) || CpuIsOnPending(old_s, 59) || (cpu_state[59] == 0))) &&
       !(CpuIsValid(old_s, 60) && (CpuIsOn(old_s, 60) || CpuIsOnPending(old_s, 60) || (cpu_state[60] == 0))) &&
       !(CpuIsValid(old_s, 61) && (CpuIsOn(old_s, 61) || CpuIsOnPending(old_s, 61) || (cpu_state[61] == 0))) &&
       !(CpuIsValid(old_s, 62) && (CpuIsOn(old_s, 62) || CpuIsOnPending(old_s, 62) || (cpu_state[62] == 0))) &&
       !(CpuIsValid(old_s, 63) && (CpuIsOn(old_s, 63) || CpuIsOnPending(old_s, 63) || (cpu_state[63] == 0))))
    ==> result == PSCI_DENIED)
  && ((result == PSCI_SUCCESS)
    ==> true)
}
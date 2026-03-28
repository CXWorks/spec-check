pub enum RmiStatusCode {
  RMI_ERROR_INPUT,
  RMI_ERROR_REALM(int),
  RMI_ERROR_REC,
  RMI_ERROR_RTT(int),
}

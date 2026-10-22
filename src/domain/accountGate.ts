export type AccountGateStatus = { loggedIn:boolean; storageReady?:boolean; supported?:boolean };
export function accountGate(status:AccountGateStatus|null, failed=false) {
  if (!status) return failed?'unavailable':'loading';
  if (status.storageReady===false || status.supported===false) return 'blocked';
  return status.loggedIn?'ready':'visitor';
}

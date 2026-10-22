export type NotificationPermission = {
  supported: boolean;
  authorization: string;
  alerts: boolean | null;
  center: boolean | null;
};
export function permissionMessage(p: NotificationPermission | null): string {
  if (!p) return '系统通知权限尚未读取。';
  if (!p.supported) return '当前平台暂不能读取系统授权状态，请在系统设置中确认。';
  if (p.authorization === 'not-determined') return '尚未申请系统通知权限；后台采集不会自动弹出授权提示。';
  if (p.authorization === 'denied') return '系统通知已关闭，请在系统设置 → 通知 → TokenDaily 中开启。';
  if (['authorized', 'provisional'].includes(p.authorization)) {
    if (p.alerts === false && p.center === false) return '通知已授权，但横幅和通知中心均已关闭。';
    if (p.authorization === 'provisional' || p.alerts === false) return '系统允许静默通知；请查看通知中心，可能不会显示横幅。';
    if (p.alerts === true) return '系统允许通知；横幅是否出现仍受专注模式等系统设置影响。';
  }
  return '系统通知权限或显示方式未知，不能确认能否提醒。';
}
export function testResultMessage(result: string): string {
  return ({
    accepted: '系统已接受测试请求，尚不能确认横幅显示；可检查通知中心记录。',
    queued: '测试请求已交给通知组件；当前平台不能确认系统接受或显示。',
    'in-center': '系统通知中心中查到本次测试通知；这不证明横幅出现或已被阅读。',
    clicked: '已收到本次测试通知的点击回调，并请求打开面板。',
    'not-observed': '通知中心暂未查到本次测试；可能尚未显示或已移除，不能据此判定失败。',
    none: '本次运行还没有可检查的测试通知。',
    unsupported: '当前平台不支持查询通知中心记录。',
  } as Record<string, string>)[result] ?? '测试通知结果未知，请检查通知中心。';
}

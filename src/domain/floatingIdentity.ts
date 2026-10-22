type Identity = { loggedIn:boolean; remote:{ enrollment:{nickname?:string}|null;account?:{accountId?:string;nickname?:string|null}|null }|null };
export const IDENTITY_REQUEST_KEY='tokendaily.open-identity';
export function floatingIdentity(status:Identity|null) {
  if(!status?.loggedIn)return {label:'游客',avatar:null,title:'游客模式 · 点击管理设备身份；不会自动参榜或上传'};
  const account=status.remote?.account;
  const name=(account?.nickname??status.remote?.enrollment?.nickname)?.trim();
  return {label:name||(account?.accountId?'官网账号':'设备身份'),avatar:name?Array.from(name)[0].toLocaleUpperCase():null,
    title:account?.accountId?`${name||'官网账号'} · 多设备用量已连接，点击管理账号与参榜`:name?`${name} · 本机游客身份，点击管理参榜`:'本机游客身份 · 点击查看状态'};
}

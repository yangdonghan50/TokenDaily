import { rankHonor } from './leaderboardHonors.ts';
import { beijingToday } from './localUsage.ts';
import { formatTokenCount } from './tokenFormat.ts';
import type { UsageState } from './usage.ts';
export type RankStatus = {
  supported:boolean; loggedIn:boolean; enabled:boolean; thisDevice:string;
  remote:{enrollment:{device:string;active:number;held:number;nickname?:string}|null;account?:{publicId?:string|null;boardHeld?:number|null;nickname?:string|null}|null;board:{period?:string;tool?:string;start?:string;end?:string;honorTotal?:number;me:{rank:number|null;rankEnd?:number;tokens?:number}|null}|null}|null;
};
export function rankPresentation(status:RankStatus|null,now:number) {
  if (!status?.supported || !status.loggedIn || !status.remote || (!status.remote.enrollment && !status.remote.account?.publicId)) return null;
  const board=status.remote.board;
  const day=beijingToday(now);
  if(!board || board.period!=='day' || board.tool!=='all' || board.start!==day || board.end!==day)return null;
  const held=status.remote.account?.publicId?status.remote.account.boardHeld:status.remote.enrollment?.held;
  const me=status.remote.board?.me;
  const rank=me?.rank;
  const tokens=me?.tokens??0;
  const ranked=Number.isSafeInteger(rank)&&rank!>0&&Number.isSafeInteger(tokens)&&tokens>0;
  const honor=held?null:rankHonor(board.me,board.honorTotal);
  const honorNote=honor?`今日荣誉：${honor.name}${honor.limit?' · 前 '+honor.limit+'%':''}。`:'';
  return {honor,label:held?'日榜 待核查':ranked?`日榜 #${rank!.toLocaleString('en-US')}`:'日榜 暂无名次',
    value:!held&&ranked?`#${rank!.toLocaleString('en-US')}`:'—',note:held?'待核查':ranked?'今日 · 多工具':'暂无名次',
    title:honorNote+`TokenDaily 多工具总榜当日排名 · ${day} · 北京时间 · ${status.remote.account?.publicId?'同一官网账号下多设备':'游客单设备'}已同步的 Codex、Claude Code、WorkBuddy Token 用量；本期没有有效用量则不排名。TRAE 尚未计入云端总榜。`};
}
export function costPresentation(sources:UsageState[],now:number) {
  let usd=0,priced=0,unpriced=0,missing=false,stale=false,known=0;
  for (const source of sources) {
    const snapshot=source.snapshot;
    const day=snapshot?.days.find(d=>d.date===beijingToday(now));
    if(!day || !Number.isSafeInteger(day.tokens) || day.tokens<0){missing=true;continue;}
    known++;stale ||= source.status!=='available' || source.lastSuccessAtMs===null || now-source.lastSuccessAtMs>90_000 || snapshot!.collectedAtMs>now+5_000;
    const cost=snapshot?.apiCost;
    if (cost && snapshot!.days.length===1 && Number.isFinite(cost.usd) && cost.usd>=0 && Number.isSafeInteger(cost.pricedTokens) && cost.pricedTokens>=0
      && Number.isSafeInteger(cost.unpricedTokens) && cost.unpricedTokens>=0 && cost.pricedTokens+cost.unpricedTokens===day.tokens) {
      usd+=cost.usd;priced+=cost.pricedTokens;unpriced+=cost.unpricedTokens;
    }else{unpriced+=day.tokens;}
  }
  const partial=missing||unpriced>0;
  const valid=Number.isFinite(usd)&&Number.isSafeInteger(priced)&&Number.isSafeInteger(unpriced);
  const unavailable=!valid || !known || !priced && partial;
  const amount=unavailable?'—':usd===0?'$0.00':usd<.01?'<$0.01':`≈$${usd.toLocaleString('en-US',{minimumFractionDigits:2,maximumFractionDigits:2})}`;
  const qualifiers=`${partial?' · 部分':''}${stale?' · 上次':''}`;
  const value=unavailable?'待估价':`${amount}${qualifiers}`;
  const note=unavailable?'待估价':`USD${qualifiers}`;
  const detail=partial?`已估价 ${formatTokenCount(priced)} Tokens；${formatTokenCount(unpriced)} Tokens 缺少已核验价格或计价字段${missing?'；还有来源未读取':''}。`:'已读取用量均可估价。';
  return {value,amount,note,title:`${value}。今日等效 API 预估费用（USD），非实际账单。标准速度、全球路由官方 Token 单价（2026-10-18 核验），不含工具费、税费及订阅费用。${detail}缺失部分不按 0 计算；不从额度或积分推算。${stale?'含旧记录，等待刷新。':''}`};
}

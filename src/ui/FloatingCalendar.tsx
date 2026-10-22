import { calendarPresentation } from '../domain/floatingCalendar.ts';
import { type UsageState } from '../domain/usage.ts';
import { UsageHeatmap } from './UsageHeatmap.tsx';

export function FloatingCalendar({usage,loading,now}:{usage:UsageState;loading:boolean;now:number}) {
  const {cells,stale}=calendarPresentation(usage,now);
  const readAt=usage.lastSuccessAtMs===null?null:new Intl.DateTimeFormat('zh-CN',{timeZone:'Asia/Shanghai',hour:'2-digit',minute:'2-digit',hourCycle:'h23'}).format(usage.lastSuccessAtMs);
  const warning=!usage.snapshot?'暂无可靠记录':stale?'记录待更新':usage.message?'统计不完整':null;
  const detail=loading?'正在读取本机历史日志':`${warning??'本地记录合计'}；${readAt?`北京时间 ${readAt} 读取`:'尚未成功读取'}。${usage.message??'空白表示未记录。'}已连接的 TRAE 来自本地数据库，恢复/同步会话可能来自其他设备。使用面板右下角刷新。`;
  return <div className="floating-calendar" aria-label="本机总 Token 日历">
    {(loading||warning)&&<span className="calendar-mini-health" role="status" aria-label={loading?'日历读取中':`日历${warning}`} title={detail}>{loading?'…':'!'}</span>}
    <UsageHeatmap cells={cells} compact/>
  </div>;
}

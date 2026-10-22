import { formatTokenCount } from './tokenFormat.ts';
export type HeatmapDay = {date:string;tokens:number|null;future?:boolean};
export function heatmapLabel(day:HeatmapDay) {
  return `${day.date} · ${day.future?'未来日期':day.tokens===null?'本机日志未记录':`${formatTokenCount(day.tokens)} Tokens`}`;
}
export function heatmapMonths(cells:HeatmapDay[]) {
  let previous='';
  return cells.filter((_,index)=>index%7===0).map(day=>{
    const month=day.date.slice(0,7),label=month===previous?'':`${Number(day.date.slice(5,7))}月`;
    previous=month;return label;
  });
}

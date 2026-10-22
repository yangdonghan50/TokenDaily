import { useState, type CSSProperties } from 'react';
import { heatmapLabel, heatmapMonths, type HeatmapDay } from '../domain/usageHeatmap.ts';
import { usageLevel } from '../domain/usage.ts';
import './usageHeatmap.css';

// Hover/focus is transient information, never a selected date or a click action.
export function UsageHeatmap({cells,compact=false}:{cells:HeatmapDay[];compact?:boolean}) {
  const [hover,setHover]=useState<string|null>(null);
  const [focus,setFocus]=useState<string|null>(null);
  const active=cells.find(cell=>cell.date===(hover??focus));
  const max=Math.max(0,...cells.filter(cell=>!cell.future).map(cell=>cell.tokens??0));
  const weeks=Math.ceil(cells.length/7);
  return <div className={`usage-heatmap ${compact?'is-compact':''}`} style={{'--heat-columns':weeks} as CSSProperties} onMouseLeave={()=>setHover(null)}>
    <div className="heatmap-months" aria-hidden="true">{heatmapMonths(cells).map((month,index)=><span key={index}>{month}</span>)}</div>
    <div className="heatmap-body"><div className="heatmap-weekdays" aria-hidden="true"><span>一</span><span>三</span><span>五</span><span>日</span></div>
      <div className="heatmap-grid" role="group" aria-label={`最近 ${weeks} 周本机总 Token 日历`}>
        {cells.map(cell=><span key={cell.date} tabIndex={cell.future?undefined:0} role="img" aria-label={heatmapLabel(cell)} aria-hidden={cell.future||undefined}
          className={`heatmap-day heat-${usageLevel(cell.tokens,max)??'missing'} ${cell.future?'is-future':''}`}
          onMouseEnter={()=>setHover(cell.date)} onMouseLeave={()=>setHover(null)}
          onFocus={event=>{if(event.currentTarget.matches(':focus-visible'))setFocus(cell.date);}} onBlur={()=>setFocus(null)}/>)}
      </div></div>
    {active&&!active.future&&<div className="heatmap-tooltip" role="tooltip">{heatmapLabel(active)}</div>}
  </div>;
}

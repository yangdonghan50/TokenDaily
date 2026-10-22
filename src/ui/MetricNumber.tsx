import { useLayoutEffect, useRef, type ReactNode } from 'react';
import { metricFontSize } from '../domain/metricFit.ts';

export function MetricNumber({value,unit,addon}:{value:string;unit?:string;addon?:ReactNode}) {
  const root=useRef<HTMLDivElement>(null);
  useLayoutEffect(()=>{
    const row=root.current!;
    const number=row.querySelector('strong')!;
    const fit=()=>{
      number.style.fontSize='';
      const style=getComputedStyle(number);
      const suffixWidth=Array.from(row.children).filter(child=>child!==number).reduce((width,child)=>width+child.getBoundingClientRect().width+2,0);
      const available=row.clientWidth-suffixWidth;
      const base=parseFloat(style.fontSize);
      number.style.fontSize=`${metricFontSize(base,number.scrollWidth,available)}px`;
    };
    fit();
    const observer=new ResizeObserver(fit);observer.observe(row);
    let disposed=false;
    void document.fonts.ready.then(()=>{if(!disposed)fit();});
    return ()=>{disposed=true;observer.disconnect();};
  },[value,unit,addon]);
  return <div ref={root} className="floating-metric-value"><strong>{value}</strong>{unit&&<span className="floating-metric-unit">{unit}</span>}{addon}</div>;
}

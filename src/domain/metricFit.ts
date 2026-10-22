// Reserve one CSS pixel for WebKit's subpixel glyph rounding.
export function metricFontSize(base:number, measured:number, available:number):number {
  if (![base,measured,available].every(Number.isFinite)||base<=0||measured<=0||available<=1) return base;
  return Math.min(base,Math.floor(base*(available-1)/measured*100)/100);
}

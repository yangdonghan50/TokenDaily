export type FloatingHotkey = { key: number; modifiers: number };
const codes: Record<string,number> = { Backspace:8, Tab:9, Enter:13, Escape:27, Space:32, PageUp:33, PageDown:34, End:35, Home:36, ArrowLeft:37, ArrowUp:38, ArrowRight:39, ArrowDown:40, Insert:45, Delete:46, NumpadMultiply:106, NumpadAdd:107, NumpadSubtract:109, NumpadDecimal:110, NumpadDivide:111, Semicolon:186, Equal:187, Comma:188, Minus:189, Period:190, Slash:191, Backquote:192, BracketLeft:219, Backslash:220, BracketRight:221, Quote:222 };
const names: Record<number,string> = {8:'Backspace',9:'Tab',13:'Enter',27:'Esc',32:'Space',33:'Page Up',34:'Page Down',35:'End',36:'Home',37:'←',38:'↑',39:'→',40:'↓',45:'Insert',46:'Delete',106:'Num *',107:'Num +',109:'Num -',110:'Num .',111:'Num /',186:';',187:'=',188:',',189:'-',190:'.',191:'/',192:'`',219:'[',220:'\\',221:']',222:"'"};
export function hotkeyLabel(value: FloatingHotkey | null, platform:'windows'|'macos'='windows'): string {
  if (!value) return '未设置';
  const {key,modifiers}=value;
  const label=key>=112&&key<=135?`F${key-111}`:key>=96&&key<=105?`Num ${key-96}`:names[key]??String.fromCharCode(key);
  return [modifiers&2?'Ctrl':'',modifiers&1?(platform==='macos'?'Option':'Alt'):'',modifiers&4?'Shift':'',modifiers&8?(platform==='macos'?'⌘':'Win'):'',label].filter(Boolean).join(' + ');
}
export function captureHotkey(event: {code:string;ctrlKey:boolean;altKey:boolean;shiftKey:boolean;metaKey:boolean;repeat:boolean;isComposing:boolean}): FloatingHotkey | null {
  if(event.repeat||event.isComposing||/^(Control|Alt|Shift|Meta)(Left|Right)$/.test(event.code)) return null;
  let key=codes[event.code];
  if(/^Key[A-Z]$/.test(event.code)) key=event.code.charCodeAt(3);
  else if(/^Digit[0-9]$/.test(event.code)) key=event.code.charCodeAt(5);
  else if(/^Numpad[0-9]$/.test(event.code)) key=96+Number(event.code.slice(6));
  else if(/^F([1-9]|1[0-9]|2[0-4])$/.test(event.code)) key=111+Number(event.code.slice(1));
  if(key===123) throw new Error('F12 为系统保留键，请选择其他按键');
  if(key===undefined) throw new Error('暂不支持这个按键，请换一个普通键或功能键');
  return {key,modifiers:(event.ctrlKey?2:0)|(event.altKey?1:0)|(event.shiftKey?4:0)|(event.metaKey?8:0)};
}

// Keep aligned with TokenDaily-Web/public/leaderboard/honors.js.
export type RankHonor = {id:string;name:string;color:string;limit?:number};
const honors:RankHonor[] = [
  {id:'gold',name:'第 1 名',color:'#a77319'},
  {id:'silver',name:'第 2 名',color:'#687f94'},
  {id:'bronze',name:'第 3 名',color:'#a46f4e'},
  {id:'stellar',name:'星耀',color:'#7771b1',limit:5},
  {id:'elite',name:'精英',color:'#277f71',limit:10},
  {id:'progress',name:'进阶',color:'#6185a0',limit:25},
  {id:'rising',name:'新锐',color:'#718b6c',limit:50},
];
export function rankHonor(row:{rank:number|null;rankEnd?:number;tokens?:number}|null|undefined,total:number|undefined):RankHonor|null {
  if(!row || !Number.isSafeInteger(total) || total!<1 || !Number.isSafeInteger(row.tokens) || row.tokens!<=0
    || !Number.isSafeInteger(row.rank) || row.rank!<1 || !Number.isSafeInteger(row.rankEnd) || row.rankEnd!<row.rank! || row.rankEnd!>total!)return null;
  return row.rank!<=3 ? honors[row.rank!-1] : honors.slice(3).find(h=>row.rankEnd!*100<=total!*h.limit!)??null;
}

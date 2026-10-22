// A skipped native resize must survive pointer capture and an async older resize.
export function createLayoutRevision() {
  let requested=0, completed=0;
  return { request:()=>++requested, current:()=>requested, pending:()=>completed<requested,
    applied:(revision:number)=>{completed=Math.max(completed,revision);} };
}

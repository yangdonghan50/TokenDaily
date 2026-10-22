export function floatingRefreshState(canRefreshSources:boolean, sourcesBusy:boolean, calendarVisible:boolean, historyBusy:boolean) {
  return {enabled:canRefreshSources||calendarVisible,busy:sourcesBusy||calendarVisible&&historyBusy};
}

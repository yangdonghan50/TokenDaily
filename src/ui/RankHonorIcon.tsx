import type { RankHonor } from '../domain/leaderboardHonors.ts';
import gold from '../../assets/honors/gold.svg';
import silver from '../../assets/honors/silver.svg';
import bronze from '../../assets/honors/bronze.svg';
import stellar from '../../assets/honors/stellar.svg';
import elite from '../../assets/honors/elite.svg';
import progress from '../../assets/honors/progress.svg';
import rising from '../../assets/honors/rising.svg';
// Original user-supplied v3 vectors; static imports keep assets local and bundled.
const icons:Record<string,string>={gold,silver,bronze,stellar,elite,progress,rising};
export function RankHonorIcon({honor}:{honor:RankHonor}) {
  return <img className="floating-rank-honor" src={icons[honor.id]} alt="" aria-hidden="true" draggable={false}/>;
}

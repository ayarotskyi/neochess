import { statToColor } from '@/common';
import { useAnalyzerStore } from '@/store/analyzer';
import { useGameStore } from '@/store/game';
import { squareFile, squareRank } from 'chessops';
import { memo, useMemo, type SVGProps } from 'react';
import { useShallow } from 'zustand/shallow';

type Props = SVGProps<SVGSVGElement> & { statIndex: number };

const Arrow = ({ statIndex, ...props }: Props) => {
  const playAs = useGameStore((state) => state.playAs);
  const stat = useAnalyzerStore(
    useShallow((state) => state.moveStatistics![statIndex]),
  );
  const move = stat.move;
  const hovered = stat.hovered;

  const toRank = squareRank(move.to);
  const fromRank = squareRank(move.from);
  const toFile = squareFile(move.to);
  const fromFile = squareFile(move.from);

  const distance = Math.sqrt(
    Math.pow(toRank - fromRank, 2) + Math.pow(toFile - fromFile, 2),
  );
  const angle = Math.atan2(toRank - fromRank, toFile - fromFile);

  const position = useMemo(
    () =>
      playAs === 'white'
        ? { bottom: `${fromRank * 12.5 + 6.75}%`, left: `${fromFile * 12.5}%` }
        : {
            bottom: `${(7 - fromRank) * 12.5 + 6.75}%`,
            left: `${(7 - fromFile) * 12.5}%`,
          },
    [fromFile, fromRank, playAs],
  );
  return (
    <svg
      color={statToColor(stat)}
      style={{
        position: 'absolute',
        ...position,
        transform: `rotate(${playAs === 'white' ? Math.PI / 2 - angle : (3 * Math.PI) / 2 - angle}rad) scaleX(${0.5 + 0.5 * stat.playRate})`,
        transformOrigin: 'bottom center',
        zIndex: 2,
        filter: 'drop-shadow(currentColor 0px 0px 10px)',
        opacity: hovered ? 1 : 0.3 + 0.6 * stat.winRate,
      }}
      width="12.5%"
      height={`${12.5 * distance}%`}
      viewBox={`0 0 1 ${distance}`}
      pointerEvents="none"
      {...props}
    >
      <g>
        <polygon points={`0.5,0 1,0.6 0,0.6`} fill="currentColor" />
        <rect
          fill="currentColor"
          x={0.3}
          y={0.6}
          width={0.4}
          height={distance - 0.5}
        />
      </g>
    </svg>
  );
};

export default memo(Arrow);

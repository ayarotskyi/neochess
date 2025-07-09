import BlackBishop from '@/icons/BlackBishop';
import BlackKing from '@/icons/BlackKing';
import BlackKnight from '@/icons/BlackKnight';
import BlackPawn from '@/icons/BlackPawn';
import BlackQueen from '@/icons/BlackQueen';
import BlackRook from '@/icons/BlackRook';
import WhiteBishop from '@/icons/WhiteBishop';
import WhiteKing from '@/icons/WhiteKing';
import WhiteKnight from '@/icons/WhiteKnight';
import WhitePawn from '@/icons/WhitePawn';
import WhiteQueen from '@/icons/WhiteQueen';
import WhiteRook from '@/icons/WhiteRook';
import { makeUci, type Color, type Role } from 'chessops';
import type { SVGProps } from 'react';
import type { MoveStat } from './store/analyzer';

export const stringToColor = (str: string) => {
  let hash = 0;
  str.split('').forEach((char) => {
    hash = char.charCodeAt(0) + ((hash << 5) - hash);
  });
  let color = '#';
  for (let i = 0; i < 3; i++) {
    const value = (hash >> (i * 8)) & 0xff;
    color += value.toString(16).padStart(2, '0');
  }
  return color;
};

export const statToColor = (stat: MoveStat) =>
  stringToColor(makeUci(stat.move) + stat.playRate + stat.winRate);

export type ColorRole = `${Color}_${Role}`;

export const PieceComponents: Record<
  ColorRole,
  React.FC<SVGProps<SVGSVGElement>>
> = {
  white_bishop: WhiteBishop,
  white_king: WhiteKing,
  white_knight: WhiteKnight,
  white_pawn: WhitePawn,
  white_queen: WhiteQueen,
  white_rook: WhiteRook,
  black_bishop: BlackBishop,
  black_king: BlackKing,
  black_knight: BlackKnight,
  black_pawn: BlackPawn,
  black_queen: BlackQueen,
  black_rook: BlackRook,
};

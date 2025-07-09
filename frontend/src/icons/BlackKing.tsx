import * as React from 'react';
const BlackKing = (props: React.SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={128}
    height={128}
    viewBox="0 0 45 45"
    {...props}
  >
    <g stroke="#000" strokeLinejoin="round" strokeWidth={1.5}>
      <path fill="none" strokeLinecap="round" d="M22.5 11.63V6M20 8h5" />
      <path d="M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5" />
      <path d="M11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10V37" />
    </g>
    <g
      fill="none"
      stroke="#FFF"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={1.5}
    >
      <path
        strokeLinecap="square"
        d="M22.5 20s3.25-3 2-5.5c0 0-.5-1.5-2-1.5s-2 1.5-2 1.5c-1.25 2.5 2 5.5 2 5.5"
      />
      <path d="M32 29.5s8.5-4 6.03-9.65C34.15 14 25 18 22.5 24.5l.01 2.1-.01-2.1C20 18 9.906 14 6.997 19.85c-2.497 5.65 4.853 9 4.853 9" />
      <path d="M11.5 30c5.5-3 15.5-3 21 0M11.5 33.5c5.5-3 15.5-3 21 0M11.5 37c5.5-3 15.5-3 21 0" />
    </g>
  </svg>
);
export default BlackKing;

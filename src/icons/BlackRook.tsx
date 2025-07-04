import * as React from 'react';
const BlackRook = (props: React.SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={128}
    height={128}
    viewBox="0 0 45 45"
    {...props}
  >
    <g stroke="#000" strokeLinejoin="round" strokeWidth={1.5}>
      <path d="M11 14V9h4v2h5V9h5v2h5V9h4v5l-3 3v12.5l2 2.5v4H12v-4l2-2.5V17ZM9 39h27v-3H9v3z" />
    </g>
    <g fill="none" stroke="#FFF" strokeLinecap="round">
      <path strokeWidth={1.2} d="M11 14h23" />
      <path strokeWidth={0.8} d="M14 17h17M14 29.5h17" />
      <path strokeWidth={1.2} d="M12 32h21M12 35.5h21" />
    </g>
  </svg>
);
export default BlackRook;

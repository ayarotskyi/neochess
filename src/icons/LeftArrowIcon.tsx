import * as React from 'react';
const LeftArrowIcon = (props: React.SVGProps<SVGSVGElement>) => (
  <svg
    fill="none"
    stroke="currentColor"
    className="w-4 h-4 text-cyan-400"
    viewBox="0 0 24 24"
    {...props}
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      d="m15 19-7-7 7-7"
    />
  </svg>
);
export default LeftArrowIcon;

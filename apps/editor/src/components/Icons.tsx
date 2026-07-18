interface IconProps {
  size?: number;
  className?: string;
}

export function RectangleIcon({ size = 14, className }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="none"
      className={className}
      aria-hidden
    >
      <rect
        x="2.5"
        y="3.5"
        width="11"
        height="9"
        rx="1"
        stroke="currentColor"
        strokeWidth="1.25"
      />
    </svg>
  );
}

export function FrameIcon({ size = 14, className }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="none"
      className={className}
      aria-hidden
    >
      <path
        d="M3 5.5h1.5V3h2.5M9 3h2.5v2.5H14M14 9v2.5h-2.5V14M7 14H4.5v-2.5H3"
        stroke="currentColor"
        strokeWidth="1.25"
        strokeLinecap="square"
      />
      <rect x="5.5" y="5.5" width="5" height="5" stroke="currentColor" strokeWidth="1.25" />
    </svg>
  );
}

export function ShaderIcon({ size = 14, className }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="none"
      className={className}
      aria-hidden
    >
      <circle cx="8" cy="8" r="5.25" stroke="currentColor" strokeWidth="1.25" />
      <path
        d="M5 8.5c1.2-2 2.2-3 3-3s1.8 1 3 3"
        stroke="currentColor"
        strokeWidth="1.25"
        strokeLinecap="round"
      />
      <circle cx="8" cy="6.5" r="1" fill="currentColor" />
    </svg>
  );
}

export function LayerTypeIcon({
  type,
  size = 14,
}: {
  type: string;
  size?: number;
}) {
  switch (type) {
    case "Frame":
      return <FrameIcon size={size} />;
    case "Shader":
      return <ShaderIcon size={size} />;
    case "Rectangle":
    default:
      return <RectangleIcon size={size} />;
  }
}

import { HugeiconsIcon, type IconSvgElement } from "@hugeicons/react";
import {
  Cursor01Icon,
  EyeIcon,
  EyeOffIcon,
  FitToScreenIcon,
  FrameIcon as HugeFrameIcon,
  HandIcon,
  Layers01Icon,
  LockIcon,
  MinusSignIcon,
  PlusSignIcon,
  SlidersHorizontalIcon,
  SparklesIcon,
  SquareIcon,
  SquareUnlock01Icon,
  ZoomIcon,
} from "@hugeicons/core-free-icons";

const STROKE = 1.55;

interface IconProps {
  size?: number;
  className?: string;
}

function Icon({
  icon,
  size = 14,
  className,
}: IconProps & { icon: IconSvgElement }) {
  return (
    <HugeiconsIcon
      icon={icon}
      size={size}
      strokeWidth={STROKE}
      className={className}
      color="currentColor"
    />
  );
}

export function RectangleIcon(props: IconProps) {
  return <Icon icon={SquareIcon} {...props} />;
}

export function FrameIcon(props: IconProps) {
  return <Icon icon={HugeFrameIcon} {...props} />;
}

export function ShaderIcon(props: IconProps) {
  return <Icon icon={SparklesIcon} {...props} />;
}

export function EyeOpenIcon(props: IconProps) {
  return <Icon icon={EyeIcon} {...props} />;
}

export function EyeClosedIcon(props: IconProps) {
  return <Icon icon={EyeOffIcon} {...props} />;
}

export function LockedIcon(props: IconProps) {
  return <Icon icon={LockIcon} {...props} />;
}

export function UnlockedIcon(props: IconProps) {
  return <Icon icon={SquareUnlock01Icon} {...props} />;
}

export function ZoomMetaIcon(props: IconProps) {
  return <Icon icon={ZoomIcon} {...props} />;
}

export function ZoomInIcon(props: IconProps) {
  return <Icon icon={PlusSignIcon} {...props} />;
}

export function ZoomOutIcon(props: IconProps) {
  return <Icon icon={MinusSignIcon} {...props} />;
}

export function FitViewIcon(props: IconProps) {
  return <Icon icon={FitToScreenIcon} {...props} />;
}

export function HandToolIcon(props: IconProps) {
  return <Icon icon={HandIcon} {...props} />;
}

export function SelectToolIcon(props: IconProps) {
  return <Icon icon={Cursor01Icon} {...props} />;
}

export function LayersHeaderIcon(props: IconProps) {
  return <Icon icon={Layers01Icon} {...props} />;
}

export function PropertiesHeaderIcon(props: IconProps) {
  return <Icon icon={SlidersHorizontalIcon} {...props} />;
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

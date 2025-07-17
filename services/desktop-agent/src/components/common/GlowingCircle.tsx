/**
 * GlowingCircle Component - React Bit Pattern
 *
 * A reusable glowing circle component following React Bits patterns.
 * This separates animation logic from business logic and provides
 * configurable glow effects for status indicators.
 *
 * React Bits Pattern: Component Composition with Props Interface
 * Reference: https://reactbits.dev/
 */

import React from "react";

interface GlowingCircleProps {
  color: string;
  size?: "sm" | "md" | "lg";
  glowIntensity?: "low" | "medium" | "high";
  isPulsing?: boolean;
  animationType?: "pulse" | "slow-pulse" | "none";
  className?: string;
}

const sizeConfig = {
  sm: "w-2 h-2",
  md: "w-2.5 h-2.5",
  lg: "w-3 h-3",
};

const glowConfig = {
  low: "0 0 0 2px",
  medium: "0 0 0 3px",
  high: "0 0 0 4px",
};

const animationConfig = {
  pulse: "animate-pulse",
  "slow-pulse": "animate-slow-pulse",
  none: "",
};

export const GlowingCircle: React.FC<GlowingCircleProps> = ({
  color,
  size = "md",
  glowIntensity = "medium",
  isPulsing = false,
  animationType = "pulse",
  className = "",
}) => {
  const sizeClass = sizeConfig[size];
  const glowSize = glowConfig[glowIntensity];
  const glowColor = `${color}20`;
  const animationClass = isPulsing ? animationConfig[animationType] : "";

  return (
    <div
      className={`${sizeClass} rounded-full transition-all duration-300 ${animationClass} ${className}`}
      style={{
        backgroundColor: color,
        boxShadow: `${glowSize} ${glowColor}`,
      }}
    />
  );
};

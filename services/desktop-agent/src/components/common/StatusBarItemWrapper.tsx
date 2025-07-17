/**
 * StatusBarItemWrapper Component - React Bit Pattern
 *
 * A reusable status bar item wrapper following React Bits patterns.
 * This provides consistent styling, hover effects, and tooltip support
 * for all status bar items.
 *
 * React Bits Pattern: Component Composition with Children
 * Reference: https://reactbits.dev/
 */

import React from "react";
import { StatusBarItemProps } from "../../types";

export const StatusBarItemWrapper: React.FC<StatusBarItemProps> = ({
  children,
  tooltipRef,
  className = "",
}) => {
  return (
    <div
      ref={tooltipRef}
      className={`status-bar-item flex items-center cursor-pointer px-3 py-1.5 select-none transition-all duration-200 hover:bg-[#313244] ${className}`}
    >
      {children}
    </div>
  );
};

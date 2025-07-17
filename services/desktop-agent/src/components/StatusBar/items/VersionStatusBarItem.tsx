/**
 * Version Status Bar Item Component
 *
 * Displays application version information using React Bits patterns.
 * Provides version details in a clean, simple design.
 *
 * Features:
 * - Version display
 * - Clean, simple design
 * - Modular architecture
 * - React Bits patterns for reusability
 *
 * References:
 * - [React Bits](https://reactbits.dev/)
 */

import React from "react";
import { StatusBarItemWrapper } from "../../common";

interface VersionStatusBarItemProps {
  version?: string;
}

/**
 * Version Status Bar Item Component
 */
export const VersionStatusBarItem: React.FC<VersionStatusBarItemProps> = ({
  version = "0.1.0",
}) => {
  return (
    <StatusBarItemWrapper>
      <span className="text-xs font-medium text-[#cccccc] transition-colors duration-200">
        v{version}
      </span>
    </StatusBarItemWrapper>
  );
};

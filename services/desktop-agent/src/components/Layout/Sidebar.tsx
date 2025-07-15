import React from "react";
import { SidebarProps } from "../../types/layout";

export const Sidebar: React.FC<SidebarProps> = ({
  isCollapsed = false,
  onToggle,
}) => {
  return (
    <div
      className={`
      bg-vscode-sidebar 
      border-r 
      border-vscode-border 
      transition-all 
      duration-300 
      ease-in-out
      flex-shrink-0
      ${isCollapsed ? "w-12" : "w-48 md:w-56 lg:w-64"}
    `}
    >
      {/* Single unified sidebar - completely empty */}
    </div>
  );
};

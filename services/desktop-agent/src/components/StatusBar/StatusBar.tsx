import React from "react";
import { StatusBarItems } from "./StatusBarItems";

export const StatusBar: React.FC = () => {
  return (
    <div className="bg-dark-editor border-t border-dark-border h-8 flex items-center justify-between text-sm flex-shrink-0 min-w-0 px-2">
      {/* Left side - empty for future items */}
      <div></div>

      {/* Right side - Status bar items */}
      <StatusBarItems />
    </div>
  );
};

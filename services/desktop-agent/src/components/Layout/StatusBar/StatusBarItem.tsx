import React from "react";

export interface StatusBarItemProps {
  label: string;
  value: string;
}

export const StatusBarItem: React.FC<StatusBarItemProps> = ({
  label,
  value,
}) => {
  return (
    <div className="flex items-center space-x-1 px-2 py-1 text-xs text-vscode-textMuted hover:bg-vscode-highlight transition-colors rounded cursor-pointer">
      <span className="font-medium">
        {label}
        {value && ":"}
      </span>
      {value && <span>{value}</span>}
    </div>
  );
};

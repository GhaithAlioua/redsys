import React from "react";
import { ContentWindowProps } from "../../types/layout";

export const ContentWindow: React.FC<ContentWindowProps> = ({ children }) => {
  return (
    <div className="flex-1 bg-vscode-editor flex flex-col overflow-hidden min-w-0">
      {/* Content Body - Completely empty for base GUI */}
      <div className="flex-1 overflow-auto min-w-0">{children}</div>
    </div>
  );
};

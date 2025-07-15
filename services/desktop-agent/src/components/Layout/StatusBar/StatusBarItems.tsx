import React from "react";
import { StatusBarItem } from "./StatusBarItem";
import { APP_CONSTANTS } from "../../../constants/app";

export const StatusBarItems: React.FC = () => {
  return (
    <div className="flex items-center space-x-2">
      {/* Version Item */}
      <StatusBarItem label="Version" value={APP_CONSTANTS.VERSION} />
    </div>
  );
};

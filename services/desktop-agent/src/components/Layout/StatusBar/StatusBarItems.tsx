import React from "react";
import { StatusBarItem } from "./StatusBarItem";
import { APP_CONSTANTS } from "../../../constants/app";
import { DockerStatusItem } from "./DockerStatusItem";

export const StatusBarItems: React.FC = () => {
  return (
    <div className="flex items-center space-x-2">
      {/* Docker Status */}
      <DockerStatusItem />

      {/* Version Item */}
      <StatusBarItem label="Version" value={APP_CONSTANTS.VERSION} />
    </div>
  );
};

import React from "react";
import { DockerStatusBarItem, VersionStatusBarItem } from "./items";

export const StatusBarItems: React.FC = () => {
  return (
    <div className="flex items-center space-x-4">
      <VersionStatusBarItem />
      <DockerStatusBarItem />
      {/* Add more status bar items here if needed */}
    </div>
  );
};

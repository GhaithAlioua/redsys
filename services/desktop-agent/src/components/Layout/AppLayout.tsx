import React from "react";
import { Sidebar } from "./Sidebar";
import { ContentWindow } from "./ContentWindow";
import { StatusBar } from "./StatusBar";

const AppLayout: React.FC = () => {
  return (
    <div className="h-screen w-screen flex flex-col bg-dark-bg text-dark-text min-w-0">
      {/* Main content area */}
      <div className="flex-1 flex min-h-0">
        {/* Sidebar */}
        <Sidebar />

        {/* Content window */}
        <ContentWindow />
      </div>

      {/* Status bar */}
      <StatusBar />
    </div>
  );
};

export default AppLayout;

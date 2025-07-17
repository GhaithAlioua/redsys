import React from "react";
import { Sidebar } from "./Sidebar";
import { StatusBar } from "./StatusBar";
import { ContentWindow } from "./ContentWindow";

const AppLayout: React.FC = () => {
  return (
    <div className="flex flex-col h-screen w-screen bg-dark-bg">
      <div className="flex flex-1">
        <Sidebar />
        <ContentWindow />
      </div>
      <StatusBar />
    </div>
  );
};

export default AppLayout;

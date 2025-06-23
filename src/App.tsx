import "./App.css";

import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import Sidebar from "./components/Sidebar";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import DownloadDialog from "./components/dialogs/DownloadDialog";

function App() {
  return (
    <>
      <DownloadDialog />
      <main className="w-full h-full flex">
        <PanelGroup autoSaveId="sidebar__resized" direction="horizontal">
          <Panel
            collapsible={true}
            collapsedSize={8}
            defaultSize={10}
            maxSize={20}
          >
            <Sidebar />
          </Panel>
          <PanelResizeHandle className="w-0.5 flex bg-slate-700" />
          <Panel>
            <div className="flex-1 flex-col flex bg-base-100"></div>
          </Panel>
        </PanelGroup>
      </main>
    </>
  );
}

export default App;

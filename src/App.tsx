import type { Component } from "solid-js";
import { SplitPane } from "./components/SplitPane";
import Navbar from "./screens/Main/layouts/Navbar";
import Sidebar from "./screens/Main/layouts/Sidebar";
import ViewFiles from "./screens/Main/layouts/ViewFiles";

const App: Component = () => {
  return (
    <>
      <Navbar />
      <div style={{ display: "flex" }}>
        <SplitPane sizes={[20, 80]}>
          <div>
            <Sidebar />
          </div>
          <div>
            <ViewFiles />
          </div>
        </SplitPane>
      </div>
    </>
  );
};

export default App;

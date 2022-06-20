import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { MainScr } from "./screens/MainScr";
import { EmulationScr } from "./screens/EmulationScr";
import { Payload } from "./interfaces";

function App() {
  const [emulationOngoing, setEmulationOngoing] = useState(false);
  const [display, setDisplay] = useState<number[]>(Array(2048).fill(0));

  const start_emu = async () => {
    const file_path = await open();
    console.log(file_path);
    if (file_path === null) {
      return;
    }
    invoke("start_cpu", { romPath: file_path });
    setEmulationOngoing(true);

    document.addEventListener("keydown", (event) => {
      if (event.key === "d") {
        emit("key-action", {
          keyValue: 11,
        });
        console.log("key press down");
      }
    });

    document.addEventListener("keyup", (event) => {
      if (event.key === "d") {
        emit("key-action", {
          keyValue: 16,
        });
        console.log("key press up");
      }
    });
  };

  useEffect(() => {
    const unlistenDisplayUpdate = listen("display-update", (event) => {
      console.log("display update received");
      const data = event.payload as Payload;
      setDisplay(data.display);

      return () => {
        unlistenDisplayUpdate.then((f) => f());
      };
    });

    return () => {
      unlistenDisplayUpdate.then((f) => f());
    };
  }, []);

  return (
    <>
      {!emulationOngoing ? (
        <MainScr testHandler={start_emu} />
      ) : (
        <EmulationScr display={display} />
      )}
    </>
  );
}

export default App;

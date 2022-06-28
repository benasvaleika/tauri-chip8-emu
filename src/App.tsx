import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { MainScr } from "./screens/MainScr";
import { EmulationScr } from "./screens/EmulationScr";
import { Payload } from "./interfaces";
import { sendKeyActionDown } from "./utils/keys";

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
      let keyEvent = event;
      let keyValue: number | undefined;

      switch (keyEvent.key) {
        case "1":
          keyValue = 1;
          break;
        case "2":
          keyValue = 2;
          break;
        case "3":
          keyValue = 3;
          break;
        case "4":
          keyValue = 12;
          break;
        case "q":
          keyValue = 4;
          break;
        case "w":
          keyValue = 5;
          break;
        case "e":
          keyValue = 6;
          break;
        case "r":
          keyValue = 13;
          break;
        case "a":
          keyValue = 7;
          break;
        case "s":
          keyValue = 8;
          break;
        case "d":
          keyValue = 9;
          break;
        case "f":
          keyValue = 14;
          break;
        case "z":
          keyValue = 10;
          break;
        case "x":
          keyValue = 0;
          break;
        case "c":
          keyValue = 11;
          break;
        case "v":
          keyValue = 15;
          break;
      }

      if (typeof keyValue !== "undefined") {
        console.log("action sent", keyValue);
        emit("key-action", {
          keyValue: keyValue,
        });
      }
    });

    document.addEventListener("keyup", (event) => {
      emit("key-action", {
        keyValue: 16,
      });
      console.log("key press up");
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

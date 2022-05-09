import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { Button } from "./components/Button";

function App() {
  const testHandler = async () => {
    const file_path = await open();
    console.log(file_path);
    invoke("start_cpu", { romPath: file_path });
  };

  return (
    <div className="text-center">
      <h1 className="mt-10 text-5xl text-white font-extrabold">
        CHIP-8 EMULATOR
      </h1>
      <Button text="Load Rom" onClick={testHandler} className="mt-20 mr-4" />
    </div>
  );
}

export default App;

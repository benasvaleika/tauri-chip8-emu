import React from "react";
import { Button } from "../components/Button";

interface MainScrProps {
  testHandler: React.MouseEventHandler<HTMLButtonElement> | undefined;
}

export const MainScr: React.FC<MainScrProps> = ({ testHandler }) => {
  return (
    <div className="text-center">
      <h1 className="mt-10 text-5xl text-white font-extrabold">
        CHIP-8 EMULATOR
      </h1>
      <Button text="Load Rom" onClick={testHandler} className="mt-20 mr-4" />
    </div>
  );
};

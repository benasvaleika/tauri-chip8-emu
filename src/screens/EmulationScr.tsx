import React from "react";
import { Canvas } from "../components/Canvas";

interface EmulationScrProps {
  display: number[];
}

export const EmulationScr: React.FC<EmulationScrProps> = ({ display }) => {
  return <Canvas display={display} />;
};

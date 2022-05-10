import React from "react";

interface PixelProps {
  active: boolean;
}

export const Pixel: React.FC<PixelProps> = ({ active }) => {
  const active_design = active ? "bg-white" : "";

  return (
    <div
      className={" w-[calc(100%/64)] h-[calc(100vh/32)] " + active_design}
    ></div>
  );
};

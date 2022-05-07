import React from "react";

interface ButtonProps {
  text: string;
  onClick?: React.MouseEventHandler<HTMLButtonElement> | undefined;
  className?: string;
}

export const Button: React.FC<ButtonProps> = ({ text, onClick, className }) => {
  return (
    <button
      onClick={onClick}
      className={
        "text-white font-extrabold border-4 py-1 px-3 text-lg " + className
      }
    >
      {text}
    </button>
  );
};

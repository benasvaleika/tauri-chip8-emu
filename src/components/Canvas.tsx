import { Pixel } from "./Pixel";
import { v4 as uuidv4 } from "uuid";

interface CanvasProps {
  display: number[];
}

export const Canvas: React.FC<CanvasProps> = ({ display }) => {
  return (
    <div className="text-white flex flex-wrap">
      {display.map((p) => {
        if (p === 0) {
          return <Pixel key={uuidv4()} active={false} />;
        } else {
          return <Pixel key={uuidv4()} active={true} />;
        }
      })}
    </div>
  );
};

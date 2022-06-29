import { emit } from "@tauri-apps/api/event";

export const sendKeyActionDown = (keyEvent: KeyboardEvent) => {
  let keyValue: number | undefined;

  if (keyEvent.key === "1") {
    keyValue = 1;
  } else if (keyEvent.key === "2") {
    keyValue = 2;
  } else if (keyEvent.key === "3") {
    keyValue = 3;
  } else if (keyEvent.key === "4") {
    keyValue = 12;
  } else if (keyEvent.key === "q") {
    keyValue = 4;
  } else if (keyEvent.key === "w") {
    keyValue = 5;
  } else if (keyEvent.key === "e") {
    keyValue = 6;
  } else if (keyEvent.key === "r") {
    keyValue = 13;
  } else if (keyEvent.key === "a") {
    keyValue = 7;
  } else if (keyEvent.key === "s") {
    keyValue = 8;
  } else if (keyEvent.key === "d") {
    keyValue = 9;
  } else if (keyEvent.key === "f") {
    keyValue = 14;
  } else if (keyEvent.key === "z") {
    keyValue = 10;
  } else if (keyEvent.key === "x") {
    keyValue = 0;
  } else if (keyEvent.key === "c") {
    keyValue = 11;
  } else if (keyEvent.key === "v") {
    keyValue = 15;
  }
  if (typeof keyValue !== "undefined") {
    console.log("action sent", keyValue);
    emit("key-action-down", {
      keyValue: keyValue,
    });
  }
};

export const sendKeyActionUp = (keyEvent: KeyboardEvent) => {
  let keyValue: number | undefined;

  if (keyEvent.key === "1") {
    keyValue = 1;
  } else if (keyEvent.key === "2") {
    keyValue = 2;
  } else if (keyEvent.key === "3") {
    keyValue = 3;
  } else if (keyEvent.key === "4") {
    keyValue = 12;
  } else if (keyEvent.key === "q") {
    keyValue = 4;
  } else if (keyEvent.key === "w") {
    keyValue = 5;
  } else if (keyEvent.key === "e") {
    keyValue = 6;
  } else if (keyEvent.key === "r") {
    keyValue = 13;
  } else if (keyEvent.key === "a") {
    keyValue = 7;
  } else if (keyEvent.key === "s") {
    keyValue = 8;
  } else if (keyEvent.key === "d") {
    keyValue = 9;
  } else if (keyEvent.key === "f") {
    keyValue = 14;
  } else if (keyEvent.key === "z") {
    keyValue = 10;
  } else if (keyEvent.key === "x") {
    keyValue = 0;
  } else if (keyEvent.key === "c") {
    keyValue = 11;
  } else if (keyEvent.key === "v") {
    keyValue = 15;
  }
  if (typeof keyValue !== "undefined") {
    console.log("action sent", keyValue);
    emit("key-action-up", {
      keyValue: keyValue,
    });
  }
};

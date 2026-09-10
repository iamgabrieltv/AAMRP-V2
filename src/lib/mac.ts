import { invoke } from "@tauri-apps/api/core";
import { Command } from "@tauri-apps/plugin-shell";
import { setArtwork } from "./common";

export async function setActivityMac(
  command: Command<string>,
  oldOutput: string[],
): Promise<string[] | void> {
  const output = await command.execute();

  if (output.stderr.length > 0 || output.stdout.length === 0) {
    console.error("Error executing AppleScript:", output.stderr);
    invoke("clear_activity");
    return;
  }

  const [title, artist, album, state, duration, position] =
    output.stdout.split("$s$");
  const currentPosition = parseFloat(position);
  const previousPosition = parseFloat(oldOutput[4]);
  const trackData = [title, artist, album, state];
  const isLooping =
    oldOutput.length > 0 &&
    Number.isFinite(previousPosition) &&
    Number.isFinite(currentPosition) &&
    currentPosition < previousPosition;

  if (
    !isLooping &&
    oldOutput.length > 0 &&
    oldOutput.slice(0, 4).every((v, i) => v === trackData[i])
  ) {
    return [...oldOutput.slice(0, 4), position];
  } else {
    oldOutput = [...trackData, position];
  }

  if (state === "paused") {
    invoke("clear_activity");
    return oldOutput;
  }

  // Calculate start and end timestamps
  const startT = Math.floor(Date.now() - currentPosition * 1000);
  const endT = Math.floor(
    Date.now() + (parseFloat(duration) - currentPosition) * 1000,
  );

  invoke("set_activity", {
    title,
    artist,
    album,
    startT,
    endT,
    largeImage: "apple_music",
    smallImage: "apple_music",
  } as SongData);

  setArtwork(title, artist, album, startT, endT);

  // return oldOutput so it can be used in the next call
  return oldOutput;
}

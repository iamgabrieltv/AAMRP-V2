import { invoke } from "@tauri-apps/api/core";
import { Command } from "@tauri-apps/plugin-shell";
import { appleRequest } from "./common";

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

  if (
    oldOutput.length > 0 &&
    oldOutput.every((v, i) => v === [title, artist, album, state][i])
  ) {
    return;
  } else {
    oldOutput = [title, artist, album, state];
  }

  if (state === "paused") {
    invoke("clear_activity");
    return oldOutput;
  }

  // Calculate start and end timestamps
  const startT = Math.floor(Date.now() - parseFloat(position) * 1000);
  const endT = Math.floor(
    Date.now() + (parseFloat(duration) - parseFloat(position)) * 1000,
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

  appleRequest(title, artist, album, startT, endT);

  // return oldOutput so it can be used in the next call
  return oldOutput;
}

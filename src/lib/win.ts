import { invoke } from "@tauri-apps/api/core";
import { appleRequest } from "./common";

export async function setActivityWin(
  oldOutput: (string | boolean)[],
): Promise<void | (string | boolean)[]> {
  const output = await invoke<WindowsMediaResponse>(
    "get_listening_status_win",
  ).catch((error) => {
    console.error("Error fetching media status:", error);
    return;
  });

  if (output) {
    if (
      oldOutput.length > 0 &&
      oldOutput.every(
        (v, i) =>
          v ===
          [output.title, output.artist, output.album, output.is_playing][i],
      )
    ) {
      return;
    } else {
      oldOutput = [
        output.title,
        output.artist,
        output.album,
        output.is_playing,
      ];
    }

    if (!output.is_playing) {
      invoke("clear_activity");
      return;
    }

    // Calculate start and end timestamps
    const startT = Date.now() - output.position * 1000;
    const endT = Date.now() + (output.duration - output.position) * 1000;

    invoke("set_activity", {
      title: output.title,
      artist: output.artist,
      album: output.album,
      startT,
      endT,
      largeImage: "apple_music",
      smallImage: "apple_music",
    } as SongData);

    appleRequest(output.title, output.artist, output.album, startT, endT);

    // return oldOutput so it can be used in the next call
    return oldOutput;
  }
}

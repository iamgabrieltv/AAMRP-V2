import { invoke } from "@tauri-apps/api/core";
import { setArtwork } from "./common";

export async function setActivityWin(oldOutput: {
  [key: string]: any;
}): Promise<void | {}> {
  const output = await invoke<WindowsMediaResponse>(
    "get_listening_status_win",
  ).catch((error) => {
    console.error("Error fetching media status:", error);
    invoke("clear_activity");
    return;
  });

  if (output) {
    if ((output.duration === 0 || output.position === 0) && output.is_playing) {
      oldOutput = {};
      console.log("detected 0/0, returning and clearing");
      return oldOutput;
    }

    if (
      output.position === output.duration ||
      output.position === output.duration - 1
    ) {
      oldOutput = {};
      console.log("clearing at " + output.position + "/" + output.duration);
      return oldOutput;
    }

    if (
      oldOutput.title === output.title &&
      oldOutput.artist === output.artist &&
      oldOutput.album === output.album &&
      oldOutput.is_playing === output.is_playing
    ) {
      return;
    } else {
      oldOutput = {
        title: output.title,
        artist: output.artist,
        album: output.album,
        is_playing: output.is_playing,
      };
    }

    if (!output.is_playing) {
      invoke("clear_activity");
      return oldOutput;
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

    setArtwork(output.title, output.artist, output.album, startT, endT);

    // return oldOutput so it can be used in the next call
    return oldOutput;
  } else {
    invoke("clear_activity");
  }
}

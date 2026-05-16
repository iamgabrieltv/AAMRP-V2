import { invoke } from "@tauri-apps/api/core";
import { Command } from "@tauri-apps/plugin-shell";

export async function setActivityMac(
  command: Command<string>,
  oldOutput: string[],
) {
  const output = await command.execute();

  if (output.stderr.length > 0) {
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
    return;
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

  invoke<AppleMusicData>("apple_request", {
    title,
    artist,
    album,
  }).then((result) => {
    let albumData = result.results.album.data.find(
      (a) => a.attributes.name === album,
    );
    let artistData = result.results.artist.data.find(
      (a) => a.attributes.url === albumData?.attributes.artistUrl,
    );
    if (artistData === undefined) {
      console.error("Artist not found");
      artistData = result.results.artist.data[0];
    }
    if (albumData === undefined) {
      console.error("Album not found");
      albumData = result.results.album.data[0];
    }
    const albumArtwork = albumData.attributes.artwork.url
      .replace("{w}", "1024")
      .replace("{h}", "1024");
    const artistArtwork = artistData.attributes.artwork.url
      .replace("{w}", "1024")
      .replace("{h}", "1024");

    invoke("set_activity", {
      title,
      artist,
      album,
      startT,
      endT,
      largeImage: albumArtwork,
      smallImage: artistArtwork,
    } as SongData);
  });
}

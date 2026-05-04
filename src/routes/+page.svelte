<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from "@tauri-apps/plugin-os";
  import { Command } from "@tauri-apps/plugin-shell";
  import { resolveResource } from "@tauri-apps/api/path";
  import { onDestroy, onMount } from "svelte";
  import { setDockVisibility } from "@tauri-apps/api/app";

  const currentPlatform = platform();
  let intervalId: number;

  onMount(async () => {
    await invoke("connect");

    if (currentPlatform === "macos") {
      setDockVisibility(false);
      let oldOutput: string[] = [];

      intervalId = setInterval(async () => {
        const scriptPath = await resolveResource("resources/mac.scpt");
        const output = await Command.create("osascript", scriptPath).execute();

        if (output.stderr.length > 0) {
          console.error("Error executing AppleScript:", output.stderr);
          invoke("clear_activity");
          return;
        }

        const [title, artist, album, state, duration, position] =
          output.stdout.split("$s$");
        if (state === "paused") {
          invoke("clear_activity");
        }

        if (
          oldOutput.length > 0 &&
          oldOutput.every((v, i) => v === [title, artist, album, state][i])
        ) {
          return;
        } else {
          oldOutput = [title, artist, album, state];
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
      }, 10000);
    }

    if (currentPlatform !== "macos") {
      let songData: SongData = {
        title: "GARBAGE",
        artist: "Melanie Martinez",
        album: "HADES",
        largeImage: "apple_music",
        smallImage: "apple_music",
        startT: Date.now(),
        endT: Date.now() + 60000,
      };

      invoke("set_activity", songData);

      let result: AppleMusicData;
      try {
        result = await invoke("apple_request", songData);
      } catch (error) {
        console.error("apple_request error:", error);
        invoke("clear_activity");
        return;
      }

      let album = result.results.album.data.find(
        (a) => a.attributes.name.toLowerCase() === songData.album.toLowerCase(),
      );
      let artist = result.results.artist.data.find(
        (a) =>
          a.attributes.name.toLowerCase() === songData.artist.toLowerCase(),
      );
      if (artist === undefined) {
        console.error("Artist not found");
        artist = result.results.artist.data[0];
      }
      if (album === undefined) {
        console.error("Album not found");
        album = result.results.album.data[0];
      }
      const albumArtwork = album.attributes.artwork.url
        .replace("{w}", "1024")
        .replace("{h}", "1024");
      const artistArtwork = artist.attributes.artwork.url
        .replace("{w}", "1024")
        .replace("{h}", "1024");

      invoke("set_activity", {
        title: songData.title,
        artist: songData.artist,
        album: songData.album,
        largeImage: albumArtwork,
        smallImage: artistArtwork,
        startT: songData.startT,
        endT: songData.endT,
      } as SongData);
    }
  });

  onDestroy(() => {
    clearInterval(intervalId);
    invoke("disconnect");
  });
</script>

<main>
  <h1>Hi</h1>
</main>

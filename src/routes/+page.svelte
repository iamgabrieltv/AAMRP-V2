<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { resolveResource } from "@tauri-apps/api/path";
  import { setDockVisibility } from "@tauri-apps/api/app";
  import { listen } from "@tauri-apps/api/event";
  import { platform } from "@tauri-apps/plugin-os";
  import { Command } from "@tauri-apps/plugin-shell";
  import { Store } from "@tauri-apps/plugin-store";
  import { onDestroy, onMount } from "svelte";

  const currentPlatform = platform();
  let store: Store;

  let interval: number | undefined = $state();

  onMount(async () => {
    store = await Store.load("config.json");
    interval = await store.get<number>("interval");
    if (interval === undefined) {
      await store.set("interval", 5);
      interval = 5;
    }

    await invoke("connect");

    if (currentPlatform === "macos") {
      setDockVisibility(false);
      const scriptPath = await resolveResource("resources/mac.scpt");
      const command = Command.create("osascript", scriptPath);
      let oldOutput: string[] = [];

      invoke("set_interval", { interval: interval * 1000 });

      async function setActivity() {
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

      listen("tick", setActivity);
    }
  });

  onDestroy(async () => {
    invoke("disconnect");
    await store.save();
  });

  let message = $state();
  function applyHandler() {
    store.set("interval", interval);
    store.save();
    message = "Restart the app to fully apply changes";
  }
</script>

<main>
  <h1>Hi</h1>
  <input type="number" bind:value={interval} min="1" />
  <button onclick={applyHandler}>Apply</button>
  <p>{message}</p>
</main>

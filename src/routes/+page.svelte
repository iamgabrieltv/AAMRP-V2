<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from "@tauri-apps/plugin-os";
  import { Command } from "@tauri-apps/plugin-shell";
  import { resolveResource } from "@tauri-apps/api/path";
  import { onDestroy, onMount } from "svelte";

  const currentPlatform = platform();
  let intervalId: number;

  let songData: SongData = {
    title: "VYZEE",
    artist: "SOPHIE",
    album: "PRODUCT",
    largeImage: "dummy",
    smallImage: "dummy",
  };

  onMount(async () => {
    invoke("connect");

    if (currentPlatform === "macos") {
      intervalId = setInterval(async () => {
        const scriptPath = await resolveResource("resources/mac.scpt");
        const output = await Command.create("osascript", scriptPath).execute();

        if (output.stderr.length > 0) {
          console.error("Error executing AppleScript:", output.stderr);
          invoke("clear_activity");
          return;
        }

        const [title, artist, album, state] = output.stdout.split("$s$");
        if (state === "paused") {
          invoke("clear_activity");
        } else {
          invoke("set_activity", {
            title,
            artist,
            album,
            largeImage: "dummy",
            smallImage: "dummy",
          } as SongData);
        }
      }, 10000);
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

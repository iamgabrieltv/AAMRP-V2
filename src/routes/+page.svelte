<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";

  let songData: SongData = {
    title: "VYZEE",
    artist: "SOPHIE",
    album: "PRODUCT",
    largeImage: "dummy",
    smallImage: "dummy",
  };

  onMount(() => {
    invoke("connect");
    invoke("set_activity", songData);
  });

  onDestroy(() => {
    invoke("disconnect");
  });
</script>

<main>
  <h1>Hi</h1>
</main>

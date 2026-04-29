<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  onMount(() => {
    invoke("set_activity");
  });
</script>

<main>
  <h1>Hi</h1>
</main>

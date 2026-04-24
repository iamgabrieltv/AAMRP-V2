<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { start, stop, setActivity } from "tauri-plugin-drpc";
  import { Activity, ActivityType, Assets } from "tauri-plugin-drpc/activity";

  const assets = new Assets()
    .setLargeImage("dummy")
    .setLargeText("Album Cover")
    .setSmallImage("dummy")
    .setSmallText("Artist");

  const activity = new Activity()
    .setActivity(ActivityType.Listening)
    .setDetails("Song Name")
    .setState("Artist Name")
    .setAssets(assets);

  const simpleActivity = new Activity()
    .setDetails("Testing Tauri")
    .setState("It works!");

  onMount(async () => {
    await start("1423726101519274056");
    setTimeout(async () => {
      await setActivity(simpleActivity);
    }, 500);
  });

  onDestroy(async () => {
    await stop();
  });
</script>

<main>
  <h1>Hi</h1>
</main>

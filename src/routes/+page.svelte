<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
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

  const json = {
    state: "Artist Name",
    details: "Song Name",
    activity_type: 2,
    assets: {
      large_image: "dummy",
      large_text: "Album Cover",
      small_image: "dummy",
      small_text: "Artist",
    },
  };

  onMount(async () => {
    await start("1423726101519274056");
    await invoke("plugin:drpc|set_activity", {
      activityJson: JSON.stringify(json),
    });
    // setTimeout(async () => {
    //   await setActivity(activity);
    // }, 500);
  });

  onDestroy(async () => {
    await stop();
  });
</script>

<main>
  <h1>Hi</h1>
</main>

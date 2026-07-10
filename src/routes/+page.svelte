<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { resolveResource } from "@tauri-apps/api/path";
  import { setDockVisibility } from "@tauri-apps/api/app";
  import { listen } from "@tauri-apps/api/event";
  import { platform } from "@tauri-apps/plugin-os";
  import { Command } from "@tauri-apps/plugin-shell";
  import { enable, isEnabled, disable } from "@tauri-apps/plugin-autostart";
  import { onDestroy, onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { setActivityMac } from "$lib/mac";
  import { setActivityWin } from "$lib/win";

  const currentPlatform = platform();

  let autostart = $state<boolean>();

  let ranInit = false;

  onMount(async () => {
    autostart = await isEnabled();
    if (autostart) {
      await getCurrentWebviewWindow().hide();
    }

    await invoke("connect");

    if (currentPlatform === "macos" && !ranInit) {
      setDockVisibility(false);
      const scriptPath = await resolveResource("resources/mac.scpt");
      const command = Command.create("osascript", scriptPath);
      let oldOutput: string[] = [];

      invoke("set_interval");

      function setActivity() {
        setActivityMac(command, oldOutput).then((output) => {
          {
            if (output) {
              oldOutput = output;
            }
          }
        });
      }

      listen("tick", setActivity);
      ranInit = true;
    }

    if (currentPlatform === "windows" && !ranInit) {
      let oldOutput = {};

      invoke("set_interval");

      function setActivity() {
        setActivityWin(oldOutput).then((output) => {
          if (output) {
            oldOutput = output;
          }
        });
      }

      listen("tick", setActivity);
      ranInit = true;
    }
  });

  onDestroy(async () => {
    await invoke("disconnect");
  });

  let message = $state<string>("");
  async function applyHandler() {
    if (autostart) {
      await enable();
    } else {
      await disable();
    }
    if (message === "") {
      message = "Settings applied.";
    }
  }
</script>

<main class="p-2">
  <h1 class="text-4xl font-bold">Settings</h1>
  <form class="flex flex-col gap-1 items-start">
    <label class="flex flex-col"
      >Autostart app on system startup <input
        type="checkbox"
        bind:checked={autostart}
        class="w-5 h-5 rounded-sm"
      /></label
    >
    <button onclick={applyHandler} class="w-fit p-1 my-1 rounded-md"
      >Apply</button
    >
  </form>
  <p class="text-[#f38ba8] font-bold text-pretty">{message}</p>
</main>

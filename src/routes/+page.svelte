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
  import { getVersion } from "@tauri-apps/api/app";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { ask } from "@tauri-apps/plugin-dialog";

  const currentPlatform = platform();

  let appVersion = $state<string>("");

  let autostart = $state<boolean>();

  let ranInit = false;

  onMount(async () => {
    appVersion = await getVersion();
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

      // Check for updates
      const response = await fetch(
        "https://api.github.com/repos/iamgabrieltv/AAMRP-V2/releases/latest",
      );
      const data: GitHubResponse = await response.json();
      if (data.tag_name !== `v${appVersion}`) {
        const answer = await ask(
          `New version ${data.tag_name} is available. Open release page?`,
          {
            title: "AAMRP Update available",
            kind: "info",
          },
        );

        if (answer) {
          openUrl(data.html_url);
        }
      }

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
    <button onclick={applyHandler} class="w-fit p-1 my-1 rounded-md btn"
      >Apply</button
    >
  </form>
  <p class="text-[#f38ba8] font-bold text-pretty">{message}</p>
  <button
    type="button"
    class="text-xs text-gray-500 fixed bottom-2 right-2 cursor-pointer"
    onclick={() => openUrl("https://github.com/iamgabrieltv/AAMRP-V2/releases")}
  >
    v{appVersion}
  </button>
</main>

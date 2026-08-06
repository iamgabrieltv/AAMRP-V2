import { ask, message } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import { getVersion } from "@tauri-apps/api/app";

export async function checkUpdates() {
  const appVersion = await getVersion();
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
  } else {
    await message("No updates available", {
      title: "AAMRP Update Checker",
      kind: "info",
    });
  }
}

> [!WARNING]
> This app uses ffmpeg to convert the animated cover to AVIF if the file doesn't exist on the server yet. This can cause a short spike of resource and network usage on your system. (not supported on Windows ARM as of now)

# AAMRP

Animated Apple Music Rich Presence

<img height="250" alt="example image" src="https://iamgabriel.dev/assets/projects/aamrp-v2.avif" />

So you can show off what you're listening to on Discord!

# ⚠️ IMPORTANT FOR MAC USERS

Mac users need to run a command before they'll be able to run the app. This is because it is not signed.

`xattr -cr [PATH TO THE .APP]`

# Credits

- This app uses a self-compiled build of [FFmpeg](https://ffmpeg.org/) licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) license
  - Build for Windows x86_64: https://aamrp.iamgabriel.dev/binaries/ffmpeg-x86_64-pc-windows-msvc.exe
  - Build for macOS aarch64: https://aamrp.iamgabriel.dev/binaries/ffmpeg-aarch64-apple-darwin
  - Build for macOS x86_64: https://aamrp.iamgabriel.dev/binaries/ffmpeg-x86_64-apple-darwin
- This app was made using [Tauri](https://tauri.app/)
- This app is in no way affiliated with Apple

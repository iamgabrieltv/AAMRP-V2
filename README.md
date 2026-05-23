> [!WARNING]
> This app uses ffmpeg to convert the animated cover to AVIF if the file doesn't exist on the server yet. This can cause a short spike of resource and network usage on your system. (not supported on Windows ARM or Intel Macs unless requested)

# AAMRP

Animated Apple Music Rich Presence

<img width="446" height="184" alt="{6F8DA35B-EAB2-4756-90AD-721848E163D6}" src="https://github.com/user-attachments/assets/d78f0ada-76d4-46f8-9524-9c6b288d42b3" />

So you can show off what you're listening to on Discord!

# ⚠️ IMPORTANT FOR MAC USERS

Mac users need to run a command before they'll be able to run the app. This is because it is not signed.

`xattr -cr [PATH TO THE .APP]`

# Credits

- This app uses a self-compiled build of [FFmpeg](https://ffmpeg.org/) licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) license
- This app was made using [Tauri](https://tauri.app/)
- This app is in no way affiliated with Apple

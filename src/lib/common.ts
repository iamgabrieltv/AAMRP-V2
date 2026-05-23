import { invoke } from "@tauri-apps/api/core";
import { Command } from "@tauri-apps/plugin-shell";
import { appCacheDir } from "@tauri-apps/api/path";
import { exists, readFile, remove } from "@tauri-apps/plugin-fs";

export async function setArtwork(
  title: string,
  artist: string,
  album: string,
  startT: number,
  endT: number,
) {
  let artistsDisplay: string | undefined;
  if (artist.includes(" & ")) {
    const artists = artist.split(" & ");
    artist = artists[0];
    artistsDisplay = artists.join(" & ");
  }

  invoke<AppleMusicData>("apple_request", {
    title,
    artist,
    album,
  })
    .then((result) => {
      let albumData = result.results.album?.data.find(
        (a) => a.attributes.name === album,
      );
      let artistData = result.results.artist?.data.find(
        (a) => a.attributes.url === albumData?.attributes.artistUrl,
      );
      if (artistData === undefined) {
        console.error("Artist not found");
        artistData = result.results.artist.data[0];
      }
      if (albumData === undefined) {
        console.error("Album not found");
        albumData = result.results.album.data[0];
      }
      const albumArtwork = albumData.attributes.artwork.url
        .replace("{w}", "1024")
        .replace("{h}", "1024");
      const artistArtwork = artistData.attributes.artwork.url
        .replace("{w}", "1024")
        .replace("{h}", "1024");

      let data: SongData = {
        title,
        artist: artistsDisplay ?? artist,
        album,
        startT,
        endT,
        largeImage: albumArtwork,
        smallImage: artistArtwork,
      };

      invoke("set_activity", data);

      setAnimatedArtwork(albumData.id, data);
    })
    .catch((error) => console.error("Error fetching Apple Music data:", error));
}

async function setAnimatedArtwork(id: string, data: SongData) {
  const checkResponse = await fetch(
    `https://aamrp.iamgabriel.dev/artwork/${id}.avif`,
    {
      method: "HEAD",
    },
  );
  if (checkResponse.ok) {
    invoke("set_activity", {
      ...data,
      largeImage: `https://aamrp.iamgabriel.dev/artwork/${id}.avif`,
    } as SongData);
    console.log("Artwork already exists");
    return;
  }

  invoke<AppleMusicAlbumData>("apple_animated_artwork_request", {
    id,
  }).then(async (result) => {
    if (result.data[0].attributes.editorialVideo) {
      const videoUrl =
        result.data[0].attributes.editorialVideo.motionDetailSquare.video;

      const cacheDir = await appCacheDir();
      const outputPath = `${cacheDir}/${id}.avif`;

      const command = Command.sidecar("binaries/ffmpeg", [
        "-protocol_whitelist",
        "file,http,https,tcp,tls,crypto",
        "-i",
        videoUrl,
        "-c:v",
        "libsvtav1",
        "-r",
        "30",
        "-an",
        "-vf",
        "scale=1080:1080",
        outputPath,
      ]);
      const output = await command.execute();
      if (output.code !== 0) {
        console.error("ffmpeg failed with code", output.code);
        // Cleanup
        if (await exists(outputPath)) {
          await remove(outputPath);
        }
        return;
      }

      if (!(await exists(outputPath))) {
        console.error("Output file not found after ffmpeg execution");
        return;
      }

      const avifData = await readFile(outputPath);
      const apiResponse = await fetch(
        `https://api.aamrp.iamgabriel.dev/artwork/${id}.avif`,
        {
          method: "PUT",
          headers: {
            "Content-Type": "image/avif",
          },
          body: avifData,
        },
      );
      if (apiResponse.ok) {
        remove(outputPath);
        invoke("set_activity", {
          ...data,
          largeImage: `https://aamrp.iamgabriel.dev/artwork/${id}.avif`,
        } as SongData);
      } else {
        console.error("Failed to upload artwork");
      }
    } else {
      console.warn("No editorial video found for this album");
    }
  });
}

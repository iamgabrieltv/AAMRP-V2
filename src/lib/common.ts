import { invoke } from "@tauri-apps/api/core";

export async function setArtwork(
  title: string,
  artist: string,
  album: string,
  startT: number,
  endT: number,
) {
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

      let albumArtwork = "";
      let artistArtwork = "";

      if (artistData !== undefined) {
        artistArtwork = artistData.attributes.artwork.url
          .replace("{w}", "1024")
          .replace("{h}", "1024");
      } else console.error("Artist not found");

      if (albumData !== undefined) {
        albumArtwork = albumData.attributes.artwork.url
          .replace("{w}", "1024")
          .replace("{h}", "1024");
      } else console.error("Album not found");

      invoke("set_activity", {
        title,
        artist,
        album,
        startT,
        endT,
        largeImage: albumArtwork,
        smallImage: artistArtwork,
      } as SongData);
    })
    .catch((error) => console.error("Error fetching Apple Music data:", error));
}

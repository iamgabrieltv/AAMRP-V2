import { invoke } from "@tauri-apps/api/core";

export async function appleRequest(
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
      let albumData = result.results.album.data.find(
        (a) => a.attributes.name === album,
      );
      let artistData = result.results.artist.data.find(
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

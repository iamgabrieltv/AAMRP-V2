interface SongData {
  title: string;
  artist: string;
  album: string;
  largeImage: string;
  smallImage: string;
  startT: number;
  endT: number;
  [key: string]: any;
}

interface AppleMusicData {
  results: {
    album: {
      data: [
        {
          id: string;
          attributes: {
            artistName: string;
            artwork: {
              url: string;
            };
            name: string;
            url: string;
          };
        },
      ];
    };
    artist: {
      data: [
        {
          id: string;
          attributes: {
            name: string;
            artwork: {
              url: string;
            };
          };
        },
      ];
    };
  };
}

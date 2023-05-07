use serde_derive::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug)]
pub struct Track {
    pos: usize,
    pub artist_name: String,
    track_uri: String,
    artist_uri: String,
    track_name: String,
    album_uri: String,
    duration_ms: usize,
    album_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    name: String,
    collaborative: String,
    pid: usize,
    modified_at: usize,
    num_tracks: usize,
    num_albums: usize,
    num_followers: usize,
    pub tracks: Vec<Track>,
    num_edits: usize,
    duration_ms: usize,
    num_artists: usize,
}


#[derive(Deserialize, Serialize, Debug)]
struct Inf{
    generated_on : String,
    slice : String,
    version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct All{
    info : Inf,
    pub playlists: Vec<Playlist>,
}

#[derive(Debug) ]
pub struct Graph  {
    pub n: usize,
    pub outedges: Vec<Vec<usize>>
}


impl Graph{
    pub fn create_graph(n: usize, edges: &Vec<(usize, usize)>) -> Graph{

        let mut g = Graph{n, outedges:vec![vec![];n]};
        for (u,v) in edges {
            g.outedges[*u].push(*v);
            g.outedges[*v].push(*u);
        }

        for playlist in g.outedges.iter_mut() {
            playlist.sort();
            playlist.dedup();
        }

        return g;
    }

   
}
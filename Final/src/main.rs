use std::{collections::HashMap};
mod tests;
mod struc;
use struc::All;
use struc::Graph;
use std::collections::VecDeque;



//this goes through every playlist in every file I am reading, and makes a vector with all the artists
//in the playlist. That vector then gets puts in a vector with all the playlists, so the final product is 
// a vector where each element is a vector representing a playlist, and each playlist just has the artists
//that appear within it
fn get_artists(input_path: &str, files_to_read: usize) -> Vec<Vec<String>>{
    let files = std::fs::read_dir(input_path).expect("Unable to read directory");
    let mut artists_of_playlist_vec: Vec<Vec<String>> = Vec::new();

    let mut count = 0;
    for file in files{
        
        let file = file.expect("Unable to read file");
        let path = file.path();
        let mystrin = std::fs::read_to_string(&path).unwrap();
        let playlist_vec = serde_json::from_str::<All>(&mystrin).unwrap();
        drop(mystrin);


        for i in playlist_vec.playlists {
            let mut artists_in_playlist_vec: Vec<String> = Vec::new();
            for j in i.tracks{
    
                artists_in_playlist_vec.push(j.artist_name);
                
    
            }
            artists_in_playlist_vec.sort();
            artists_in_playlist_vec.dedup();
            artists_of_playlist_vec.push(artists_in_playlist_vec)
        }
        count += 1;
        if count % 75 == 0{
            println!("Succesfully read {}/1000 files", count);
        }
        if count == files_to_read {break};
       
    }
    

    //return artists_of_playlist_vec;
    return artists_of_playlist_vec;
}

//takes a vector of vectors and returns a sorted vector with all the unique elements
pub fn unique_artists(vec: &Vec<Vec<String>>) -> Vec<String>{
    let artists_per_playlist = vec.clone();
    let mut artist_vec: Vec<String> =  artists_per_playlist.into_iter().flatten().collect();
    artist_vec.sort();
    artist_vec.dedup();

    return artist_vec;
}

//takes my vector with all the artists in each playlist and makes a similar vector where all
//the artists are the number they are associated with in the hashmap
fn convert_to_num(vec: Vec<Vec<String>>, hash: &HashMap<String, usize>) -> Vec<Vec<usize>>{
    let mut numerical_artists_playlists: Vec<Vec<usize>> = Vec::new();

    for (i, playlist) in vec.iter().enumerate(){
        numerical_artists_playlists.push(vec![0;playlist.len()]);
        for (j, artist) in playlist.iter().enumerate(){
            numerical_artists_playlists[i][j] = *hash.get(artist).unwrap();
        }
    }   
    return numerical_artists_playlists;
}


fn mark_component_bfs(vertex:usize, graph:&Graph, component:&mut Vec<Option<usize>>, component_no:usize) {
    component[vertex] = Some(component_no);
    
    let mut queue = VecDeque::new();
    queue.push_back(vertex);
    
    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }

}

fn compute_distance_bfs(start: usize, graph: &Graph, ignore: &Vec<usize>) -> f32{
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); // <= we know this distance
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    let mut sum: f32 = 0.0;
    let mut count: f32 = 0.0;
    for v in 0..graph.n {

        if !ignore.contains(&v){
            sum += distance[v].unwrap() as f32;
            count +=1.0;
        }
       
    }
    return sum/count;

    //println!();
}



fn main() {

    let input_path = "data";
    //change this number to change the number of files you want to read
    let n_files_to_read:usize = 8;
    

    //let files = std::fs::read_dir(input_path).expect("Unable to read directory");
    let artists_of_playlist_vec: Vec<Vec<String>> = get_artists(input_path, n_files_to_read);


    

    //this makes a separate vector with the names of all of the unique artists in artists_of_playlist_vec
    let artist_vec = unique_artists(&artists_of_playlist_vec);
    println!("Found all unique artists: {} of them", artist_vec.len());

    
    //creates a hashmap by iterating over artists vec, where the key is the artist name and the value is a number
    //the number will be the node corresponding to each artist
    let mut artist_numbers = HashMap::<String, usize>::new();
    for (index, value) in artist_vec.iter().enumerate(){
        artist_numbers.insert(String::from(value), index);
    }
    println!("Hashmap Made");


    let numerical_artists_playlists = convert_to_num(artists_of_playlist_vec, &artist_numbers);

    
    //this code is meant to check if an artist appears in a playlist, and if it does, to create a tuple with that 
    //that artist an every artist in that playlist, and add it to a vector
    let mut tuples_of_edges: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    //iterate over every artist we have
    for artist in artist_vec.iter(){

        //go through each playlist with artists as numbers
        for playlist in &numerical_artists_playlists{

            //if that playlist contains the number that is associated with the artist we are on
            //make a tuple with that artist and every artist in the playlist
            if playlist.contains(artist_numbers.get(artist).unwrap()){
                
                for singer in playlist{

                    tuples_of_edges.push((*artist_numbers.get(artist).unwrap(), *singer))
                    
                    
                }

            }

        }
        count += 1;
        if count % 1000 == 0{
            println!("Found connections of {}, artists number {}/{}", artist, count, artist_vec.len() );
        }
    }
    tuples_of_edges.retain(|&(first, second)| first != second);
   
    

    let g = Graph::create_graph(artist_numbers.len(), &tuples_of_edges);
    println!("Graph Made");
    drop(tuples_of_edges);

    //finds the component of each node
    let num = g.n;
    let mut component: Vec<Option<usize>> = vec![None;num];
    let mut component_count = 0;
    for v in 0..num {
        if let None = component[v] {
            component_count += 1;
            mark_component_bfs(v, &g, &mut component, component_count);
        }
    }

    //finds which component is the largest by counting how many there are of each.
    // Index 0 corresponds to component 1
    let mut num_of_each_comp: Vec<usize> = vec![0; component_count];
    for comp_num in &component{
        num_of_each_comp[comp_num.unwrap()-1] += 1;
    }
    let mut max_val = 0;
    let mut max_index: usize = 0;
    for (index, &item) in num_of_each_comp.iter().enumerate(){
        if item > max_val{
            max_val = item;
            max_index = index;
        }
    }

    //after figuring out which component is largest, makes a list of all nodes that don't belong to 
    //largest component
    let component_max = max_index +1;
    let mut unwanted_nodes : Vec<usize> = vec![];
    for (index, &comp_num) in component.iter().enumerate(){
        if comp_num.unwrap() != component_max{
            unwanted_nodes.push(index);
        }
    }
    drop(component);
    drop(num_of_each_comp);
    println!("Largest componenent found");



    //for every node not in the list of unwanted nodes, runs BFS
    //compute distance bfs returns the average distance from that node to other nodes
    //adds that average to the total average.
    //prints progress after every 1000 nodes searched.
    let mut sum:f32 = 0.0;
    let mut count:f32 = 0.0;
    for i in 0..g.n {
        if !unwanted_nodes.contains(&i){
            sum += compute_distance_bfs(i, &g, &unwanted_nodes);
            count += 1.0;
        }
        if count % 1000.0 == 0.0 {
            println!("Found average for up to node: {}", count);
            println!("Current average: {}", sum/count);
        }
    }

    let average  = sum/count;
    
    
    println!("Average number of steps from every node to any other node: {:?} for {} artists", average, (artist_numbers.len()-unwanted_nodes.len()));

 
}    


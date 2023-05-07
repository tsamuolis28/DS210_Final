use crate::struc::Graph;

use super::unique_artists;
use std::{collections::HashMap};
use super:: convert_to_num;
use super::mark_component_bfs;

#[test]
pub fn test_unique(){
    let my_vec: Vec<Vec<String>> = vec![vec![String::from("a"),String::from("b"),String::from("c")],
                                       vec![String::from("b"),String::from("c"),String::from("d")],
                                       vec![String::from("c"),String::from("d"),String::from("e")]];
    let new_vec = unique_artists(&my_vec);
    assert_eq!(new_vec, vec!["a","b","c","d","e"]);

}

#[test]
pub fn test_num_convert(){
    let mut letters = HashMap::<String,usize>::new();
    let test_vec = vec![vec![String::from("a"),String::from("b"),String::from("c")],
    vec![String::from("b"),String::from("c"),String::from("d")],
    vec![String::from("c"),String::from("d"),String::from("e")]];
    letters.insert(String::from("a"), 0 );
    letters.insert(String::from("b"), 1 );
    letters.insert(String::from("c"), 2 );
    letters.insert(String::from("d"), 3 );
    letters.insert(String::from("e"), 4 );

    let converted_playlist = convert_to_num(test_vec, &letters);
    assert_eq!(converted_playlist, vec![[0,1,2],[1,2,3], [2,3,4]]);

}


#[test]
pub fn component_test(){
    let edges: Vec<(usize,usize)> = vec![(0,1),(0,2),(1,2),(2,4),(0,4),(5,7),(6,8)];
    let g = Graph::create_graph(9, &edges);
    let mut component: Vec<Option<usize>> = vec![None;9];
    let mut component_count = 0;
    for v in 0..9 {
        if let None = component[v] {
            component_count += 1;
            mark_component_bfs(v, &g, &mut component, component_count);
        }

    }
    assert_eq!(component, vec![Some(1),Some(1),Some(1),Some(2),Some(1),Some(3),Some(4),Some(3),Some(4)]);

}
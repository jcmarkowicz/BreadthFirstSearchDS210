use std::io;
use std::collections::HashMap;
mod read_file;
use std::collections::VecDeque;
type AdjacencyLists = Vec<Vec<Vertex>>;
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
struct Graph { //builds graph using material from lecture 28
    n: usize, 
    outedges: AdjacencyLists,
}
pub fn reverse_edges(list:&ListOfEdges)-> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}
impl Graph {
    pub fn add_directed_edges(&mut self,edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_undirected(n:usize,edges:&ListOfEdges)-> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
    fn create_directed(n:usize,edges:&ListOfEdges)-> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
}
fn main(){
    println!("Breadth First search options"); 
    println!("1:Choose a node and find it distance to another node");
    println!("2:Display the average distance from a node to all other nodes ");
    println!("3:Display a nodes list of connections");
    println!("4:6 degrees of seperation: Average amount of edges between all nodes");
    println!("5:Display top five strongest connected nodes");
    println!("Enter corresponding number");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).ok().expect("Failed to read line"); //readline function
    let input:i32= input_line.trim().parse().expect("Input not an integer");//parse string into integer
    println!("Choose the data set type Enron or Facebook");
    let mut dataset= String::new();
    io::stdin().read_line(&mut dataset).ok().expect("Failed to read line");
    let (text_file, num) = match dataset.trim() { //match user input to desired dataset
        "Enron" => ("email-Enron.txt", 36692), //text file and num vertices 
        "Facebook" => ("facebook_combined.txt", 4039),
        _ => {
            println!("Invalid dataset");
            return;
        }
    };
    let edges = read_file::read_file(text_file);//create a vector of all edges
    let g = Graph::create_undirected(num,&edges);//create undirected graph for both datasets
    let mut line = String::new();
    match input { // implemented match statement for user selection
        1 => {
            println!("Enter start vertex then target vertex");
            io::stdin().read_line(&mut line).expect("Failed to read line");
            let vertex: usize = line.trim().parse().expect("Input not an integer");
            let mut line2 = String::new();
            io::stdin().read_line(&mut line2).expect("Failed to read line");
            let target: usize = line2.trim().parse().expect("Input not an integer"); //get user start and end nodes for distance function
            compute_and_print_distance_bfs(vertex, target, &g);
        }
        2 => {
            println!("Enter start vertex");
            io::stdin().read_line(&mut line).ok().expect("Failed to read line");
            let vertex: usize = line.trim().parse().expect("Input not an integer");
            println!("Average distance to all:{}", compute_avg_length_to_all(vertex, &g));//compute average distance to random node
        }
        3 => {
            println!("Enter start vertex");
            io::stdin().read_line(&mut line).ok().expect("Failed to read line");
            let vertex: usize = line.trim().parse().expect("Input not an integer");
            print_outedges(vertex,&g); //print all connections
        }
        4 => {
            let mut total: f32 = 0.0; //sum of total average distances per each vertice
            for i in 0..g.n { //iterate through all vertices
                total += compute_avg_length_to_all(i, &g);
            }
            let six_degrees = total / g.n as f32; //divide by num vertices to get total average 
            println!("{}", six_degrees);
        }
        5 => {
            let mut map = HashMap::new(); //create hashmap to connect each vertice to its average distance to all
            for i in 0..g.n {
                let avg_distance = compute_avg_length_to_all(i, &g);
                map.insert(i, avg_distance); //insert the pair
            }
            let mut sorted_vec: Vec<(&usize, &f32)> = map.iter().collect(); //create a vector containing hashmap pairs
            sorted_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // sort each tuple in ascending order by lowest average distance traveled to all 
            let first_five = &sorted_vec[..5]; //slice the first five pairs 
            println!("{:?}", first_five);
        }
        _ => println!("Invalid input"),
    }
}
fn print_outedges(start: Vertex, graph: &Graph) {
    let mut distance: Vec<Option<u32>> = vec![None;graph.n]; //create vector containing an option for each vertice
    distance[start] = Some(0); //index the starting vertex and set intial distance traveled to zero
    let mut queue: VecDeque<Vertex> = VecDeque::new();//initialize a queue data structure 
    queue.push_back(start); // insert the start vertex into the queue 
    while let Some(v) = queue.pop_front() { //choose the vertex at the head of the queue 
        for u in graph.outedges[v].iter() { //iterate through each connected vertice 
            println!{"{}",u}
        }
    }
}
fn compute_and_print_distance_bfs(start: Vertex,end:Vertex, graph: &Graph) {
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {  // if node has not been visited mark it with option none to some 
                distance[*u] = Some(distance[v].unwrap() + 1); // add one to the sum distance 
                if u == &end{ //if targeted node is found 
                    return print!("Distance from{} to {} is {}",start,end,distance[end].unwrap());//print distance start to end 
                }
                else{
                    queue.push_back(*u);
                }
            }
        }
    }
}
fn compute_avg_length_to_all(start: Vertex, graph: &Graph)->f32 {
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    let sum: f32 = distance.iter().filter_map(|x| x.map(|val| val as f32)).sum(); //get the sum of the distance vector which contains the total distance to reach all nodes from start node 
    let avg:f32 = sum/graph.n as f32; //divide total distance by num vertices to get the average distance 
    return avg;
}
fn tests(){ //assert_eq! tests 
    let list = vec![(0, 1), (1, 2), (2, 3)];//create vector of connected edges 
    let expected_result = vec![(1, 0), (2, 1), (3, 2)]; //expected result of reversing list
    let result = reverse_edges(&list);
    assert_eq!(result, expected_result);

    let mut g = Graph{n: 4, outedges: vec![vec![]; 4]}; //intialize graph
    let edges = vec![(0, 1), (1, 2), (2, 3)]; //create edges
    g.add_directed_edges(&edges);
    let expected_result = vec![vec![1], vec![2], vec![3], vec![]]; //expected directed connections per vertice
    assert_eq!(g.outedges, expected_result);

    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let g = Graph::create_undirected(4, &edges);
    let expected_result = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];//expected undirected connections per vertice
    assert_eq!(g.outedges, expected_result);
}





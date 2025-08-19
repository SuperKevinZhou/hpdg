use std::{collections::{hash_map::Entry, HashMap}, fmt};
use rand::{distr::{weighted::WeightedIndex, Distribution}, rng, rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct Edge {
    u: usize,
    v: usize,
    w: i64,
    weighted: bool,
}

impl Edge {
    pub fn new(u: usize, v: usize, w: Option<i64>) -> Edge {
        if let Some(w) = w {
            Edge { u, v, w, weighted: true }
        } else {
            Edge { u, v, w: 0, weighted: false }
        }
    }
}

// impl Edge {
//     pub fn unweighted_edge(&self) -> String {
//         format!("{} {}", self.u, self.v)
//     }
// }

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.weighted {
            write!(f, "{} {} {}", self.u, self.v, self.w)
        } else {
            write!(f, "{} {}", self.u, self.v)
        }
    }
}

impl From<Edge> for (usize, usize) {
    fn from(val: Edge) -> Self {
        (val.u, val.v)
    }
}

impl Into<Edge> for (usize, usize) {
    fn into(self) -> Edge {
        Edge { u: self.0, v: self.1, w: 0, weighted: false }
    }
}

impl Into<Edge> for (u64, u64) {
    fn into(self) -> Edge {
        Edge {
            u: self.0 as usize,
            v: self.1 as usize,
            w: 0,
            weighted: false
        }
    }
}


impl Into<Edge> for (u32, u32) {
    fn into(self) -> Edge {
        Edge {
            u: self.0 as usize,
            v: self.1 as usize,
            w: 0,
            weighted: false,
        }
    }
}

impl Into<Edge> for (isize, isize) {
    fn into(self) -> Edge {
        Edge {
            u: self.0 as usize,
            v: self.1 as usize,
            w: 0,
            weighted: false,
        }
    }
}

impl Into<Edge> for (i64, i64) {
    fn into(self) -> Edge {
        Edge {
            u: self.0 as usize,
            v: self.1 as usize,
            w: 0,
            weighted: false,
        }
    }
}

impl Into<Edge> for (i32, i32) {
    fn into(self) -> Edge {
        Edge {
            u: self.0 as usize,
            v: self.1 as usize,
            w: 0,
            weighted: false,
        }
    }
}

pub struct SwitchGraph {
    directed: bool,
    edges: HashMap<(usize, usize), usize>,
}

impl SwitchGraph {
    #[allow(private_bounds)]
    pub fn new<I, E>(edges: I, directed: bool) -> SwitchGraph
    where
        I: IntoIterator<Item = E>,
        E: Into<Edge>
    {
        let mut graph = SwitchGraph {
            directed,
            edges: HashMap::new(),
        };

        for (u, v) in edges.into_iter().map(|e: E| { Into::<Edge>::into(e).into() }) {
            graph.insert(u, v);
        }

        graph
    }

    pub fn insert(&mut self, u: usize, v: usize) {
        self.insert_single(u, v);
        
        if !self.directed && u != v {
            self.insert_single(v, u);
        }
    }

    pub fn remove(&mut self, u: usize, v: usize) {
        self.remove_single(u, v);
        
        if !self.directed && u != v {
            self.remove_single(v, u);
        }
    }

    fn insert_single(&mut self, u: usize, v: usize) {
        *self.edges.entry((u, v)).or_insert(0) += 1;
    }

    fn remove_single(&mut self, u: usize, v: usize) {
        if let Entry::Occupied(mut entry) = self.edges.entry((u, v)) {
            *entry.get_mut() -= 1;
            if *entry.get() == 0 {
                entry.remove();
            }
        }
    }

    pub fn switch(&mut self, self_loop: bool, repeated_edges: bool) -> bool {
        let normalized_edges: Vec<((usize, usize), usize)> = self.get_normalized_edges();
        
        if normalized_edges.len() < 2 {
            return false;
        }

        let weights: Vec<usize> = normalized_edges.iter().map(|(_, count)| *count).collect();
        let total_weight: usize = weights.iter().sum();
        
        if total_weight < 2 {
            return false;
        }

        let mut rng = rng();
        
        let first_index = WeightedIndex::new(&weights)
            .ok()
            .map(|dist| dist.sample(&mut rng))
            .unwrap_or(0);
        let &(mut e1, _) = &normalized_edges[first_index];
        
        let second_index = WeightedIndex::new(&weights)
            .ok()
            .map(|dist| dist.sample(&mut rng))
            .unwrap_or(0);
        let &(mut e2, _) = &normalized_edges[second_index];

        if !self.directed {
            e1 = (e1.0.min(e1.1), e1.0.max(e1.1));
            e2 = (e2.0.min(e2.1), e2.0.max(e2.1));
        }

        let (x1, y1) = e1;
        let (x2, y2) = e2;

        if self_loop {
            if x1 == x2 || y1 == y2 {
                return false;
            }
        } else {
            let set1 = [x1, y1];
            let set2 = [x2, y2];
            if set1.iter().any(|v| set2.contains(v)) {
                return false;
            }
        }

        if !repeated_edges
            && (self.edges.contains_key(&(x1, y2))) || self.edges.contains_key(&(x2, y1)) {
                return false;
        }

        self.remove(x1, y1);
        self.insert(x1, y2);
        self.remove(x2, y2);
        self.insert(x2, y1);

        true
    }

    fn get_normalized_edges(&self) -> Vec<((usize, usize), usize)> {
        self.edges
            .iter()
            .map(|(&(u, v), &count)| {
                if self.directed {
                    ((u, v), count)
                } else {
                    ((u.min(v), u.max(v)), count)
                }
            })
            .collect()
    }

    pub fn from_directed_degree_sequence(
        degree_sequence: &[(usize, usize)],
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Self, &'static str> {
        if degree_sequence.iter().any(|&(out, in_)| out == 0 && in_ > 0) {
            return Err("Degree sequence is not graphical: some vertices have zero out-degree but positive in-degree");
        }
        
        if degree_sequence.iter().any(|&(out, in_)| in_ == 0 && out > 0) {
            return Err("Degree sequence is not graphical: some vertices have zero in-degree but positive out-degree");
        }
        
        let total_out: usize = degree_sequence.iter().map(|&(out, _)| out).sum();
        let total_in: usize = degree_sequence.iter().map(|&(_, in_)| in_).sum();
        
        if total_out != total_in {
            return Err("Degree sequence is not graphical: total out-degree != total in-degree");
        }
        
        if degree_sequence.is_empty() {
            return Ok(SwitchGraph::new(Vec::<(usize, usize)>::new(), true));
        }

        let mut vertices: Vec<(usize, usize, usize)> = degree_sequence
            .iter()
            .enumerate()
            .map(|(i, &(out, in_))| (out, in_, i))
            .collect();
        
        vertices.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

        let mut graph = SwitchGraph::new(Vec::<(usize, usize)>::new(), true);

        loop {
            let candidate = vertices.iter_mut()
                .find(|(_, in_deg, _)| *in_deg > 0);
            
            let (_, in_deg, vto) = match candidate {
                Some(tuple) => tuple,
                None => break,
            };
            
            let in_deg_val = *in_deg;
            let vto_val = *vto;
            *in_deg = 0;
            
            let mut current_in_deg = in_deg_val;
            let mut j = 0;
            
            while current_in_deg > 0 && j < vertices.len() {
                let (out_deg, _, vfrom) = &mut vertices[j];
                
                if *out_deg == 0 {
                    j += 1;
                    continue;
                }
                
                if !self_loop && *vfrom == vto_val {
                    j += 1;
                    continue;
                }
                
                graph.insert(*vfrom, vto_val);
                *out_deg -= 1;
                current_in_deg -= 1;
                
                if !repeated_edges {
                    j += 1;
                }
            }
            
            if current_in_deg > 0 {
                return Err("Degree sequence is not graphical...");
            }
            
            vertices.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        }

        Ok(graph)
    }

    pub fn from_undirected_degree_sequence(
        degree_sequence: &[usize],
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Self, &'static str> {
        let total_degree: usize = degree_sequence.iter().sum();
        if total_degree % 2 != 0 {
            return Err("Degree sequence is not graphical: total degree must be even");
        }
        
        if degree_sequence.is_empty() {
            return Ok(SwitchGraph::new(Vec::<(usize, usize)>::new(), false));
        }

        let mut vertices: Vec<(usize, usize)> = degree_sequence
            .iter()
            .enumerate()
            .map(|(i, &deg)| (deg, i))
            .collect();
        
        vertices.sort_by(|a, b| b.0.cmp(&a.0));

        let mut edges = Vec::new();

        while !vertices.is_empty() && vertices[0].0 > 0 {
            let (deg, v) = vertices[0];
            vertices[0].0 = 0;
            
            let mut current_deg = deg;
            let mut j = 1;
            
            if self_loop && current_deg > 1 {
                while current_deg > 1 {
                    edges.push((v, v));
                    current_deg -= 2;
                    
                    if !repeated_edges {
                        break;
                    }
                }
            }
            
            while current_deg > 0 && j < vertices.len() {
                let (other_deg, other_v) = &mut vertices[j];
                
                if *other_deg == 0 {
                    j += 1;
                    continue;
                }
                
                edges.push((v, *other_v));
                current_deg -= 1;
                *other_deg -= 1;
                
                if !repeated_edges {
                    j += 1;
                }
            }
            
            if current_deg > 0 {
                return Err("Degree sequence is not graphical: unable to satisfy degree requirements");
            }
            
            vertices.retain(|&(deg, _)| deg > 0);
            vertices.sort_by(|a, b| b.0.cmp(&a.0));
        }

        Ok(SwitchGraph::new(edges, false))
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.edges
            .iter()
            .flat_map(|(&(u, v), &count)| std::iter::repeat_n((u, v), count))
    }
}

pub struct Graph {
    directed: bool,
    edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new(point_count: usize, directed: bool) -> Graph {
        let mut graph = Graph {
            directed,
            edges: HashMap::new(),
        };

        for point in 1..=point_count {
            graph.edges.insert(point, Vec::new());
        }

        graph
    }
}

impl Graph {
    pub fn iter_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
            .flat_map(|v| v.iter())
            .filter(|e| { e.v >= e.u || self.directed })
    }

    pub fn iter_edges_mut(&mut self) -> impl Iterator<Item = &mut Edge> {
        self.edges.values_mut()
            .flat_map(|v| v.iter_mut())
            .filter(|e| { e.v >= e.u || self.directed })
    }

    pub fn edge_count(&self) -> usize {
        self.iter_edges().count()
    }
    
    pub fn add_single_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        self.edges
            .entry(u)
            .and_modify(|g| { g.push(Edge::new(u, v, w)) })
            .or_insert(vec![Edge::new(u, v, w)]);
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        // let w = w.unwrap_or(1);
        // self.add_single_edge(u, v, w);

        // if (!self.directed) && u != v {
        //     self.add_single_edge(v, u, w);
        // }
        if let Some(w) = w {
            self.add_single_edge(u, v, Some(w));
            if !self.directed && u != v {
                self.add_single_edge(v, u, Some(w));
            }
        } else {
            self.add_single_edge(u, v, None);
            if !self.directed && u != v {
                self.add_single_edge(v, u, None);
            }
        }
    }

    pub fn to_string(&self, shuffle: bool, line_reserve: Option<usize>, edge_display_function: Option<Box<dyn Fn(&Edge) -> String>>) -> String {
        let mut rng = rng();
        let edge_display_function = edge_display_function.unwrap_or_else(|| { Box::new(|e: &Edge| e.to_string()) });
        let mut buf: Vec<String> = Vec::new();
        buf.reserve(self.edge_count() * line_reserve.unwrap_or(6));

        if shuffle {
            let mut new_node_id: Vec<usize> = (1..=self.edges.keys().count()).collect();
            new_node_id.shuffle(&mut rng);
            let mut edge_buf: Vec<Edge> = Vec::new();
            for edge in self.iter_edges() {
                edge_buf.push(Edge::new(new_node_id[edge.u - 1], new_node_id[edge.v - 1], if edge.weighted { Some(edge.w) } else { None }));
            }
            edge_buf.shuffle(&mut rng);
            // for edge in edge_buf {
            //     if !self.directed && rng.random_bool(0.5) {

            //     }
            // }
            edge_buf.iter_mut()
                .for_each(|e| {
                    if !self.directed && rng.random_bool(0.5) {
                        let tmpu = e.u;
                        let tmpv = e.v;
                        e.u = tmpv;
                        e.v = tmpu;
                    }
                });
            for edge in &edge_buf {
                buf.push(edge_display_function(edge));
            }
        } else {
            for edge in self.iter_edges() {
                buf.push(edge_display_function(edge));
            }
        }
        buf.join("\n")
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(false, None, None))
    }
}

// impl Graph {
//     fn tree(
//         point_count: usize,
//         chain: f64,
//         flower: f64,
//         weight_limit: (i64, i64),
//         directed: bool,
//         weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
//         father_gen: Option<Box<dyn FnMut(&mut ThreadRng, usize) -> usize>>
//     ) -> Graph {
//         todo!()
//     }
// }

impl Graph {
    pub fn tree(
        point_count: usize,
        chain: f64,
        flower: f64,
        weight_limit: (i64, i64),
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
        father_gen: Option<Box<dyn FnMut(&mut ThreadRng, usize) -> usize>>,
    ) -> Graph {
        assert!(
            (0.0..=1.0).contains(&chain) && (0.0..=1.0).contains(&flower),
            "chain and flower must be between 0.0 and 1.0"
        );
        assert!(
            chain + flower <= 1.0,
            "chain plus flower must be less than or equal to 1.0"
        );
        
        let mut rng = rng();
        let (min_weight, max_weight) = weight_limit;
        
        let default_weight_gen = |rng: &mut ThreadRng| rng.random_range(min_weight..=max_weight);
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));
        
        let default_father_gen = |rng: &mut ThreadRng, cur| {
            if cur <= 1 {
                1
            } else {
                rng.random_range(1..cur)
            }
        };
        let mut father_gen = father_gen.unwrap_or_else(|| Box::new(move |rng, cur| default_father_gen(rng, cur)));
        
        let total_edges = point_count.saturating_sub(1);
        let chain_count = ((total_edges as f64) * chain).round() as usize;
        let flower_count = ((total_edges as f64) * flower).round() as usize;
        // let random_count = total_edges.saturating_sub(chain_count + flower_count);
        
        let mut graph = Graph::new(point_count, directed);
        
        let chain_end = chain_count + 1;
        for i in 2..=chain_end {
            let weight = weight_gen(&mut rng);
            graph.add_edge(i - 1, i, Some(weight));
        }
        
        let flower_start = chain_end + 1;
        let flower_end = (flower_start + flower_count).min(point_count + 1);
        for i in flower_start..flower_end {
            let weight = weight_gen(&mut rng);
            graph.add_edge(1, i, Some(weight));
        }
        
        let random_start = flower_end;
        for i in random_start..=point_count {
            if i == 1 { continue; }
            let father = father_gen(&mut rng, i);
            let weight = weight_gen(&mut rng);
            graph.add_edge(father, i, Some(weight));
        }
        
        graph
    }
}
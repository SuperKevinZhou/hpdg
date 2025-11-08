use std::{collections::{hash_map::Entry, HashMap}, fmt};
use rand::{distr::{weighted::WeightedIndex, Distribution}, rng, rngs::ThreadRng, seq::SliceRandom, Rng};

#[derive(Clone, Debug)]
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

    pub fn is_weighted(&self) -> bool {
        self.weighted
    }

    pub fn weight(&self) -> Option<i64> {
        if self.weighted {
            Some(self.w)
        } else {
            None
        }
    }

    pub fn format_unweighted(&self) -> String {
        format!("{} {}", self.u, self.v)
    }

    pub fn format_weighted(&self) -> String {
        format!("{} {} {}", self.u, self.v, self.w)
    }

    pub fn format_default(&self) -> String {
        if self.weighted {
            self.format_weighted()
        } else {
            self.format_unweighted()
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
        write!(f, "{}", self.format_default())
    }
}

impl From<Edge> for (usize, usize) {
    fn from(val: Edge) -> Self {
        (val.u, val.v)
    }
}

impl From<(usize, usize)> for Edge {
    fn from(value: (usize, usize)) -> Self {
        Edge { u: value.0, v: value.1, w: 0, weighted: false }
    }
}

impl From<(usize, usize, i64)> for Edge {
    fn from(value: (usize, usize, i64)) -> Self {
        Edge { u: value.0, v: value.1, w: value.2, weighted: true }
    }
}

impl Into<Edge> for (usize, usize) {
    fn into(self) -> Edge {
        Edge { u: self.0, v: self.1, w: 0, weighted: false }
    }
}

impl From<(u64, u64)> for Edge {
    fn from(value: (u64, u64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: 0, weighted: false }
    }
}

impl From<(u64, u64, i64)> for Edge {
    fn from(value: (u64, u64, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: value.2, weighted: true }
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

impl From<(u32, u32)> for Edge {
    fn from(value: (u32, u32)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: 0, weighted: false }
    }
}

impl From<(u32, u32, i64)> for Edge {
    fn from(value: (u32, u32, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: value.2, weighted: true }
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

impl From<(isize, isize)> for Edge {
    fn from(value: (isize, isize)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: 0, weighted: false }
    }
}

impl From<(isize, isize, i64)> for Edge {
    fn from(value: (isize, isize, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: value.2, weighted: true }
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

impl From<(i64, i64)> for Edge {
    fn from(value: (i64, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: 0, weighted: false }
    }
}

impl From<(i64, i64, i64)> for Edge {
    fn from(value: (i64, i64, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: value.2, weighted: true }
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

impl From<(i32, i32)> for Edge {
    fn from(value: (i32, i32)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: 0, weighted: false }
    }
}

impl From<(i32, i32, i64)> for Edge {
    fn from(value: (i32, i32, i64)) -> Self {
        Edge { u: value.0 as usize, v: value.1 as usize, w: value.2, weighted: true }
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

/// A switchable multigraph to perform edge-switch operations.
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

        let mut second_index = first_index;
        if normalized_edges.len() > 1 {
            for _ in 0..5 {
                second_index = WeightedIndex::new(&weights)
                    .ok()
                    .map(|dist| dist.sample(&mut rng))
                    .unwrap_or(first_index);
                if second_index != first_index {
                    break;
                }
            }
            if second_index == first_index {
                second_index = (first_index + 1) % normalized_edges.len();
            }
        }
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
            && (self.edges.contains_key(&(x1, y2)) || self.edges.contains_key(&(x2, y1)))
        {
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
        Self::validate_directed_degree_sequence(degree_sequence)?;
        
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

    pub fn from_directed_degree_sequence_simple(
        degree_sequence: &[(usize, usize)],
    ) -> Result<Self, &'static str> {
        Self::from_directed_degree_sequence(degree_sequence, false, false)
    }

    pub fn from_undirected_degree_sequence(
        degree_sequence: &[usize],
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Self, &'static str> {
        Self::validate_undirected_degree_sequence(degree_sequence)?;
        
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

    pub fn validate_directed_degree_sequence(
        degree_sequence: &[(usize, usize)],
    ) -> Result<(), &'static str> {
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
        Ok(())
    }

    pub fn validate_undirected_degree_sequence(
        degree_sequence: &[usize],
    ) -> Result<(), &'static str> {
        let total_degree: usize = degree_sequence.iter().sum();
        if total_degree % 2 != 0 {
            return Err("Degree sequence is not graphical: total degree must be even");
        }
        Ok(())
    }

    pub fn from_undirected_degree_sequence_simple(
        degree_sequence: &[usize],
    ) -> Result<Self, &'static str> {
        Self::from_undirected_degree_sequence(degree_sequence, false, false)
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.edges
            .iter()
            .flat_map(|(&(u, v), &count)| std::iter::repeat_n((u, v), count))
    }

    pub fn iter_edges_unique(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.edges.keys().cloned()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().sum()
    }

    pub fn edge_count_unique(&self) -> usize {
        self.edges.len()
    }
}

pub struct Graph {
    directed: bool,
    edges: HashMap<usize, Vec<Edge>>,
}

#[derive(Debug, Clone)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub directed: bool,
    pub min_degree: usize,
    pub max_degree: usize,
    pub avg_degree: f64,
}

#[derive(Debug, Clone)]
pub struct GraphGenOptions {
    pub directed: bool,
    pub self_loop: bool,
    pub repeated_edges: bool,
}

impl Default for GraphGenOptions {
    fn default() -> Self {
        Self {
            directed: false,
            self_loop: false,
            repeated_edges: false,
        }
    }
}

pub enum DegreeSequence<'a> {
    Directed(&'a [(usize, usize)]),
    Undirected(&'a [usize]),
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

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub fn with_nodes<I: IntoIterator<Item = usize>>(nodes: I, directed: bool) -> Graph {
        let mut graph = Graph {
            directed,
            edges: HashMap::new(),
        };
        for node in nodes {
            graph.edges.insert(node, Vec::new());
        }
        graph
    }

    pub fn node_count(&self) -> usize {
        self.edges.len()
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub fn is_valid(&self) -> bool {
        let nodes: std::collections::HashSet<usize> = self.edges.keys().cloned().collect();
        for edge in self.iter_edges_all() {
            if !nodes.contains(&edge.u) || !nodes.contains(&edge.v) {
                return false;
            }
        }
        true
    }

    pub fn stats(&self) -> GraphStats {
        let node_count = self.node_count();
        let edge_count = self.edge_count();
        let mut min_degree = usize::MAX;
        let mut max_degree = 0usize;
        let mut total_degree = 0usize;

        for (_node, edges) in &self.edges {
            let degree = edges.len();
            min_degree = min_degree.min(degree);
            max_degree = max_degree.max(degree);
            total_degree += degree;
        }

        if node_count == 0 {
            min_degree = 0;
        }

        let avg_degree = if node_count == 0 {
            0.0
        } else {
            total_degree as f64 / node_count as f64
        };

        GraphStats {
            node_count,
            edge_count,
            directed: self.directed,
            min_degree,
            max_degree,
            avg_degree,
        }
    }

    pub fn to_adj_list_string(&self, sep: &str) -> String {
        let mut nodes: Vec<usize> = self.edges.keys().cloned().collect();
        nodes.sort_unstable();
        let mut lines = Vec::with_capacity(nodes.len());

        for node in nodes {
            let mut items = Vec::new();
            if let Some(edges) = self.edges.get(&node) {
                for edge in edges {
                    if edge.weighted {
                        items.push(format!("{}:{}", edge.v, edge.w));
                    } else {
                        items.push(edge.v.to_string());
                    }
                }
            }
            lines.push(format!("{}: {}", node, items.join(sep)));
        }

        lines.join("\n")
    }

    pub fn to_matrix(&self, default: i64) -> (Vec<usize>, Vec<Vec<i64>>) {
        let mut nodes: Vec<usize> = self.edges.keys().cloned().collect();
        nodes.sort_unstable();
        let n = nodes.len();
        let mut index = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            index.insert(*node, i);
        }

        let mut matrix = vec![vec![default; n]; n];
        for edge in self.iter_edges_all() {
            if let (Some(&i), Some(&j)) = (index.get(&edge.u), index.get(&edge.v)) {
                let value = if edge.weighted { edge.w } else { 1 };
                matrix[i][j] = value;
            }
        }

        (nodes, matrix)
    }
}

impl Graph {
    pub fn iter_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
            .flat_map(|v| v.iter())
            .filter(|e| { e.v >= e.u || self.directed })
    }

    pub fn iter_edges_all(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values().flat_map(|v| v.iter())
    }

    pub fn iter_edges_mut(&mut self) -> impl Iterator<Item = &mut Edge> {
        self.edges.values_mut()
            .flat_map(|v| v.iter_mut())
            .filter(|e| { e.v >= e.u || self.directed })
    }

    pub fn iter_edges_all_mut(&mut self) -> impl Iterator<Item = &mut Edge> {
        self.edges.values_mut().flat_map(|v| v.iter_mut())
    }

    pub fn edge_count(&self) -> usize {
        self.iter_edges().count()
    }

    pub fn edge_count_all(&self) -> usize {
        self.iter_edges_all().count()
    }
    
    pub fn add_single_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        self.edges
            .entry(u)
            .and_modify(|g| { g.push(Edge::new(u, v, w)) })
            .or_insert(vec![Edge::new(u, v, w)]);
    }

    fn add_directed_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        self.add_single_edge(u, v, w);
    }

    fn add_undirected_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        self.add_single_edge(u, v, w);
        if u != v {
            self.add_single_edge(v, u, w);
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: Option<i64>) {
        if self.directed {
            self.add_directed_edge(u, v, w);
        } else {
            self.add_undirected_edge(u, v, w);
        }
    }

    pub fn add_edges<I, E>(&mut self, edges: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<Edge>,
    {
        for edge in edges {
            let edge: Edge = edge.into();
            if edge.weighted {
                self.add_edge(edge.u, edge.v, Some(edge.w));
            } else {
                self.add_edge(edge.u, edge.v, None);
            }
        }
    }

    pub fn add_edge_with_weight<F>(&mut self, u: usize, v: usize, mut weight_gen: F)
    where
        F: FnMut() -> i64,
    {
        let w = weight_gen();
        self.add_edge(u, v, Some(w));
    }

    pub fn add_edges_with_weight<I, F>(&mut self, edges: I, mut weight_gen: F)
    where
        I: IntoIterator<Item = (usize, usize)>,
        F: FnMut() -> i64,
    {
        for (u, v) in edges {
            let w = weight_gen();
            self.add_edge(u, v, Some(w));
        }
    }

    pub fn chain(
        point_count: usize,
        weight_limit: Option<(i64, i64)>,
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        let mut rng = rng();
        let is_unweighted = weight_limit.is_none();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::new(point_count, directed);
        for i in 2..=point_count {
            let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
            graph.add_edge(i - 1, i, weight);
        }
        graph
    }

    pub fn flower(
        point_count: usize,
        weight_limit: Option<(i64, i64)>,
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        let mut rng = rng();
        let is_unweighted = weight_limit.is_none();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::new(point_count, directed);
        for i in 2..=point_count {
            let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
            graph.add_edge(1, i, weight);
        }
        graph
    }

    pub fn to_string(&self, shuffle: bool, line_reserve: Option<usize>, edge_display_function: Option<Box<dyn Fn(&Edge) -> String>>) -> String {
        let mut rng = rng();
        let edge_display_function = edge_display_function.unwrap_or_else(|| { Box::new(|e: &Edge| e.format_default()) });
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

    pub fn to_string_with<F>(&self, shuffle: bool, line_reserve: Option<usize>, edge_display_function: F) -> String
    where
        F: Fn(&Edge) -> String,
    {
        self.to_string(shuffle, line_reserve, Some(Box::new(edge_display_function)))
    }

    pub fn shuffle_edges(&mut self) {
        let mut rng = rng();
        for edges in self.edges.values_mut() {
            edges.shuffle(&mut rng);
        }
    }

    pub fn shuffle_labels(&self) -> Graph {
        let mut rng = rng();
        let mut nodes: Vec<usize> = self.edges.keys().cloned().collect();
        let mut new_nodes = nodes.clone();
        new_nodes.shuffle(&mut rng);

        let mapping: HashMap<usize, usize> = nodes
            .iter()
            .zip(new_nodes.iter())
            .map(|(&old, &new)| (old, new))
            .collect();

        let mut graph = Graph::with_nodes(new_nodes, self.directed);
        for edge in self.iter_edges_all() {
            let u = mapping[&edge.u];
            let v = mapping[&edge.v];
            let w = if edge.weighted { Some(edge.w) } else { None };
            graph.add_single_edge(u, v, w);
        }

        graph
    }

    pub fn edges_random_oriented(&self) -> Vec<Edge> {
        let mut rng = rng();
        let mut edges: Vec<Edge> = self.iter_edges().cloned().collect();
        if !self.directed {
            for edge in edges.iter_mut() {
                if rng.random_bool(0.5) {
                    let tmp = edge.u;
                    edge.u = edge.v;
                    edge.v = tmp;
                }
            }
        }
        edges
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
        weight_limit: Option<(i64, i64)>,
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
        let is_unweighted = weight_limit.is_none();
        let use_custom_weight_gen = weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
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
        
        let mut graph = Graph::new(point_count, directed);

        let chain_end = chain_count + 1;
        let flower_start = chain_end + 1;
        let flower_end = (flower_start + flower_count).min(point_count + 1);

        if !use_custom_weight_gen {
            let chain_graph = Graph::chain(chain_end, weight_limit, directed, None);
            graph.add_edges(chain_graph.iter_edges_all().cloned());

            if flower_count > 0 {
                let flower_graph = Graph::flower(flower_count + 1, weight_limit, directed, None);
                let offset = chain_end.saturating_sub(1);
                for edge in flower_graph.iter_edges_all() {
                    let mut u = edge.u;
                    let mut v = edge.v;
                    if u != 1 { u += offset; }
                    if v != 1 { v += offset; }
                    let weight = if edge.weighted { Some(edge.w) } else { None };
                    graph.add_edge(u, v, weight);
                }
            }
        } else {
            for i in 2..=chain_end {
                let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
                graph.add_edge(i - 1, i, weight);
            }

            for i in flower_start..flower_end {
                let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
                graph.add_edge(1, i, weight);
            }
        }
        
        let random_start = flower_end;
        for i in random_start..=point_count {
            if i == 1 { continue; }
            let father = father_gen(&mut rng, i);
            let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
            graph.add_edge(father, i, weight);
        }
        
        graph
    }

    pub fn binary_tree(
        point_count: usize,
        left: f64,
        right: f64,
        weight_limit: Option<(i64, i64)>,
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(
            point_count > 0,
            "point_count must be above zero"
        );
        assert!(
            (0.0..=1.0).contains(&left) && (0.0..=1.0).contains(&right),
            "left and right must be between 0.0 and 1.0"
        );
        assert!(
            left + right <= 1.0,
            "left plus right must be less than or equal to 1.0"
        );
        
        let mut rng = rng();
        let is_unweighted = weight_limit.is_none();
        
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));
        
        let mut graph = Graph::new(point_count, directed);
        
        let mut can_left = vec![1];
        let mut can_right = vec![1];
        
        for node_id in 2..=point_count {
            let edge_pos: f64 = rng.random();
            
            let is_left = if edge_pos < left {
                true
            } else if edge_pos < left + right {
                false
            } else {
                let mid = left + right + (1.0 - left - right) / 2.0;
                edge_pos <= mid
            };
            
            let parent = if is_left {
                let idx = rng.random_range(0..can_left.len());
                can_left.swap_remove(idx)
            } else {
                let idx = rng.random_range(0..can_right.len());
                can_right.swap_remove(idx)
            };
            
            let weight = if is_unweighted { None } else { Some(weight_gen(&mut rng)) };
            graph.add_edge(parent, node_id, weight);
            can_left.push(node_id);
            can_right.push(node_id);
        }
        
        graph
    }

    pub fn binary_tree_with_side_weights(
        point_count: usize,
        left: f64,
        right: f64,
        left_weight_limit: Option<(i64, i64)>,
        right_weight_limit: Option<(i64, i64)>,
        directed: bool,
        left_weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
        right_weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(
            (0.0..=1.0).contains(&left) && (0.0..=1.0).contains(&right),
            "left and right must be between 0.0 and 1.0"
        );
        assert!(
            left + right <= 1.0,
            "left plus right must be less than or equal to 1.0"
        );

        let mut rng = rng();
        let default_left_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = left_weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
        let default_right_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = right_weight_limit.unwrap();
            rng.random_range(min_weight..=max_weight)
        };
        let mut left_weight_gen = left_weight_gen.unwrap_or_else(|| Box::new(default_left_gen));
        let mut right_weight_gen = right_weight_gen.unwrap_or_else(|| Box::new(default_right_gen));

        let mut graph = Graph::new(point_count, directed);

        let mut can_left = vec![1];
        let mut can_right = vec![1];

        for node_id in 2..=point_count {
            let edge_pos: f64 = rng.random();

            let is_left = if edge_pos < left {
                true
            } else if edge_pos < left + right {
                false
            } else {
                let mid = left + right + (1.0 - left - right) / 2.0;
                edge_pos <= mid
            };

            let parent = if is_left {
                let idx = rng.random_range(0..can_left.len());
                can_left.swap_remove(idx)
            } else {
                let idx = rng.random_range(0..can_right.len());
                can_right.swap_remove(idx)
            };

            let weight = if is_left {
                left_weight_limit.map(|_| left_weight_gen(&mut rng))
            } else {
                right_weight_limit.map(|_| right_weight_gen(&mut rng))
            };
            graph.add_edge(parent, node_id, weight);
            can_left.push(node_id);
            can_right.push(node_id);
        }

        graph
    }

    pub fn binary_tree_with_weight_gen<F>(
        point_count: usize,
        left: f64,
        right: f64,
        directed: bool,
        mut weight_gen: F,
    ) -> Graph
    where
        F: FnMut(&mut ThreadRng, usize, usize) -> i64,
    {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(
            (0.0..=1.0).contains(&left) && (0.0..=1.0).contains(&right),
            "left and right must be between 0.0 and 1.0"
        );
        assert!(
            left + right <= 1.0,
            "left plus right must be less than or equal to 1.0"
        );

        let mut rng = rng();
        let mut graph = Graph::new(point_count, directed);

        let mut can_left = vec![1];
        let mut can_right = vec![1];

        for node_id in 2..=point_count {
            let edge_pos: f64 = rng.random();

            let is_left = if edge_pos < left {
                true
            } else if edge_pos < left + right {
                false
            } else {
                let mid = left + right + (1.0 - left - right) / 2.0;
                edge_pos <= mid
            };

            let parent = if is_left {
                let idx = rng.random_range(0..can_left.len());
                can_left.swap_remove(idx)
            } else {
                let idx = rng.random_range(0..can_right.len());
                can_right.swap_remove(idx)
            };

            let weight = weight_gen(&mut rng, parent, node_id);
            graph.add_edge(parent, node_id, Some(weight));
            can_left.push(node_id);
            can_right.push(node_id);
        }

        graph
    }

    pub fn graph(
        point_count: usize,
        edge_count: usize,
        directed: bool,
        self_loop: bool,
        repeated_edges: bool,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::new(point_count, directed);
        let mut used: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut count = 0usize;

        while count < edge_count {
            let mut u = rng.random_range(1..=point_count);
            let mut v = rng.random_range(1..=point_count);
            if !self_loop && u == v {
                continue;
            }

            let key = if directed {
                (u, v)
            } else {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }
                (u, v)
            };

            if !repeated_edges && used.contains(&key) {
                continue;
            }

            let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
            graph.add_edge(u, v, weight);
            used.insert(key);
            count += 1;
        }

        graph
    }

    pub fn graph_with_options(
        point_count: usize,
        edge_count: usize,
        options: GraphGenOptions,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        Graph::graph(
            point_count,
            edge_count,
            options.directed,
            options.self_loop,
            options.repeated_edges,
            weight_limit,
            weight_gen,
        )
    }

    pub fn graph_with_weight_limit(
        point_count: usize,
        edge_count: usize,
        options: GraphGenOptions,
        weight_limit: (i64, i64),
    ) -> Graph {
        Graph::graph_with_options(point_count, edge_count, options, Some(weight_limit), None)
    }

    pub fn simple_graph(point_count: usize, edge_count: usize, directed: bool) -> Graph {
        Graph::graph(
            point_count,
            edge_count,
            directed,
            false,
            false,
            None,
            None,
        )
    }

    pub fn multigraph(
        point_count: usize,
        edge_count: usize,
        directed: bool,
        self_loop: bool,
    ) -> Graph {
        Graph::graph(
            point_count,
            edge_count,
            directed,
            self_loop,
            true,
            None,
            None,
        )
    }

    pub fn complete_graph(
        point_count: usize,
        directed: bool,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::new(point_count, directed);
        for u in 1..=point_count {
            let start = if directed { 1 } else { u + 1 };
            for v in start..=point_count {
                if u == v {
                    continue;
                }
                let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
                graph.add_edge(u, v, weight);
            }
        }
        graph
    }

    pub fn complete_bipartite(
        left_count: usize,
        right_count: usize,
        directed: bool,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(left_count > 0 && right_count > 0, "partition sizes must be above zero");
        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let total = left_count + right_count;
        let mut graph = Graph::new(total, directed);
        let left_nodes: Vec<usize> = (1..=left_count).collect();
        let right_nodes: Vec<usize> = ((left_count + 1)..=total).collect();

        for &u in &left_nodes {
            for &v in &right_nodes {
                let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
                graph.add_edge(u, v, weight);
            }
        }

        if directed {
            for &u in &right_nodes {
                for &v in &left_nodes {
                    let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
                    graph.add_edge(u, v, weight);
                }
            }
        }

        graph
    }

    pub fn k_regular_approx(
        point_count: usize,
        k: usize,
        directed: bool,
        self_loop: bool,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        let mut rng = rng();
        let mut graph = Graph::new(point_count, directed);

        if directed {
            for u in 1..=point_count {
                for _ in 0..k {
                    let v = rng.random_range(1..=point_count);
                    if !self_loop && u == v {
                        continue;
                    }
                    graph.add_edge(u, v, None);
                }
            }
            return graph;
        }

        let mut stubs = Vec::with_capacity(point_count.saturating_mul(k));
        for u in 1..=point_count {
            for _ in 0..k {
                stubs.push(u);
            }
        }
        stubs.shuffle(&mut rng);

        for pair in stubs.chunks(2) {
            if pair.len() < 2 {
                break;
            }
            let u = pair[0];
            let v = pair[1];
            if !self_loop && u == v {
                continue;
            }
            graph.add_edge(u, v, None);
        }

        graph
    }

    pub fn dag(
        point_count: usize,
        edge_count: usize,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        Self::dag_with_options(point_count, edge_count, false, weight_limit, weight_gen)
    }

    pub fn dag_with_options(
        point_count: usize,
        edge_count: usize,
        self_loop: bool,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(!self_loop, "DAG does not allow self loops");
        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::new(point_count, true);
        let mut used: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut count = 0usize;

        while count < edge_count {
            let u = rng.random_range(1..=point_count);
            let v = rng.random_range(1..=point_count);
            if u == v {
                continue;
            }
            let (from, to) = if u < v { (u, v) } else { (v, u) };
            if used.contains(&(from, to)) {
                continue;
            }
            let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
            graph.add_edge(from, to, weight);
            used.insert((from, to));
            count += 1;
        }

        graph
    }

    pub fn udag(
        point_count: usize,
        edge_count: usize,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(
            edge_count <= point_count.saturating_sub(1),
            "edge_count must be <= point_count - 1 for UDAG"
        );
        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut parent: Vec<usize> = (0..=point_count).collect();
        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        fn union(parent: &mut [usize], a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent[rb] = ra;
            }
        }

        let mut graph = Graph::new(point_count, false);
        let mut count = 0usize;
        while count < edge_count {
            let u = rng.random_range(1..=point_count);
            let v = rng.random_range(1..=point_count);
            if u == v {
                continue;
            }
            let ru = find(&mut parent, u);
            let rv = find(&mut parent, v);
            if ru == rv {
                continue;
            }
            let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
            graph.add_edge(u, v, weight);
            union(&mut parent, u, v);
            count += 1;
        }

        graph
    }

    pub fn connected(
        point_count: usize,
        edge_count: usize,
        directed: bool,
        weight_limit: Option<(i64, i64)>,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(
            edge_count >= point_count.saturating_sub(1),
            "edge_count must be >= point_count - 1 for connected graph"
        );

        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut graph = Graph::tree(point_count, 0.0, 0.0, weight_limit, directed, None, None);
        let mut used: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for edge in graph.iter_edges() {
            let mut u = edge.u;
            let mut v = edge.v;
            if !directed && u > v {
                std::mem::swap(&mut u, &mut v);
            }
            used.insert((u, v));
        }

        let mut count = graph.edge_count();
        while count < edge_count {
            let mut u = rng.random_range(1..=point_count);
            let mut v = rng.random_range(1..=point_count);
            if u == v {
                continue;
            }
            let key = if directed {
                (u, v)
            } else {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }
                (u, v)
            };
            if used.contains(&key) {
                continue;
            }
            let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
            graph.add_edge(u, v, weight);
            used.insert(key);
            count += 1;
        }

        graph
    }

    pub fn ensure_connected(&mut self) {
        let mut parent: HashMap<usize, usize> = self.edges.keys().map(|&k| (k, k)).collect();

        fn find(parent: &mut HashMap<usize, usize>, x: usize) -> usize {
            let p = *parent.get(&x).unwrap();
            if p != x {
                let root = find(parent, p);
                parent.insert(x, root);
            }
            *parent.get(&x).unwrap()
        }

        fn union(parent: &mut HashMap<usize, usize>, a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent.insert(rb, ra);
            }
        }

        for edge in self.iter_edges() {
            union(&mut parent, edge.u, edge.v);
        }

        let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
        for &node in self.edges.keys() {
            let root = find(&mut parent, node);
            components.entry(root).or_default().push(node);
        }

        let mut reps: Vec<usize> = components.values().map(|v| v[0]).collect();
        if reps.len() <= 1 {
            return;
        }
        let mut rng = rng();
        reps.shuffle(&mut rng);
        for pair in reps.windows(2) {
            let u = pair[0];
            let v = pair[1];
            self.add_edge(u, v, None);
        }
    }

    pub fn make_k_connected(&mut self, k: usize) {
        if k == 0 {
            return;
        }
        self.ensure_connected();
        if k == 1 {
            return;
        }
        let mut rng = rng();
        let nodes: Vec<usize> = self.edges.keys().cloned().collect();
        for _ in 0..(k - 1) * nodes.len().saturating_sub(1) {
            if nodes.len() < 2 {
                break;
            }
            let u = nodes[rng.random_range(0..nodes.len())];
            let v = nodes[rng.random_range(0..nodes.len())];
            if u == v {
                continue;
            }
            self.add_edge(u, v, None);
        }
    }

    pub fn forest(
        point_count: usize,
        tree_count: usize,
        weight_limit: Option<(i64, i64)>,
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
    ) -> Graph {
        assert!(point_count > 0, "point_count must be above zero");
        assert!(tree_count >= 1 && tree_count <= point_count, "invalid tree_count");

        let mut rng = rng();
        let use_weight = weight_limit.is_some() || weight_gen.is_some();
        let default_weight_gen = |rng: &mut ThreadRng| {
            let (min_weight, max_weight) = weight_limit.expect("weight_limit required for default generator");
            rng.random_range(min_weight..=max_weight)
        };
        let mut weight_gen = weight_gen.unwrap_or_else(|| Box::new(default_weight_gen));

        let mut parent: Vec<usize> = (0..=point_count).collect();
        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        fn union(parent: &mut [usize], a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent[rb] = ra;
            }
        }

        let mut graph = Graph::new(point_count, directed);
        let mut components = point_count;
        while components > tree_count {
            let u = rng.random_range(1..=point_count);
            let v = rng.random_range(1..=point_count);
            if u == v {
                continue;
            }
            let ru = find(&mut parent, u);
            let rv = find(&mut parent, v);
            if ru == rv {
                continue;
            }
            let weight = if use_weight { Some(weight_gen(&mut rng)) } else { None };
            graph.add_edge(u, v, weight);
            union(&mut parent, u, v);
            components -= 1;
        }

        graph
    }

    pub fn forest_with_repeats(
        point_count: usize,
        tree_count: usize,
        weight_limit: Option<(i64, i64)>,
        directed: bool,
        weight_gen: Option<Box<dyn FnMut(&mut ThreadRng) -> i64>>,
        repeat_times: usize,
    ) -> Graph {
        let mut graph = Graph::forest(point_count, tree_count, weight_limit, directed, weight_gen);
        if repeat_times == 0 {
            return graph;
        }

        let mut rng = rng();
        let edges: Vec<Edge> = graph.iter_edges().cloned().collect();
        if edges.is_empty() {
            return graph;
        }

        for _ in 0..repeat_times {
            let edge = edges[rng.random_range(0..edges.len())].clone();
            let weight = if edge.weighted { Some(edge.w) } else { None };
            graph.add_edge(edge.u, edge.v, weight);
        }

        graph
    }

    pub fn from_directed_degree_sequence(
        degree_sequence: &[(usize, usize)],
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Graph, &'static str> {
        let switch = SwitchGraph::from_directed_degree_sequence(
            degree_sequence,
            self_loop,
            repeated_edges,
        )?;
        let mut graph = Graph::new(degree_sequence.len(), true);
        for (u, v) in switch.iter_edges() {
            graph.add_edge(u, v, None);
        }
        Ok(graph)
    }

    pub fn from_undirected_degree_sequence(
        degree_sequence: &[usize],
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Graph, &'static str> {
        let switch = SwitchGraph::from_undirected_degree_sequence(
            degree_sequence,
            self_loop,
            repeated_edges,
        )?;
        let mut graph = Graph::new(degree_sequence.len(), false);
        for (u, v) in switch.iter_edges() {
            graph.add_edge(u, v, None);
        }
        Ok(graph)
    }

    pub fn from_degree_sequence(
        degree_sequence: DegreeSequence<'_>,
        self_loop: bool,
        repeated_edges: bool,
    ) -> Result<Graph, &'static str> {
        match degree_sequence {
            DegreeSequence::Directed(seq) => {
                Graph::from_directed_degree_sequence(seq, self_loop, repeated_edges)
            }
            DegreeSequence::Undirected(seq) => {
                Graph::from_undirected_degree_sequence(seq, self_loop, repeated_edges)
            }
        }
    }

    pub fn max_edge_count(point_count: usize, directed: bool, self_loop: bool) -> usize {
        if directed {
            if self_loop {
                point_count * point_count
            } else {
                point_count.saturating_mul(point_count.saturating_sub(1))
            }
        } else if self_loop {
            point_count.saturating_mul(point_count.saturating_add(1)) / 2
        } else {
            point_count.saturating_mul(point_count.saturating_sub(1)) / 2
        }
    }

    pub fn estimate_comb(n: usize, k: usize) -> f64 {
        if k > n {
            return 0.0;
        }
        if k == 0 || k == n {
            return 1.0;
        }
        let k = k.min(n - k);
        let mut res = 1.0f64;
        for i in 1..=k {
            res *= (n + 1 - i) as f64 / i as f64;
        }
        res
    }

    pub fn estimate_upperbound(point_count: usize, directed: bool, self_loop: bool) -> f64 {
        Graph::max_edge_count(point_count, directed, self_loop) as f64
    }

    pub fn hack_spfa(point_count: usize, weight_limit: Option<(i64, i64)>) -> Graph {
        assert!(point_count > 1, "point_count must be above one");
        let mut graph = Graph::new(point_count, true);
        let (min_w, max_w) = weight_limit.unwrap_or((-1, 1));

        for i in 1..point_count {
            let forward_w = 0.clamp(min_w, max_w);
            let backward_w = (-1).clamp(min_w, max_w);
            graph.add_edge(i, i + 1, Some(forward_w));
            graph.add_edge(i + 1, i, Some(backward_w));
        }

        graph
    }

    pub fn hack_spfa_with_options(
        point_count: usize,
        forward_weight: i64,
        backward_weight: i64,
        extra_edges: usize,
    ) -> Graph {
        assert!(point_count > 1, "point_count must be above one");
        let mut rng = rng();
        let mut graph = Graph::new(point_count, true);

        for i in 1..point_count {
            graph.add_edge(i, i + 1, Some(forward_weight));
            graph.add_edge(i + 1, i, Some(backward_weight));
        }

        for _ in 0..extra_edges {
            let v = rng.random_range(2..=point_count);
            graph.add_edge(1, v, Some(forward_weight));
        }

        graph
    }
}

pub struct GraphMatrix<T> {
    matrix: Vec<Vec<T>>,
    default: T,
}

impl<T: Clone> GraphMatrix<T> {
    pub fn new(n: usize, default: T) -> Self {
        let matrix = vec![vec![default.clone(); n]; n];
        Self { matrix, default }
    }
}

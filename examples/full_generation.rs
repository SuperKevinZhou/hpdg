use hpdg::graph::Graph;
use hpdg::io::IO;
use hpdg::string::{SentenceConfig, StringGen};
use hpdg::vector::{IntRange, Vector};

fn main() {
    let mut io = IO::new("example".to_string());

    // Graph sample
    let mut g = Graph::new(5, false);
    g.add_edge(1, 2, None);
    g.add_edge(2, 3, None);

    // Random vector sample
    let vecs = Vector::random_int(3, &[IntRange::Max(10), IntRange::MinMax(5, 8)]);

    // Random sentence sample
    let mut cfg = SentenceConfig::default();
    cfg.sentence_terminators = ".".to_string();
    let sentence = StringGen::random_sentence(6, Some(&cfg));

    io.input_writeln("5 2");
    io.input_writeln(g.to_string());
    io.input_writeln(vecs.len());
    for v in vecs {
        io.input_writeln(v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
    io.input_writeln(sentence);

    let _ = io.flush_to_disk();
}

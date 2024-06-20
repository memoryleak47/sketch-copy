use egg::*;

fn get_rules(rules: &[&'static str]) -> Vec<Rewrite<SymbolLang, ()>> {
    let mut rewrites = Vec::new();

    for r in rules {
        let rewrite = match *r {
            "transpose-maps" => rew("(m ?n1 (m ?n2 ?f))", "(o T (o (m ?n2 (m ?n1 ?f)) T))"),
            "split-map" => rew("(m (* ?n1 ?n2) ?f)", "(o j (o (m ?n1 (m ?n2 ?f)) (s ?n2)))"),

            "map-fission" => rew("(m ?n (o ?f ?g))", "(o (m ?n ?f) (m ?n ?g))"),
            "map-fusion" => rew("(o (m ?n ?f) (m ?n ?g))", "(m ?n (o ?f ?g))"),

            "assoc1" => rew("(o ?a (o ?b ?c))", "(o (o ?a ?b) ?c)"),
            "assoc2" => rew("(o (o ?a ?b) ?c)", "(o ?a (o ?b ?c))"),
            x => panic!("unknown rule: {x}"),
        };
        rewrites.push(rewrite);
    }

    rewrites
}

fn rew(s1: &str, s2: &str) -> Rewrite<SymbolLang, ()> {
    let p = |x: &str| x.parse::<Pattern<SymbolLang>>().unwrap();
    Rewrite::new("", p(s1), p(s2)).unwrap()
}

static RULES: &[&'static str] = &["assoc1", "assoc2", "map-fission", "transpose-maps", "split-map"];
fn main() {
    let mid = "(o (m (* n1 32) (o (m (* n2 32) j) j)) (o (o j (o (m n1 (m 32 (m n2 (m 32 (m n3 (m 32 f)))))) (s 32))) (m (* n1 32) (o (m n2 (m 32 (s 32))) (s 32)))))".parse().unwrap();
    let goal = "(o (o (m (* n1 32) (o (m (* n2 32) j) j)) j) (o (o (m n1 (o T (m n2 (o (m 32 T) T)))) (o (m n1 (m n2 (m n3 (m 32 (m 32 (m 32 f)))))) (m n1 (m n2 T)))) (o (o (m n1 (o (m n2 (m 32 T)) T)) (s 32)) (m (* n1 32) (o (m n2 (m 32 (s 32))) (s 32))))))".parse().unwrap();

    let rewrites = get_rules(&RULES);
    let r = Runner::<SymbolLang, ()>::default().with_node_limit(10_000_000).with_expr(&mid).run(&rewrites);
    dbg!(r.stop_reason);
    let mut eg = r.egraph;
    let i1 = eg.add_expr(&mid);
    let i2 = eg.add_expr(&goal);
    dbg!(eg.find(i1), eg.find(i2));
}

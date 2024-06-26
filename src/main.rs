use egg::*;

fn get_rules(rules: &[&'static str]) -> Vec<Rewrite<SymbolLang, ()>> {
    let mut rewrites = Vec::new();

    for r in rules {
        let rewrite = match *r {
            "transpose-maps" => rew(r, "(m ?n1 (m ?n2 ?f))", "(o T (o (m ?n2 (m ?n1 ?f)) T))"),
            "split-map" => rew(r, "(m (* ?n1 ?n2) ?f)", "(o j (o (m ?n1 (m ?n2 ?f)) (s ?n2)))"),

            "map-fission" => rew(r, "(m ?n (o ?f ?g))", "(o (m ?n ?f) (m ?n ?g))"),
            "map-fusion" => rew(r, "(o (m ?n ?f) (m ?n ?g))", "(m ?n (o ?f ?g))"),

            "assoc1" => rew(r, "(o ?a (o ?b ?c))", "(o (o ?a ?b) ?c)"),
            "assoc2" => rew(r, "(o (o ?a ?b) ?c)", "(o ?a (o ?b ?c))"),
            x => panic!("unknown rule: {x}"),
        };
        rewrites.push(rewrite);
    }

    rewrites
}

fn rew(name: &str, s1: &str, s2: &str) -> Rewrite<SymbolLang, ()> {
    let p = |x: &str| x.parse::<Pattern<SymbolLang>>().unwrap();
    Rewrite::new(name, p(s1), p(s2)).unwrap()
}

static RULES: &[&'static str] = &["assoc1", "assoc2", "map-fission", "transpose-maps"];
fn main() {
    let mid: RecExpr<SymbolLang> = "(o (m (* n1 32) (o (m (* n2 32) j) j)) (o (o j (o (m n1 (m 32 (m n2 (m 32 (m n3 (m 32 f)))))) (s 32))) (m (* n1 32) (o (m n2 (m 32 (s 32))) (s 32)))))".parse().unwrap();
    let goal: RecExpr<SymbolLang> = "(o (o (m (* n1 32) (o (m (* n2 32) j) j)) j) (o (o (m n1 (o T (m n2 (o (m 32 T) T)))) (o (m n1 (m n2 (m n3 (m 32 (m 32 (m 32 f)))))) (m n1 (m n2 T)))) (o (o (m n1 (o (m n2 (m 32 T)) T)) (s 32)) (m (* n1 32) (o (m n2 (m 32 (s 32))) (s 32))))))".parse().unwrap();

    let mid2 = mid.clone();
    let goal2 = goal.clone();

    let rewrites = get_rules(&RULES);
    let r = Runner::<SymbolLang, ()>::default()
             .with_iter_limit(100)
             .with_node_limit(10_000_000)
             .with_time_limit(std::time::Duration::from_secs(40))
             .with_expr(&mid)
             .with_hook(move |r| {
                let lkp = |a| r.egraph.lookup_expr(a).map(|x| r.egraph.find(x));
                if lkp(&mid2) == lkp(&goal2) {
                    return Err(String::from("success!"));
                } else { Ok(()) }
            }).run(&rewrites);
    r.print_report();
    let eg = r.egraph;
    dbg!(eg.lookup_expr(&mid));
    dbg!(eg.lookup_expr(&goal));
}

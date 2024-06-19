use egg::*;

fn main() {
    let mut rewrites: Vec<Rewrite<SymbolLang, ()>> = Vec::new();
    rewrites.push(rewrite!("assoc1"; "(o (o ?f ?g) ?h)" => "(o ?f (o ?g ?h))"));
    rewrites.push(rewrite!("assoc2"; "(o ?f (o ?g ?h))" => "(o (o ?f ?g) ?h)"));
    rewrites.push(rewrite!("map-fission"; "(m ?n (o ?f ?g))" => "(o (m ?n ?f) (m ?n ?g))"));
    rewrites.push(rewrite!("transpose-maps"; "(m ?n1 (m ?n2 ?f))" => "(o T (o (m ?n2 (m ?n1 ?f)) T))"));

    let start: RecExpr<SymbolLang> = "(m (* n1 32) (m (* n2 32) f))".parse().unwrap();
    let mid: RecExpr<SymbolLang> = "(o (o (o (m (* n1 32) j) j) (o (m n1 (m 32 (m n2 (m 32 f)))) (m n1 (m 32 (s 32))))) (s 32))".parse().unwrap();

    let r = Runner::<SymbolLang, ()>::default().with_node_limit(1_000_000).with_expr(&start).run(&rewrites);
    dbg!(r.stop_reason);
    let mut eg = r.egraph;
    let i1 = eg.add_expr(&start);
    let i2 = eg.add_expr(&mid);
    dbg!(eg.find(i1), eg.find(i2));
}

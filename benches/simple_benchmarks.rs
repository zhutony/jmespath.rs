#![feature(test)]
#[cfg(test)]

extern crate jmespath;
extern crate test;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use test::Bencher;

#[bench]
fn bench_parsing_foo_bar_baz(b: &mut Bencher) {
    b.iter(|| jmespath::Expression::new("foo.bar.baz"));
}

#[bench]
fn bench_lexing_foo_bar_baz(b: &mut Bencher) {
    b.iter(|| for _ in jmespath::tokenize("foo.bar.baz") {});
}

#[bench]
fn bench_full_foo_bar_baz(b: &mut Bencher) {
    let json = Json::from_str("{\"foo\":{\"bar\":{\"baz\":true}}}").unwrap();
    b.iter(|| {
        let expr = jmespath::Expression::new("foo.bar.baz").unwrap();
        expr.search(json.clone()).unwrap();
    });
}

#[bench]
fn bench_full_or_branches(b: &mut Bencher) {
    let json = Json::from_str("{\"foo\":true}").unwrap();
    b.iter(|| {
        let expr = jmespath::Expression::new("bar || baz || bam || foo").unwrap();
        expr.search(json.clone()).unwrap();
    });
}

#[bench]
fn bench_expr_identifier(b: &mut Bencher) {
    let expr = "abcdefghijklmnopqrstuvwxyz";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_expr_subexpr(b: &mut Bencher) {
    let expr = "abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_deep_projection_104(b: &mut Bencher) {
    let deep = "a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*]";
    b.iter(|| jmespath::parse(deep));
}

#[bench]
fn bench_parse_identifier(b: &mut Bencher) {
    let expr = "abcdefghijklmnopqrstuvwxyz";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_parse_subexpr(b: &mut Bencher) {
    let expr = "abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_parse_nested50(b: &mut Bencher) {
    let expr = "j49.j48.j47.j46.j45.j44.j43.j42.j41.j40.j39.j38.j37.j36.j35.j34.j33.j32.j31.j30.j29.j28.j27.j26.j25.j24.j23.j22.j21.j20.j19.j18.j17.j16.j15.j14.j13.j12.j11.j10.j9.j8.j7.j6.j5.j4.j3.j2.j1.j0";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_parse_nested50_pipe(b: &mut Bencher) {
    let expr = "j49|j48|j47|j46|j45|j44|j43|j42|j41|j40|j39|j38|j37|j36|j35|j34|j33|j32|j31|j30|j29|j28|j27|j26|j25|j24|j23|j22|j21|j20|j19|j18|j17|j16|j15|j14|j13|j12|j11|j10|j9|j8|j7|j6|j5|j4|j3|j2|j1|j0";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_parse_nested50_index(b: &mut Bencher) {
    let expr = "[49][48][47][46][45][44][43][42][41][40][39][38][37][36][35][34][33][32][31][30][29][28][27][26][25][24][23][22][21][20][19][18][17][16][15][14][13][12][11][10][9][8][7][6][5][4][3][2][1][0]";
    b.iter(|| jmespath::parse(expr));
}

#[bench]
fn bench_parse_raw_string_literal(b: &mut Bencher) {
    let expr = "'abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz'";
    b.iter(|| jmespath::parse(expr));
}

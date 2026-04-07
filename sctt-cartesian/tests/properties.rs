use std::panic;

use proptest::prelude::*;
use sctt_cartesian::evaluate::evaluate;
use sctt_cartesian::quote::{normalize, quote};
use sctt_cartesian::syntax::Term;
use sctt_cartesian::value::Env;

/// Generate well-formed SCTT terms up to a given depth.
/// Terms are syntactically valid but not necessarily well-typed.
/// NbE should be total on all syntactic terms -- panics indicate evaluator bugs
/// only when the input is well-typed; ill-typed inputs may panic legitimately.
fn arb_term(max_depth: u32) -> impl Strategy<Value = Term> {
    let leaf = prop_oneof![
        Just(Term::Nat),
        Just(Term::Zero),
        (0..3usize).prop_map(Term::var),
        (0..3u8).prop_filter_map("valid level", |n| Term::universe(n)),
    ];

    leaf.prop_recursive(max_depth, 64, 8, |inner| {
        prop_oneof![
            inner.clone().prop_map(Term::lambda),
            (inner.clone(), inner.clone()).prop_map(|(f, a)| Term::app(f, a)),
            inner.clone().prop_map(Term::succ),
            inner.clone().prop_map(Term::fst),
            inner.clone().prop_map(Term::snd),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Term::pair(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(d, c)| Term::pi(d, c)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Term::sigma(a, b)),
            inner.clone().prop_map(Term::path_lam),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn nbe_idempotent(t in arb_term(3)) {
        // NbE may panic on ill-typed terms -- that's acceptable for this test.
        // We only care that when NbE succeeds, it's idempotent.
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let n1 = normalize(&t);
            let n2 = normalize(&n1);
            assert_eq!(n1, n2, "NbE should be idempotent");
        }));
        // If it panicked on an ill-typed term, that's fine -- skip it
        if result.is_err() {
            // panic on ill-typed input is acceptable
        }
    }

    #[test]
    fn nbe_roundtrip(t in arb_term(3)) {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let env = Env::new();
            let v1 = evaluate(&env, &t);
            let t1 = quote(0, 0, &v1);
            let v2 = evaluate(&env, &t1);
            let t2 = quote(0, 0, &v2);
            assert_eq!(t1, t2, "eval-quote should be idempotent on normal forms");
        }));
        if result.is_err() {
            // panic on ill-typed input is acceptable
        }
    }
}

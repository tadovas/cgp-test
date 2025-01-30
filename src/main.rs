use cgp_test::{CanExecuteTx, Rollup, RollupSpec};

fn main() {
    println!("Hello, world!");

    generic_do_it(
        &Rollup::<TestSpec>::new("param with spec1"),
        "doer param".to_string(),
    );

    generic_do_it(
        &Rollup::<ProdSpec>::new("param with spec2"),
        "doer param".to_string(),
    );
}

fn generic_do_it<T: CanExecuteTx>(doer: &T, param: String) {
    let res = doer.do_it(param);
    println!("{res}");
}

struct TestSpec;

impl RollupSpec for TestSpec {
    fn specific() -> String {
        "test spec".to_owned()
    }
}

struct ProdSpec;

impl RollupSpec for ProdSpec {
    fn specific() -> String {
        "prod spec".to_owned()
    }
}

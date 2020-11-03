use modding::client::Client;
use std::{sync::Arc, time::Instant};
use rhai::{Engine, EvalAltResult, Scope};
use rhai::RegisterFn;

#[derive(Clone)]
struct Test {
    pub test: u32
}

impl Test {
    fn test(&mut self) -> u32 {
        self.test.clone()
    }
}

fn main() -> Result<(), Box<EvalAltResult>> {
    let client = Client::load("mods");
    let start = Instant::now();
    // println!("{:#?} | elapsed {:?}", client, start.elapsed());
    println!("{} | elapsed {:?}", client.get_translation("test/en", "ready_for_today"), start.elapsed());
    // println!("{} | elapsed {:?}", client.base.language_default().get("ready_for_today"), start.elapsed());

    let mut engine = Engine::new();
    engine.register_type::<Test>().register_get("test", Test::test).register_custom_operator("altagos", 255)?;
    engine.register_fn("altagos", |x: Test, y: &str| {println!("{}x{}", x.test, y);});

    let result = engine.eval::<i64>("40 + 2")?;
    //                      ^^^^^^^ cast the result to an 'i64', this is required

    println!("Answer: {}", result);             // prints 42

    let ast = engine.compile(
        r#"
            // a function with two parameters: string and i64
            fn hello(x, y) {
                x.len + y
            }
    
            // functions can be overloaded: this one takes only one parameter
            fn hello(x) {
                x * 2
            }
    
            // this one takes no parameters
            fn hello() {
                42
            }
    
            // this one is private and cannot be called by 'call_fn'
            private fn hidden() {
                throw "you shouldn't see me!";
            }
        "#)?;
    let mut scope = Scope::new();

    scope.push("test", Test {test: 12});

    engine.eval_with_scope::<()>(&mut scope, r"
        let x = test.test;
    ")?;

    // Second invocation using the same state
    let ast = engine.compile_file("mods/base/commands/test.rhai".into())?;
    let result: () = engine.call_fn(&mut scope, &ast, "test", ())?;

    // println!("result: {}", result);  

    Ok(())
}
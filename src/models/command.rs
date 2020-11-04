use rhai::{Engine, Scope};
use serde::{Deserialize, Serialize};
use serenity::{client::Context, model::channel::Message};
use crate::{error::CommandResult, traits::ModuleComponent, util::deserialize_file};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Command {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub description: String,
    pub command: String,
}

impl Command {
    pub fn run(&self, ctx: Context, msg: Message) -> CommandResult<(), Box<dyn std::error::Error + '_>> {
        tracing::debug!("Running command: `{}`", self.name);
        let mut engine = Engine::new();
        let mut scope = Scope::new();

        engine.on_print(|x| println!("hello: {}", x));
        engine.on_debug(|x| println!("DEBUG: {}", x));

        let ast = engine.compile(self.command.as_str()).unwrap();
        let _result: () = engine.call_fn(&mut scope, &ast, self.name.as_str(), (ctx, msg)).unwrap();

        tracing::debug!("Running command `{}` was successfull", self.name);
        CommandResult::Success
    }
}

impl ModuleComponent for Command {
    fn load(path: &str) -> Command {
        deserialize_file::<Command>(path).expect(format!("Invalid command file: `{}`", path).as_str())
    }
}

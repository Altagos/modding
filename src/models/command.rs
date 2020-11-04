use rhai::{Engine, Scope};
use serde::{Deserialize, Serialize};
use serenity::{client::Context as SerenityCtx, model::channel::Message as SerenityMsg};
use crate::{error::CommandResult, traits::ModuleComponent, util::deserialize_file};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Command {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub description: String,
    pub command: String,
}

#[derive(Clone)]
struct Message {
    msg: SerenityMsg,
}

impl Message {
    pub fn id(&mut self) -> u64 {
        self.msg.id.0.clone()
    }

    pub fn author_id(&mut self) -> u64 {
        self.msg.author.id.0.clone()
    }

    pub fn content(&mut self) -> String {
        self.msg.content.clone()
    }
}

impl Command {
    pub fn run(&self, ctx: SerenityCtx, msg: SerenityMsg) -> CommandResult<(), Box<dyn std::error::Error + '_>> {
        tracing::debug!("Running command: `{}`", self.name);
        let mut engine = Engine::new();
        engine.register_type::<SerenityCtx>();
        engine.register_type::<Message>();
        engine.register_get("id", Message::id);
        engine.register_get("author_id", Message::author_id);
        engine.register_get("content", Message::content);

        let mut scope = Scope::new();

        engine.on_print(|x| println!("hello: {}", x));
        engine.on_debug(|x| println!("DEBUG: {}", x));

        let ast = engine.compile(self.command.as_str()).unwrap();
        let _result: () = engine.call_fn(&mut scope, &ast, self.name.as_str(), (ctx, Message { msg })).unwrap();

        tracing::debug!("Running command `{}` was successfull", self.name);
        CommandResult::Success
    }
}

impl ModuleComponent for Command {
    fn load(path: &str) -> Command {
        deserialize_file::<Command>(path).expect(format!("Invalid command file: `{}`", path).as_str())
    }
}

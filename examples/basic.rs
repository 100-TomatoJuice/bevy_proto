use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_proto::{HandlePath, ProtoCommands, ProtoComponent, ProtoData, ProtoPlugin, Prototypical};

/// This is the component we will use with our prototype
/// It must derive both Serialize and Deserialize from serde in order to compile
#[derive(Serialize, Deserialize)]
struct Person {
	pub name: String,
}

/// This is where we implement the [`ProtoComponent`] trait.
///
/// Note that we must apply the `#[typetag::serde]` attribute
#[typetag::serde]
impl ProtoComponent for Person {
	fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
		/// Here, we create the component we're going to insert.
		/// This can really be any valid Bevy component type, but we'll
		/// use `Person` since it's so simple
		let component = Self {
			name: self.name.clone(),
		};

		// Attach the component(s) to the entity
		commands.insert(component);
	}
}

/// For simple types, deriving [`ProtoComponent`] can be used to automatically
/// generate the required `impl` block.
///
/// The [`Person`] component defined above could have simply been written as:
/// ```
/// #[derive(Serialize, Deserialize, ProtoComponent)]
/// struct Person {
/// 	// Optional: #[proto_comp(Clone)]
/// 	pub name: String,
/// }
/// ```
///
/// Here, we call the attribute with the "Copy" argument as this struct can
/// readily derive Copy and should be marginally faster than Clone
#[derive(Serialize, Deserialize, ProtoComponent)]
struct Ordered {
	#[proto_comp(Copy)]
	pub order: i32,
}

/// Spawn in the person.
///
/// This system also demonstrates the minimum requirements for using the prototype system
fn spawn_person(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
	/// Here, we attempt to get our prototype by name.
	/// We'll raise an exception if it's not found, just so we can fail fast.
	/// In reality, you'll likely want to handle this prototype not existing.
	let proto = data.get_prototype("Person Test 1").expect("Should exist!");

	// Spawn in the prototype!
	proto.spawn(&mut commands, &data, &asset_server);

	// Spawn it again!
	proto.spawn(&mut commands, &data, &asset_server);

	// Spawn in others!
	for i in 1..4 {
		data.get_prototype(format!("Person Test {}", i).as_str())
			.unwrap()
			.spawn(&mut commands, &data, &asset_server);
	}
}

/// A system to test our spawner. This makes each entity introduce itself when spawned in
fn introduce(query: Query<(&Person, &Ordered), Added<Person>>) {
	for (person, ordered) in query.iter() {
		println!("{}. Hello! My name is {}", ordered.order, person.name);
	}
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		// This plugin should come AFTER any others that it might rely on
		// In this case, we need access to what's added by [`DefaultPlugins`]
		// so we place this line after that one
		.add_plugin(ProtoPlugin::default())
		// Add our spawner system (this one only runs once at startup)
		.add_startup_system(spawn_person.system())
		.add_system(introduce.system())
		.run();
}
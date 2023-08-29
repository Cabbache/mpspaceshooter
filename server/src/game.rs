use std::collections::HashMap;
use std::error::Error;

use futures::future::join_all;
use serde_json::{from_str, json, to_string, Value};
use warp::ws::Message;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use uuid::Uuid;

use crate::Clients;
use crate::Client;
use crate::WorldLoot;
use crate::handler::spawn_from_prev;

use utils::server_gameobjects::*;
use utils::shared_gameobjects::*;
use utils::trajectory::*;

const LOOT_RADIUS: f32 = 60.0; //players must be within this distance to claim

impl Client {
	pub async fn transmit(&self, msg: &ServerMessage, public_id: Option<String>) -> Result<(), Box<dyn Error>> {
		if let Some(ch) = self.sender.as_ref() {
			let public_id = match public_id {
				Some(id) => id, 
				None => self.state.read().await.id.clone(),
			};
			let serialized_msg = match msg {
				ServerMessage::PlayerJoin(_) |
				ServerMessage::GameState{pstates: _, worldloot: _} => {
				//ServerMessage::LootCollected{loot_id: _, collector: _} => {
					match msg {
						ServerMessage::GameState{ pstates, worldloot } => {
							let encoded_states: Vec<Value> = pstates.iter().map(|state| {
								state.encode(public_id == state.id)
							}).collect();
							to_string(&json!({
								"t": "GameState",
								"c": {
									"players": encoded_states,
									"loot": worldloot,
								}
							}))?
						},
						ServerMessage::PlayerJoin(pstate) => {
							to_string(&json!({
								"t": "PlayerJoin",
								"c": pstate.encode(pstate.id == public_id),
							}))?
						},
						_ => String::new()
					}
				},  
				_ => to_string(msg)?
			};
			ch.send(Ok(Message::text(serialized_msg))).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
		}
		Ok(())
	}
}

pub async fn handle_game_message(public_id: String, message: &str, clients: &Clients, world_loot: &WorldLoot) -> Result<(), Box<dyn Error>>{
	let message: ClientMessage = match from_str(message) {
		Ok(v) => v,
		Err(m) => {
			eprintln!("Can't deserialize message: {}", m);
			return Ok(());
		}
	};

	let time_now = current_time();

	let clr = clients.read().await;
	let sender_state = match clr.get(&public_id) {
		Some(v) => &v.state,
		None => {
			eprintln!("Can't find sender in clients: {}", public_id);
			return Ok(());
		}
	};

	let mut state = sender_state.read().await.clone();
	if state.trajectory.time < time_now - MAX_TIME_BEFORE {
		let mut writer = sender_state.write().await;
		writer.trajectory.advance_to_min_time(time_now);
		state = writer.clone();
	}
	let is_allowed = match message {
		ClientMessage::StateQuery => true,
		ClientMessage::Ping => true,
		ClientMessage::Spawn => state.trajectory.health == 0u8, //You have to be dead to call spawn
		_ => state.trajectory.health > 0u8, //You have to be alive to call the rest
	};

	if !is_allowed {
		eprintln!("Rejected message {:?}", message);
		return Ok(());
	}

	match message {
		ClientMessage::Ping => {
			if let Some(client) = clr.get(&public_id) {
				client.transmit(
					&ServerMessage::Pong(time_now),
					Some(public_id.to_string())
				).await?;
			} else {
					eprintln!("Can't find client");
			}
		},
		ClientMessage::AckPong => {
			
		},
		ClientMessage::Spawn => {
			spawn_from_prev(&mut *sender_state.write().await); //reset health and position
			broadcast(
				&ServerMessage::PlayerJoin(sender_state.read().await.clone()),
				&clr
			).await; //broadcast playerjoin
		},
		ClientMessage::TrajectoryUpdate {change, time, at} => {
			let cost = get_cost(change.clone().utype);
			if state.cash < cost {
				eprintln!("{} attempted to buy something without enough cash", public_id);
				return Ok(());
			}
			let successful: bool;
			let updated_trajectory = {
				let mut writeable = sender_state.write().await;
				successful = writeable.trajectory.update(change.clone(), at.clone(), time, time_now);
				if successful {
					writeable.cash -= cost;
				} else {
					writeable.trajectory.advance(time_now);
				}
				writeable.trajectory.clone()
			};
			if successful { //if accepted by the server, broadcast change
				broadcast(
					&ServerMessage::TrajectoryUpdate{
						change: change,
						time: time,
						at: at,
						from: public_id.clone(),
					},
					&clr,
				).await;
			} else { //if rejected, correct the client
				println!("Correcting");
				if let Some(client) = clr.get(&public_id) {
					client.transmit(
						&ServerMessage::Correct{id: public_id.clone(), tr: updated_trajectory.to_b64()},
						Some(public_id.clone())
					).await?;
				} else {
					eprintln!("Can't find client");
				}
			}
		},
		ClientMessage::StateQuery => { //TODO rate limit this
			eprintln!("Got statequery");

			//gpt-4 did this
			// Use futures::future::join_all to wait for all tasks to complete.
			let players_futures: Vec<_> = clr.iter()
				.map(|(_, value)| value.state.write())
				.collect();

			let players: Vec<_> = join_all(players_futures).await
				.into_iter()
				.filter_map(|mut lock| {
					lock.trajectory.advance_to_min_time(time_now);
					//lock.trajectory.advance_to_time(time_now);
					let clone = lock.clone();
					match clone.trajectory.health > 0u8 { //to only send the living ones
						true => Some(clone),
						false => None
					}
				})
				.collect();

			//Send the response to the client.
			if let Some(client) = clr.get(&public_id) {
				client.transmit(
					&ServerMessage::GameState{
						pstates: players,
						worldloot: world_loot.read().await.clone(),
					},
					Some(public_id)
				).await?;
			} else {
					eprintln!("Can't find client")
			}
		},
		ClientMessage::ChangeSlot { slot } => { //TODO consider also changing trigger_pressed to false both on client and server when changeslot
			if sender_state.read().await.clone().inventory.selection != slot
				{ sender_state.write().await.inventory.selection = slot; }
		},
		ClientMessage::Shoot(mut shoot_info) => {
			println!("{:?}", shoot_info);
			let weapon_selected = state.inventory.weapons.get(
				&state.inventory.selection
			).unwrap().clone();

			//check if there is even ammo
			if weapon_selected.ammo <= 0 {
				return Ok(());
			}

			{
				let mut writeable = sender_state.write().await;
				writeable.inventory.weapons.get_mut(
						&state.inventory.selection
				).unwrap().ammo -= 1;
				state = writeable.clone();
			}

			shoot_info.shooter = Some(public_id);

			shoot_info.victim = match shoot_info.victim.clone() {
				None => None,
				Some(mut victim) => match clr.get(&victim.id) {
					None => None, //Malicious behavior
					Some(player) => {
						if player.state.read().await.trajectory.health == 0 {
							None
						} else {
							let victim_state = {
								let mut victim_writer = player.state.write().await;
								victim_writer.trajectory.apply_change(
									UpdateTypeWrapper {
										utype: UpdateType::Bullet,
										value: Some(25u8),
									}
								);
								if victim_writer.trajectory.health == 0 {
									victim_writer.trajectory.advance(time_now + 5000); //so that loot is dropped there
								}
								victim_writer.clone()
							};

							if victim_state.trajectory.health == 0 {
								let mut rng = StdRng::from_entropy();
								let dropped_loot = LootObject{
									x: victim_state.trajectory.pos.x,
									y: victim_state.trajectory.pos.y,
									loot: match rng.gen_range(0..101){
										0..=25 => LootContent::Cash(victim_state.cash / 2),
										26..=50 => LootContent::PistolAmmo(15),
										51..=75 => LootContent::Health(30),
										_ => LootContent::SpeedBoost,
									}
								};
								let dropped_loot_uuid = Uuid::new_v4().as_simple().to_string();
								world_loot.write().await.insert(
									dropped_loot_uuid.clone(),
									dropped_loot.clone(),
								);
								victim.loot = Some(
									LootDrop{
										object: dropped_loot,
										uuid: dropped_loot_uuid,
									}
								);
							}
							Some(victim)
						}
					}
				}
			};

			broadcast(&ServerMessage::Shoot(shoot_info), &clr).await;
		},
		ClientMessage::ClaimLoot { loot_id } => {
			let loot_thing = {
				let loot_lock = world_loot.read().await;
				loot_lock.get(&loot_id).cloned()
			};
			match loot_thing {
				Some(loot_obj) => {
					let pp = {
						let mut writable = sender_state.write().await;
						writable.trajectory.advance(time_now); //this may be problematic
						writable.trajectory.pos.clone()
					};
					if (pp.y - loot_obj.y).powi(2) + (pp.x - loot_obj.x).powi(2) > LOOT_RADIUS.powi(2){
						if let Some(client) = clr.get(&public_id){
							client.transmit(&ServerMessage::LootReject(loot_id), Some(public_id)).await?;
						} else {
							eprintln!("Weird, did not find client {} in clr", public_id);
						}
						return Err("Too far for loot claim".into());
					}

					{
						//make sure to acquire both locks before proceeding
						let mut pstate_writer = sender_state.write().await;
						let mut world_loot_writer = world_loot.write().await;

						world_loot_writer.remove(&loot_id);
						match loot_obj.loot {
							LootContent::Cash(amount) => pstate_writer.cash += amount,
							LootContent::PistolAmmo(amount) => {
								let selection = pstate_writer.inventory.selection;
								if let Some(weapon) = pstate_writer.inventory.weapons.get_mut(&selection) {
									weapon.ammo += amount;
								}
							},
							LootContent::Health(health) => {
								pstate_writer.trajectory.apply_change(UpdateTypeWrapper {
									utype: UpdateType::Health,
									value: Some(health),
								});
							}
							LootContent::SpeedBoost => {
							}
						}
					}//locks are released
					println!("locks released");

					broadcast(
						&ServerMessage::LootCollected { loot_id: loot_id, collector: public_id },
						&clr
					).await; //broadcast that loot was collected
				},
				None => {
					return Err(format!("Can't find requested lootobject: {}", loot_id).into());
				}
			};
		},
		ClientMessage::Correct(id) => {
			if let Some(other) = clr.get(&id){
				let correction = other.state.read().await.trajectory.to_b64();
				if let Some(sender) = clr.get(&public_id){
					sender.transmit(
						&ServerMessage::Correct{id: id, tr: correction},
						Some(public_id)
					).await?;
				}
			} else {
				eprintln!("Client requested correction for non existing player");
			}
		}
	}
	Ok(())
}

pub async fn broadcast(msg: &ServerMessage, clients_readlock: &tokio::sync::RwLockReadGuard<'_, HashMap<std::string::String, Client>>){
	for (public_id, client) in clients_readlock.iter(){
		if let Err(e) = client.transmit(msg, Some(public_id.to_string())).await {
			eprintln!("Error transmitting message: {}", e);
		}
	}
}

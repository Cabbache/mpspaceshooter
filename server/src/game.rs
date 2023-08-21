use std::collections::HashMap;
use std::error::Error;

use futures::future::join_all;
use serde_json::{from_str, json, to_string, Value};
use warp::ws::Message;
use xxhash_rust::xxh3::xxh3_64;
use uuid::Uuid;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::Clients;
use crate::Client;
use crate::WorldLoot;
use crate::handler::spawn_from_prev;

use utils::gameobjects::*;
use utils::trajectory::*;

const LOOT_RADIUS: f32 = 60.0; //players must be within this distance to claim

impl Client {
	pub async fn transmit(&self, msg: &ServerMessage, public_id: Option<String>) -> Result<(), Box<dyn Error>> {
		if let Some(ch) = self.sender.as_ref() {
			let public_id = match public_id {
				Some(id) => id, 
				None => self.state.read().await.public_id.clone(),
			};
			let serialized_msg = match msg {
				ServerMessage::PlayerJoin(_) |
				ServerMessage::GameState{pstates: _, worldloot: _, bodies: _} => {
				//ServerMessage::LootCollected{loot_id: _, collector: _} => {
					match msg {
						ServerMessage::GameState{ pstates, worldloot, bodies } => {
							let encoded_states: Vec<Value> = pstates.iter().map(|state| {
								state.encode(public_id == state.public_id)
							}).collect();
							to_string(&json!({
								"t": "GameState",
								"c": {
									"players": encoded_states,
									"loot": worldloot,
									"bodies": bodies,
								}
							}))?
						},
						ServerMessage::PlayerJoin(pstate) => {
							to_string(&json!({
								"t": "PlayerJoin",
								"c": pstate.encode(pstate.public_id == public_id),
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

pub async fn handle_game_message(private_id: &str, message: &str, clients: &Clients, world_loot: &WorldLoot) -> Result<(), Box<dyn Error>>{
	let message: ClientMessage = match from_str(message) {
		Ok(v) => v,
		Err(m) => {
			eprintln!("Can't deserialize message: {}", m);
			return Ok(());
		}
	};

	println!("{}", current_time());

	let clr = clients.read().await;
	let sender_state = match clr.get(private_id) {
		Some(v) => &v.state,
		None => {
			eprintln!("Can't find sender in clients: {}", private_id);
			return Ok(());
		}
	};

	let health = sender_state.read().await.clone().health;
	let is_allowed = match message {
		ClientMessage::StateQuery => true, //dead or alive, this is allowed
		ClientMessage::Spawn => health <= 0f32, //You have to be dead to call spawn
		_ => health > 0f32, //You have to be alive to call the rest
	};

	if !is_allowed {
		eprintln!("modded client detected (action doesn't match vital status)");
		return Ok(());
	}

	match message {
		ClientMessage::Ping => {
			if let Some(client) = clr.get(private_id) {
				let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
				client.transmit(
					&ServerMessage::Pong(current_time()),
					Some(public_id)
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
			let successful: bool;
			let updated_trajectory = {
				let mut writeable = sender_state.write().await;
				successful = writeable.trajectory.update(change.clone(), at.clone(), time, current_time());
				writeable.trajectory.clone()
			};
			if successful { //if accepted by the server, broadcast change
				broadcast(
					&ServerMessage::TrajectoryUpdate{
						change: change,
						time: time,
						at: at,
						from: format!("{:x}", xxh3_64(private_id.as_bytes())),
					},
					&clr,
				).await;
			} else { //if rejected, correct the client
				if let Some(client) = clr.get(private_id) {
					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					client.transmit(
						&ServerMessage::Correct{id: public_id.clone(), tr: updated_trajectory.to_b64()},
						Some(public_id)
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
					lock.trajectory.advance(current_time());
					let clone = lock.clone();
					match clone.health > 0.0 { //to only send the living ones
						true => Some(clone),
						false => None
					}
				})
				.collect();

			//Send the response to the client.
			if let Some(client) = clr.get(private_id) {
				let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
				client.transmit(
					&ServerMessage::GameState{
						pstates: players,
						worldloot: world_loot.read().await.clone(),
						bodies: utils::trajectory::BODIES.to_vec(),
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
		ClientMessage::TrigUpdate { pressed } => {
			let cl = sender_state.read().await.clone();

			//This would require a modded client
			if cl.trigger_pressed == pressed {
				eprintln!("trigger press identical");
				return Ok(());
			}

			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			sender_state.write().await.trigger_pressed = pressed;
			let weapon_selected = cl.inventory.weapons.get(
				&cl.inventory.selection
			).unwrap().clone();

			//check if there is even ammo
			if weapon_selected.ammo <= 0 {
				return Ok(());
			}

			//check if ammo needs to be decremented
			match (weapon_selected.weptype.clone(), pressed) {
				(WeaponType::Pistol, false) => {},
				(WeaponType::Grenade {press_time: _}, true) => {},
				_ => {
					let mut writeable = sender_state.write().await;
					writeable.inventory.weapons.get_mut(
							&cl.inventory.selection
					).unwrap().ammo -= 1;
				}
			};

			//check if client needs to know
			match (weapon_selected.weptype.clone(), pressed) {
				(WeaponType::Pistol, false) => {},
				_ => {
					broadcast(
						&ServerMessage::TrigUpdate {
							by: public_id,
							weptype: weapon_selected.weptype.clone(),
							pressed: pressed,
						},
						&clr
					).await;				
				}
			};

			//Update healths
			match weapon_selected.weptype {
				//TODO since we don't care if pistol released, we should make the client not even send it
				WeaponType::Pistol => {
					if !pressed {
						return Ok(());
					}

					let (ss, rr) = {
						let mut writeable = sender_state.write().await;
						writeable.trajectory.advance(current_time());
						(writeable.trajectory.pos.clone(), writeable.trajectory.spin)
					};

					//boring linear search
					for (key, value) in clr.iter() { //TODO try using for_each
						if key == private_id{ //Can't shoot yourself
							continue;
						}

						let playerstate = {
							let mut writable = value.state.write().await;
							writable.trajectory.advance(current_time());
							writable.clone()
						};

						//ignore dead players
						if playerstate.health <= 0f32 {
							continue;
						}
						let pp = playerstate.trajectory.pos;
						let hit = utils::trajectory::line_intersects_circle(ss.x, ss.y, pp.x, pp.y, rr);
						if !hit {
							continue;
						}

						let new_health = {
							let mut writeable = value.state.write().await;
							writeable.health -= 10f32; //hard coded pistol damage to 10
							writeable.health
						};

						if new_health > 0f32 {
							value.transmit(
								&ServerMessage::HealthUpdate(new_health),
								Some(playerstate.public_id)
							).await?; //tell the player that lost the health their new health
						} else {
							let mut rng = StdRng::from_entropy();
							let dropped_loot = LootObject{
								x: pp.x,
								y: pp.y,
								loot: match rng.gen_range(0..101){
									0..=33 => LootContent::Cash(playerstate.cash / 2),
									34..=67 => LootContent::PistolAmmo(15),
									_ => LootContent::SpeedBoost,
								}
							};
							let dropped_loot_uuid = Uuid::new_v4().as_simple().to_string();
							world_loot.write().await.insert(
								dropped_loot_uuid.clone(),
								dropped_loot.clone(),
							);
							broadcast( //tell everyone that the player died and what loot they dropped
								&ServerMessage::PlayerDeath{
									loot: Some(LootDrop{
										object: dropped_loot,
										uuid: dropped_loot_uuid,
									}),
									from: playerstate.public_id,
								},
								&clr
							).await;
						}
						break; //important! you can only hit one player at one time
					};
				},
				WeaponType::Grenade { press_time: _ } => {
					if pressed {
						
					}
				}
			}
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
						writable.trajectory.advance(current_time());
						writable.trajectory.pos.clone()
					};
					if (pp.y - loot_obj.y).powi(2) + (pp.x - loot_obj.x).powi(2) > LOOT_RADIUS.powi(2){
						if let Some(client) = clr.get(private_id){
							client.transmit(&ServerMessage::LootReject(loot_id), Some(format!("{:x}", xxh3_64(private_id.as_bytes())))).await?;
						} else {
							eprintln!("Weird, did not find client {} in clr", private_id);
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
							LootContent::SpeedBoost => {
							}
						}
					}//locks are released
					println!("locks released");

					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					broadcast(
						&ServerMessage::LootCollected { loot_id: loot_id, collector: public_id },
						&clr
					).await; //broadcast that loot was collected
				},
				None => {
					return Err(format!("Can't find requested lootobject: {}", loot_id).into());
				}
			};
		}
	}
	Ok(())
}



pub async fn broadcast(msg: &ServerMessage, clients_readlock: &tokio::sync::RwLockReadGuard<'_, HashMap<std::string::String, Client>>){
	for (private_id, client) in clients_readlock.iter(){
		let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
		if let Err(e) = client.transmit(msg, Some(public_id)).await {
			eprintln!("Error transmitting message: {}", e);
		}
	}
}

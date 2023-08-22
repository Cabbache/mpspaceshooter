import init, { Trajectory, UpdateType, getbody, num_bodies, get_shop_item, num_shop_items } from './pkg/utils.js';
async function runAll(){
	await init();

	// Get the modal
	const joinmodal = document.getElementById("join-modal");
	const form = joinmodal.querySelector('form');
	const submitButton = joinmodal.querySelector('input[type="submit"]');
	submitButton.disabled = true;

	// Add event listeners for the nickname input field and color radio buttons
	const nicknameInput = joinmodal.querySelector('input[name="nick"]');
	const colorRadios = joinmodal.querySelectorAll('input[name="color"]');
	nicknameInput.addEventListener('input', updateSubmitButtonState);

	colorRadios.forEach(radio => {
		radio.addEventListener('change', updateSubmitButtonState);
	});

	function createItemCard(imageSrc, altText, itemName, itemDescription, itemPrice) {
    // Create the main item card div
    const itemCard = document.createElement('div');
    itemCard.className = 'item-card';

    // Create the item image div and its children
    const itemImage = document.createElement('div');
    itemImage.className = 'item-image';

    const img = document.createElement('img');
    img.src = imageSrc;
    img.alt = altText;

    itemImage.appendChild(img);

    // Create the item info div and its children
    const itemInfo = document.createElement('div');
    itemInfo.className = 'item-info';

    const h3 = document.createElement('h3');
    h3.textContent = itemName;

    const p = document.createElement('p');
    p.textContent = itemDescription;

    const itemPriceSpan = document.createElement('span');
    itemPriceSpan.className = 'item-price';
    itemPriceSpan.textContent = itemPrice;

    itemInfo.appendChild(h3);
    itemInfo.appendChild(p);
    itemInfo.appendChild(itemPriceSpan);

    // Create the Buy Now button
    const buyButton = document.createElement('button');
    buyButton.className = 'item-btn';
    buyButton.textContent = 'Buy Now';

    // Append everything to the main item card div
    itemCard.appendChild(itemImage);
    itemCard.appendChild(itemInfo);
    itemCard.appendChild(buyButton);

    return [itemCard, buyButton];
	}

	// Function to update the state of the submit button based on the input values
	function updateSubmitButtonState() {
		const nickname = nicknameInput.value.trim();
		const color = joinmodal.querySelector('input[name="color"]:checked');

		if (nickname.length >= 3 && color !== null && nickname.length <= 24) {
			submitButton.disabled = false;
		} else {
			submitButton.disabled = true;
		}
	}

	form.addEventListener('submit', function(event) {
		event.preventDefault();

		const nick = joinmodal.querySelector('input[name="nick"]').value;
		const color = joinmodal.querySelector('input[name="color"]:checked').value;

		joinmodal.style.display = "none";
		form.reset();
		runClient(nick, color);
	});

	var emitters = (async () => {
		const response = await fetch(`${window.location.origin}/static/emitters.json`);
		if (!response.ok){
			console.error("Can't load emitters");
			return null;
		}
		return await response.json();
	})();

	async function runClient(player_nick, player_color){
		const items_div = document.getElementById("shop-items");
		for (let i = 0;i < num_shop_items(); ++i){
			let item = get_shop_item(i);
			console.log(item.cost);
			console.log(item.display_name());
			const [item_card, buy_btn] = createItemCard("path_to_image", item.display_name(), item.display_name(), "description", item.cost);
			items_div.appendChild(item_card);
			buy_btn.addEventListener('click', () => {
				console.log(`buy ${item.display_name()} ${item.id}`);
				if (gameState[public_id].p.cash < item.cost){
					alert("Not enough money");
					return;
				}
				gameState[public_id].p.cash -= item.cost;
				cash_text.text = gameState[public_id].p.cash;
				perform_update("AddBoost");
			});
		}

		emitters = await emitters;
		if (!emitters){
			console.error("Halting client - error loading emitters");
			return;
		}

		var gameState = {};
		var worldLoot = {};
		var emissions = [];
		var bodies = [];
		var socket = null;
		var opened = false;
		var public_id = null;

		const keyleft = "a";
		const keyright = "d";
		const keyup = "w";
		const keyshoot = " ";
		const keyshop = "escape";
		const keyzoomin = "arrowup";
		const keyzoomout = "arrowdown";

		var keymap = {
			[keyleft]: false,
			[keyright]: false,
			[keyup]: false,
			[keyshoot]: false,
			[keyshop]: false,
			[keyzoomin]: false,
			[keyzoomout]: false,
		};

		const invr2 = 0.7071067811865475;
		const PI = 3.14159265358979323;
		const ACCELERATION = 200.0;
		const G = 2000.0;
		const TIMESTEP_FPS = 8;

		const bullet_distance = 500;
		const player_speed = 200;
		const player_radius = 25;
		const dome_radius = 6000;
		const fadeRate = 8;
		const rotation_speed = PI;

		const healthbar_maxwidth = 0.15; //This gets multiplied by the  screen width

		var pingInterval = null;
		var lastPing = 0;
		var current_rtt = null;
		var clocks_delta = 0; //estimated difference between client/server clocks

		const app = new PIXI.Application({
				width: window.innerWidth,
				height: window.innerHeight,
				backgroundColor: 0x101510
		});
		const TIMESTEP = 1 / TIMESTEP_FPS;

		let gunshot_texture, pistol_ammo_texture, coins_texture, coins_label_texture, heart_texture, seamless_texture, speed_boost_texture;
		await Promise.all([
			PIXI.Assets.load("static/textures/gunshot.png").then(texture => gunshot_texture = texture),
			PIXI.Assets.load("static/textures/pistol_ammo.png").then(texture => pistol_ammo_texture = texture),
			PIXI.Assets.load("static/textures/coins.png").then(texture => coins_texture = texture),
			PIXI.Assets.load("static/textures/coins_label.png").then(texture => coins_label_texture = texture),
			PIXI.Assets.load("static/textures/heart.png").then(texture => heart_texture = texture),
			PIXI.Assets.load("static/textures/seamless.jpg").then(texture => seamless_texture = texture),
			PIXI.Assets.load("static/textures/speed_boost.png").then(texture => speed_boost_texture = texture)
		]);

		var background = new PIXI.TilingSprite(
			seamless_texture,
			(3+Math.ceil(app.screen.width / seamless_texture.width))*seamless_texture.width,
			(3+Math.ceil(app.screen.height / seamless_texture.height))*seamless_texture.height
		);

		const world = new PIXI.Container();
		world.position.set(app.screen.width/2, app.screen.height/2);

		//tells pixijs to consider the zindex of the children
		world.sortableChildren = true;

		const players_container = new PIXI.Container();
		const loot_container = new PIXI.Container();
		const bullets_container = new PIXI.Container();
		const bodies_container = new PIXI.Container();

		//higher zindex makes it appear on top
		bullets_container.zIndex = 1;
		players_container.zIndex = 2;
		bodies_container.zIndex = 3;
		loot_container.zIndex = 4;

		const worldMask = new PIXI.Graphics();
		worldMask.beginFill(0xffffff);
		worldMask.drawCircle(0,0,dome_radius);
		worldMask.endFill();

		world.addChild(players_container);
		world.addChild(loot_container);
		world.addChild(bullets_container);
		world.addChild(bodies_container);
		world.addChild(background);
		world.addChild(worldMask);
		world.mask = worldMask;

		app.stage.addChild(world);

		//Add the healthbar
		var healthbar = new PIXI.Sprite(PIXI.Texture.WHITE);
		healthbar.width = app.screen.width * healthbar_maxwidth;
		healthbar.height = 20;
		healthbar.tint = 0x00ff00;
		healthbar.position.set(app.screen.width*0.01, app.screen.height*0.98 - 20);
		app.stage.addChild(healthbar);

		//Add the hearth next to the healthbar TODO reduce the hard coded values and make a container for the healthbar and the heart
		var heart_sprite = new PIXI.Sprite(heart_texture);
		heart_sprite.scale.set(0.7,0.7);
		heart_sprite.position.set(
			app.screen.width * (healthbar_maxwidth+0.02),
			app.screen.height*0.98 - 10 - heart_sprite.height/2
		);
		app.stage.addChild(heart_sprite);

		//add the coords text
		var coords_text = new PIXI.Text("x: -, y: -", { fontFamily: "Arial", fontSize: 18, fill: 0x88ff88 });
		coords_text.anchor.set(0.5);
		coords_text.position.set(app.screen.width * 0.93, app.screen.height*0.02);
		app.stage.addChild(coords_text);

		//add the fps text
		var fps_text = new PIXI.Text("fps: -", { fontFamily: "Arial", fontSize: 18, fill: 0x88ff88 });
		fps_text.anchor.set(0.5);
		fps_text.position.set(app.screen.width * 0.93, app.screen.height*0.02 + 20);
		app.stage.addChild(fps_text);

		//add the latency text
		var latency_text = new PIXI.Text("latency: ", { fontFamily: "Arial", fontSize: 18, fill: 0x88ff88 });
		latency_text.anchor.set(0.5);
		latency_text.position.set(app.screen.width * 0.93, app.screen.height*0.02 + 40);
		app.stage.addChild(latency_text);

		//add the cash text
		var cash_text = new PIXI.Text("?", {
			fill: "#e5a50a",
			fillGradientType: 1,
			fontFamily: "Helvetica",
			fontSize: 32,
			fontWeight: "bold",
			letterSpacing: 1,
			lineJoin: "round",
			stroke: "#99c1f1"
		});
		cash_text.anchor.set(0.5);
		cash_text.position.set(app.screen.width * 0.02, app.screen.height*0.02 + 20);
		app.stage.addChild(cash_text);

		var coin_sprite = new PIXI.Sprite(coins_label_texture);
		coin_sprite.position.set(
			app.screen.width * 0.05,
			app.screen.height * 0.02
		);
		app.stage.addChild(coin_sprite);

		//Add the text that shows ammo left
		var ammo_text = new PIXI.Text("?", { fontFamily: "\"Lucida Console\", Monaco, monospace", fontSize: 23, fill: 0xffee00 });
		ammo_text.anchor.set(0.5);
		ammo_text.position.set(app.screen.width * 0.9, app.screen.height*0.98 - 15);
		app.stage.addChild(ammo_text);

		var pistol_ammo_sprite = new PIXI.Sprite(pistol_ammo_texture);
		//heart_sprite.scale.set(0.7,0.7);
		pistol_ammo_sprite.position.set(
			app.screen.width * 0.93,
			app.screen.height*0.98 - 35
		);
		app.stage.addChild(pistol_ammo_sprite);

		//false if not hit, distance to hit point otherwise
		//TODO wasm
		function line_circle_intersect(xp, yp, xc, yc, rot){
			//shift everything to make line start from origin
			let a = xc - xp;
			let b = yc - yp;
			let rot_90 = rot - PI/2;

			//compute the quadratic's 'b' coefficient (for variable r in polar form)
			let qb = -(2*a*Math.cos(rot_90) + 2*b*Math.sin(rot_90));
			let discriminant = qb*qb - 4*(a*a + b*b - player_radius*player_radius);
			if (discriminant < 0){ //no real roots (no line-circle intersection)
				return false;
			}

			let root = Math.sqrt(discriminant);

			//the actual solutions
			const r1 = (root - qb)/2;
			const r2 = (-root - qb)/2;

			const r1Good = bullet_distance > r1 && r1 > 0;
			const r2Good = bullet_distance > r2 && r2 > 0;

			if (!r1Good && !r2Good)
				return false;
			else if (r1Good != r2Good){
				if (r1Good)
					return r1;
				else
					return r2;
			} else if (r1Good && r2Good){
				return Math.min(r1, r2);
			}
		}

		const openWebSocket = function(){
			fetch(`${window.location.origin}/register`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(
					{
						nick: player_nick,
						color: player_color
					}
				)
			})
			.then(response => response.json())
			.then(result => {
				public_id = result['public'];
				//socket = new WebSocket(`wss://${window.location.hostname}/ws/`+result['private']);
				socket = new WebSocket(`ws://${window.location.host}/ws/`+result['private']);
				socket.onmessage = recvFn;
				socket.onopen = () => {
					opened=true;
					const pingFn = () => { //TODO: instead of using ping, use other things to measure latency
						socket.send(JSON.stringify({"t": "Ping"}));
						lastPing = local_time();
						setTimeout(pingFn, 5000+(Math.random()*5000));
					}
					pingFn();
				};
				socket.onclose = (event)=>{
					opened=false;
					clearInterval(pingInterval);
				}
			});
		};

		const getPlayerSprite = function(player){
			var player_container = new PIXI.Container();
			player_container.position.set(player.trajectory.pos.x, player.trajectory.pos.y);

			let actualBody = new PIXI.Container();
			actualBody.rotation = player.trajectory.spin;
			var text = new PIXI.Text(player.name, { fontFamily: "Arial", fontSize: 16, fill: 0xffffff });
			text.anchor.set(0.5);
			text.position.set(0, -60);

			let circle = new PIXI.Graphics();
			circle.beginFill(player.color.r << 16 | player.color.g << 8 | player.color.b);
			circle.drawCircle(0, 0, 25);
			circle.endFill();
			let circleTexture = app.renderer.generateTexture(circle);
			circle = new PIXI.Sprite(circleTexture);
			circle.anchor.set(0.5);

			let thruster = new PIXI.Graphics();
			thruster.beginFill(0x808080);
			thruster.drawPolygon([
				-20,0,
				20,0,
				0,-30,
			]);
			thruster.endFill();
			let thrusterTexture = app.renderer.generateTexture(thruster)
			thruster = new PIXI.Sprite(thrusterTexture);
			thruster.anchor.set(0.5);
			thruster.position.set(0, 20);

			let weapon = new PIXI.Sprite(PIXI.Texture.WHITE);
			weapon.anchor.set(0.5);
			weapon.width = 5;
			weapon.height = 20;
			weapon.anchor.set(0.5);
			weapon.tint = 0x000000;
			weapon.position.set(0, -35);

			actualBody.addChild(weapon);
			actualBody.addChild(thruster);
			actualBody.addChild(circle);
			player_container.addChild(actualBody);
			player_container.addChild(text);
			return {
				container: player_container,
				sprite: actualBody,
			};
		}

		const getLootSprite = function(lootObj){
			var loot_obj = new PIXI.Container();
			loot_obj.position.set(lootObj.x, lootObj.y);
			var loot_texture = new PIXI.Sprite({
				"Cash": coins_texture,
				"PistolAmmo": pistol_ammo_texture,
				"SpeedBoost": speed_boost_texture
			}[typeof lootObj.loot === "string" ? lootObj.loot:Object.keys(lootObj.loot)[0]]);
			loot_texture.anchor.set(0.5);
			loot_obj.addChild(loot_texture);
			return loot_obj;
		}

		const update_healthbar = function(healthvalue){
			const prcnt = healthvalue / 100;
			healthbar.width = Math.max(0, app.screen.width * healthbar_maxwidth * prcnt);
			healthbar.tint = Math.round(0xff * prcnt) << 8 | Math.round((1-prcnt) * 0xff) << 16;
		}

		const handle_gamestate = function(state){
			//remove all other sprites
			for (var i = world.children.length - 1; i >= 0; i--)
				players_container.removeChild(world.children[i]);

			//TODO also remove loot

			gameState = {};
			worldLoot = {};
			bodies = [];
			state.players.forEach((p) => {
				p.trajectory = new Trajectory(p.trajectory);
				const square = getPlayerSprite(p);
				if (p.id == public_id) {
					square.container.x = app.screen.width/2;
					square.container.y = app.screen.height/2;
					world.pivot.x = p.trajectory.pos.x;
					world.pivot.y = p.trajectory.pos.y;
					app.stage.addChild(square.container);
					ammo_text.text = p.inventory.weapons[0].ammo;
					cash_text.text = p.cash;
				} else {
					players_container.addChild(square.container);
				}

				gameState[p.id] = {
					graphics: square.container,
					child: square.sprite,
					p: p
				};
			});	

			Object.entries(state.loot).forEach(([loot_uuid, lootObj]) => {
				summon_loot(loot_uuid, lootObj);
			});

			for (let i = 0;i < num_bodies();++i){
				const body = getbody(i);
				var body_obj = new PIXI.Container();
				let circle = new PIXI.Graphics();
				circle.beginFill(0xffffff);
				circle.drawCircle(0, 0, body.radius);//body.radius
				circle.endFill();
				let circleTexture = app.renderer.generateTexture(circle);
				circle = new PIXI.Sprite(circleTexture);
				circle.anchor.set(0.5);

				body_obj.position.set(body.pos.x, body.pos.y);
				body_obj.addChild(circle);
				bodies_container.addChild(body_obj);
				bodies.push(body);
			}
		};

		const summon_loot = function(loot_uuid, lootObj){
			const loot_sprite = getLootSprite(lootObj);
			loot_container.addChild(loot_sprite);
			worldLoot[loot_uuid] = {
				graphics: loot_sprite,
				l: lootObj,
			};
		}

		const handle_update = function(content){
			let broadcaster = content["from"];
			if (broadcaster == public_id)
				return;

			if (content["time"] < Number(gameState[broadcaster].p.trajectory.time)) {
				console.error(`update is in the past`);
			}
			gameState[broadcaster].p.trajectory.insert_update(UpdateType[content.change], content["at"], BigInt(content["time"]));
		}

		const change_propulsion_emitter = (pid, is_emitting) => {
			if (gameState[pid].emitter && gameState[pid].emitter.emit == is_emitting){ //do nothing if already in that state
				return;
			}
			if (is_emitting){
				let emitJSON = JSON.parse(JSON.stringify(emitters["propel"])); //careful here
				emitJSON.behaviors.push(
					{
						type: 'textureSingle',
						config: {
								texture: PIXI.Texture.WHITE
							}
					}
				);

				let emitter = new PIXI.particles.Emitter(
					gameState[pid].child,
					emitJSON
				);
				emitter.emit = true;
				emissions.push(emitter);
				delete gameState[pid].emitter;
				gameState[pid].emitter = emitter;
			} else {
				if (gameState[pid].emitter){
					gameState[pid].emitter.emit = false;
				}
			}		
		}

		const handle_trigUpdate = function(content){
			const weapon = content.weptype;
			const updater = content.by;
			const isTriggered = content.pressed;
			if (!isTriggered) //temporary
				return;
			if (weapon == "Pistol"){
				const line_start_x = public_id == updater ? world.pivot.x:gameState[updater].graphics.x;
				const line_start_y = public_id == updater ? world.pivot.y:gameState[updater].graphics.y;

				const line_rotation = gameState[updater].child.rotation;
				let hitInfo = {
					hit: false,
					shortest_line: bullet_distance,
					x: 0,
					y: 0
				};
				Object.entries(gameState).forEach(([pubid, item]) => {
					if (pubid == updater) //don't check if the shooter is shooting themselves
						return;

					const check_x = pubid == public_id ? world.pivot.x:item.graphics.x;
					const check_y = pubid == public_id ? world.pivot.y:item.graphics.y;

					const hit = line_circle_intersect(line_start_x, line_start_y, check_x, check_y, line_rotation);
					if (hit === false)
						return;

					hitInfo.hit = true;
					if (hit >= hitInfo.shortest_line)
						return;

					hitInfo.shortest_line = hit;
					hitInfo.x = check_x;
					hitInfo.y = check_y;
				});

				//draws a line instead of gunshot
				//const bullet_line = new PIXI.Graphics();
				//bullet_line.lineStyle(4, 0xffff00, 1);
				//bullet_line.position.set(line_start_x, line_start_y);
				//bullet_line.lineTo(0, -hitInfo.shortest_line);
				//bullet_line.rotation = line_rotation;
				//bullets_container.addChild(bullet_line);

				//TODO offset this a bit so that it appears at tip of gun
				const gunshot_sprite = new PIXI.Sprite(gunshot_texture);
				gunshot_sprite.position.set(line_start_x, line_start_y);
				gunshot_sprite.rotation = line_rotation;
				gunshot_sprite.anchor.set(0.5, 1.2); //this affects the gunshot position relative to the shooter
				gunshot_sprite.scale.set(0.25, 0.25); //This depends on the png size
				bullets_container.addChild(gunshot_sprite);

				if (hitInfo.hit){
					let emitJSON = JSON.parse(JSON.stringify(emitters["bulletHit"])); //careful here
					emitJSON.pos = {
						x: hitInfo.x,
						y: hitInfo.y
					};
					emitJSON.behaviors.push(
						{
								type: 'rotationStatic',
								config: {
										min: (180/PI)*line_rotation+90 - 30,
										max: (180/PI)*line_rotation+90 + 30
								}
						}
					);
					emitJSON.behaviors.push(
						{
							type: 'textureSingle',
							config: {
									texture: PIXI.Texture.WHITE
								}
						}
					);

					let emitter = new PIXI.particles.Emitter(
						world,
						emitJSON
					);
					emitter.emit = true;
					emissions.push(emitter);
				}
			}
		}

		const handle_playerjoin = function(content){
			content.trajectory = new Trajectory(content.trajectory);
			if (content.id == public_id){ //this happens when spawning
				world.pivot.x = content.trajectory.pos.x;
				world.pivot.y = content.trajectory.pos.y;
				gameState[public_id].p = content;
				gameState[public_id].child.rotation = content.trajectory.spin;
				
				//update coords text
				coords_text.text = `x: ${Math.round(world.pivot.x)}, y: ${-Math.round(world.pivot.y)}`;

				//update health bar
				update_healthbar(content.health);

				//update ammo bar
				const inventory = content.inventory;
				const selectedWeapon = inventory.weapons[inventory.selection];
				ammo_text.text = selectedWeapon.ammo;

				//update cash bar
				cash_text.text = content.cash;
			} else {
				const square = getPlayerSprite(content);
				players_container.addChild(square.container);
				gameState[content.id] = {
					graphics: square.container,
					child: square.sprite,
					p: content
				};
			}
		}

		const handle_healthUpdate = function(newHealth){
			gameState[public_id].p.health = newHealth;
			update_healthbar(newHealth);
		}

		const handle_playerleave = function(public_id){
			players_container.removeChild(gameState[public_id].graphics);
			delete gameState[public_id];
		}

		const handle_playerdeath = function(content){
			console.log(content);
			const is_self = content.from == public_id;
			if (is_self){
				if (content.loot){
					summon_loot(content.loot.uuid, content.loot.object);
				}
				alert("You died!");
				socket.send(
					JSON.stringify({
						"t":"Spawn"
					})
				);
			} else {
				const emitJSON = JSON.parse(JSON.stringify(emitters["explosion"]));
				emitJSON.pos = {
					x: gameState[content.from].p.trajectory.pos.x,
					y: gameState[content.from].p.trajectory.pos.y
				};
				emitJSON.behaviors.push({
					type: 'textureSingle',
					config: {
						texture: PIXI.Texture.WHITE
					}
				});
				let emitter = new PIXI.particles.Emitter(
					world,
					emitJSON
				);
				emitter.emit = true;
				emissions.push(emitter);
				if (content.loot){
					summon_loot(content.loot.uuid, content.loot.object);
				}
				handle_playerleave(content.from);
			}
		}

		const handle_rejection = function(content){
			console.log(`rejection: ${content}`);
			delete worldLoot[content].claimed;
		}

		const handle_correction = function(content){
			gameState[content['id']].p.trajectory = new Trajectory(content['tr']);
		}

		const handle_pong = function(content){
			const original_rtt = current_rtt;
			const now = local_time();
			current_rtt = now - lastPing;
			clocks_delta = now - content - Math.round(current_rtt/2);
			latency_text.text = `latency: ${current_rtt}ms`;
			if (original_rtt == null)
				socket.send(JSON.stringify({"t":"StateQuery"}));
		}

		const handle_lootcollection = function(content){
			if (!(content.loot_id in worldLoot)){
				console.error("Could not find collected loot");
				return;
			}
			loot_container.removeChild(worldLoot[content.loot_id].graphics);
			const loot_content_clone = JSON.parse(JSON.stringify(worldLoot[content.loot_id].l.loot));
			delete worldLoot[content.loot_id];

			const loot_type = typeof loot_content_clone === 'string' ? loot_content_clone:Object.keys(loot_content_clone)[0];
			const loot_value = typeof loot_content_clone === 'string' ? null:Object.values(loot_content_clone)[0];
			if (content.collector !== public_id && loot_type !== "SpeedBoost") return;

			({
				"Cash": () => {
					gameState[content.collector].p.cash += loot_value;
					cash_text.text = gameState[content.collector].p.cash;
				},
				"PistolAmmo": () => {
					const pp = gameState[content.collector].p;
					pp.inventory.weapons[pp.inventory.selection].ammo += loot_value;
					ammo_text.text = pp.inventory.weapons[pp.inventory.selection].ammo;
				},
				"SpeedBoost": () => {
					gameState[content.collector].p.speed += 1.0;
				}
			}[loot_type])();
		}

		const recvFn = function(event) {
			let data = JSON.parse(event.data);
			console.log(data);
			let datatype = data["t"];
			let content = data["c"];
			const fmap = {
				"Pong": handle_pong,
				"PlayerJoin": handle_playerjoin,
				"PlayerLeave": handle_playerleave,
				"HealthUpdate": handle_healthUpdate,
				"GameState": handle_gamestate,
				"TrajectoryUpdate": handle_update,
				"PlayerDeath": handle_playerdeath,
				"LootCollected": handle_lootcollection,
				"TrigUpdate": handle_trigUpdate,
				"Correct": handle_correction,
				"LootReject": handle_rejection
			};
			if (!(datatype in fmap)){
				console.error(`received unknown server message: ${JSON.stringify(content)}`);
				return;
			}
			fmap[datatype](content);
		};

		function perform_update(utype) {
			const chAt = gameState[public_id].p.trajectory.hash_str();
			const time = Number(gameState[public_id].p.trajectory.time);
			gameState[public_id].p.trajectory.apply_change(UpdateType[utype]);
			socket.send(
				JSON.stringify({
					"t":"TrajectoryUpdate",
					"c":{
						"change": utype,
						"at": chAt,
						"time": time,
					 }
				})
			);		
		}

		const keyAction = function (repeated, name, up){
			if (!opened || repeated) return;

			name = name.toLowerCase();
			if (keymap[name] === undefined)
				return;

			keymap[name] = !up;

			if (name == keyright || name == keyleft){
				let response = "";
				if (keymap[keyleft] == keymap[keyright]){
					response = "RotStop";
				} else if (keymap[keyleft]){
					response = "RotCcw";
				} else if (keymap[keyright]){
					response = "RotCw";
				}
				perform_update(response);
			} else if (name == keyup) {
				let change = up ? "PropOff":"PropOn";
				perform_update(change);
				change_propulsion_emitter(public_id, !up);
			} else if (name == keyshoot) {
				return;
				const inventory = gameState[public_id].p.inventory;
				const selectedWeapon = inventory.weapons[inventory.selection];
				if (selectedWeapon.ammo <= 0)
					return;
				if (keymap[name])
					ammo_text.text = --selectedWeapon.ammo;
				socket.send(
					JSON.stringify({
						"t":"TrigUpdate",
						"c":{
							"pressed": keymap[name],
						 }
					})
				);
			} else if (name == keyshop) {
				document.getElementById("shop-modal").style.display = keymap[name] ? "flex":"none";
			} else if ((name == keyzoomout || name == keyzoomin) && !up) {
				let multiplier = 1.5;
				if (name == keyzoomout){
					multiplier = 1 / multiplier;
				}
				world.scale.x *= multiplier;
				world.scale.y *= multiplier;
				gameState[public_id].graphics.scale.x *= multiplier;
				gameState[public_id].graphics.scale.y *= multiplier;
			}
		};

		document.body.appendChild(app.view);
		openWebSocket();

		var fps_sum = 0;
		var fps_ctr = 0;
		const GraphicsTicker = PIXI.Ticker.shared.add(delta => {
			fps_sum += PIXI.Ticker.shared.FPS;
			fps_ctr++;
			if (fps_ctr > 100){
				fps_text.text = `fps: ${Math.round(fps_sum / fps_ctr)}`;
				fps_ctr = fps_sum = 0;
			}

			if (current_rtt == null)
				return;

			const deltaTime = delta / (1000*PIXI.settings.TARGET_FPMS);

			Object.entries(gameState).forEach(([pid, player]) => {
				if (player.p.trajectory.collision)
					return;
				if (pid == public_id){
					player.p.trajectory.advance(BigInt(server_time()), false);
					return;
				}
				const ptime = BigInt(server_time() - (50 + current_rtt));
				if (player.p.trajectory.advance(ptime, false)){
					change_propulsion_emitter(pid, player.p.trajectory.propelling);
				} else {
					socket.send(JSON.stringify({"t": "Correct", "c": pid}));
				}
			});
			
			Object.values(gameState).forEach(player => {
				if (player.p.trajectory.collision)
					return;
				const shallow_copy = player.p.id == public_id ? world.pivot:player.graphics

				//lerp the positions
				let now;
				if (player.p.id == public_id){
					now = server_time();
				} else {
					now = server_time() - (50 + current_rtt);
				}

				const lerped = player.p.trajectory.lerp(BigInt(now));
				shallow_copy.x = lerped.x;
				shallow_copy.y = lerped.y;
				player.child.rotation = lerped.r;
			});

			const tile_x = Math.floor(world.pivot.x / seamless_texture.width);
			const tile_y = Math.floor(world.pivot.y / seamless_texture.height);
			background.x = tile_x*seamless_texture.width - background.width/2;
			background.y = tile_y*seamless_texture.height - background.height/2;
			coords_text.text = `x: ${Math.round(world.pivot.x)}, y: ${-Math.round(world.pivot.y)}`;

			for (let i = 0; i < bullets_container.children.length; ++i){
				bullets_container.children[i].scale.x = Math.max(
					0,
					bullets_container.children[i].scale.x-fadeRate*deltaTime
				);
			}
			//remove the gunshots from memory
			bullets_container.children = bullets_container.children.filter(child => child.scale.x > 0.01);

			emissions.forEach(emitter => {
				if (!emitter.emit)
					emitter.destroy();
			});
			emissions = emissions.filter(emitter => emitter.emit);
			emissions.forEach(emitter => emitter.update(deltaTime));

			//TODO it happens often that server rejects claim
			Object.entries(worldLoot).forEach(([loot_id, lootObj]) => {
				if (lootObj.claimed)
					return;
				const trig = Math.pow(lootObj.l.x - world.pivot.x, 2) + Math.pow(lootObj.l.y - world.pivot.y, 2) < 10*10;
				if (!trig)
					return;
				lootObj.claimed = true;
				socket.send(JSON.stringify({"t":"ClaimLoot","c":{"loot_id": loot_id}}));
			});
		});

		GraphicsTicker.speed = 1;
		GraphicsTicker.minFPS = 30;
		GraphicsTicker.maxFPS = 60;

		window.addEventListener("resize", function(){
			app.renderer.resize(window.innerWidth, window.innerHeight);
		});

		window.addEventListener('keydown', (event) => keyAction(event.repeat, event.key, false));
		window.addEventListener('keyup', (event) => keyAction(event.repeat, event.key, true));

		function server_time() {
			return local_time() - clocks_delta;
		}

		function local_time(){
			return Date.now();
		}
	}
}
runAll();

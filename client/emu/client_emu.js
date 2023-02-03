const WebSocket = require('ws');

const recvFn = function(event) {
	//console.log("receive: " + event.data);
};

const delay = 20;

const schedule = [
	"MoveDown",
	"MoveRight",
	"MoveDown",
	"MoveRight",
	"MoveRight",
	"Stopped",
	"MoveLeft",
	"MoveLeft",
	"MoveUp",
	"MoveLeft",
	"MoveUp",
]

var sockets = [];

const openWebSocket = function(){
	fetch(`http://127.0.0.1/register`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: "{}"
	})
	.then(response => response.json())
	.then(result => {
		var socket = new WebSocket(`ws://127.0.0.1/ws/`+result['private']);
		sockets.push(socket);
		console.log("clients = " + sockets.length);
		socket.onmessage = recvFn;
		socket.addEventListener('open', function (event){
			let index = 0;
			setInterval(
				() => {
					socket.send(
						JSON.stringify(
							{"t":"MotionUpdate", "c":{"motion": schedule[index]}}
						)
					);
					index += 1;
					index %= schedule.length;
				}, delay
			);
		});
		socket.onclose = (event)=>{
			console.log(event.wasClean ? "closed cleanly":"connection died");
		}
	});
}

setInterval(() => openWebSocket(), 20);

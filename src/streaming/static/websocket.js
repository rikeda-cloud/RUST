var ws = new WebSocket('ws://' + window.location.host + '/ws');
ws.binaryType = 'arraybuffer';

ws.onmessage = function(event) {
	var img = document.getElementById('stream');
	img.src = URL.createObjectURL(new Blob([event.data], { type: 'image/jpeg' }));
};

function sendNumber(number) {
	console.log(JSON.stringify({ type: 'number_selection', key: number }));
	ws.send(JSON.stringify({ number: number }));
}

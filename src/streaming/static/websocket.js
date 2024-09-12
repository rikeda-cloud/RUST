var graph;
var ws;
var node_details = [
	"canny",
	"binary",
	"face",
	"white_balance",
	"superpixel",
	"haar_like",
	"removed_red",
	"removed_green",
	"removed_blue",
	"text",
];
var camera = "camera";

function initializeWebSocket() {
	ws = new WebSocket('ws://' + window.location.host + '/ws');
	ws.binaryType = 'arraybuffer';

	ws.onopen = function() { console.log('WebSocket connection opened'); };
	ws.onclose = function() { console.log('WebSocket connection closed'); };
	ws.onerror = function(error) { console.error('WebSocket error: ', error); };

	ws.onmessage = function(event) {
		var img = document.getElementById('stream');
		img.src = URL.createObjectURL(new Blob([event.data], { type: 'image/jpeg' }));
	};
}

function sendNodeConnections() {
	var cells = graph.getModel().cells;
	var connections = Object.keys(cells)
		.map(cellId => cells[cellId])
		.filter(cell => cell.edge)
		.map(cell => ({
			source: cell.source ? cell.source.value : null,
			target: cell.target ? cell.target.value : null
		}));
	ws.send(JSON.stringify({ nodes: connections }));
}

function main(container) {
	graph = new mxGraph(container);
	var parent = graph.getDefaultParent();
	initializeWebSocket();

	graph.getModel().beginUpdate();
	try {
		var _ = graph.insertVertex(parent, null, camera, 500, 100, 100, 50, 'rounded=1;fillColor=#FF6666;fontColor=#FFFFFF');

		var nodes = [];
		for (var i = 0; i < 10; i++) {
			var row = Math.floor(i / 2);
			var col = i % 2;
			var nodeXPosition = 50 + col * 100;
			var nodeYPosition = 30 + row * 40;
			var node = graph.insertVertex(parent, null, node_details[i], nodeXPosition, nodeYPosition, 80, 30, 'rounded=1;fillColor=#66CCFF;fontColor=#000000');
			nodes.push(node);
		}
	} finally {
		graph.getModel().endUpdate();
	}

	graph.setConnectable(true);
	graph.setAllowDanglingEdges(false);
	graph.isValidSource = function(cell) { return cell.value !== camera; };
	graph.isValidTarget = function(_) { return true; };

	// 1対1の接続制限ロジック（カメラ以外のノードは1つの入力と1つの出力を持てる）
	graph.addEdge = function(edge, parent, source, target, index) {
		var sourceEdges = graph.getModel().getOutgoingEdges(source);
		var targetEdges = graph.getModel().getIncomingEdges(target);

		if (source.value === camera) { return null; }
		if (sourceEdges.length > 0 || targetEdges.length > 0) { return null; }
		return mxGraph.prototype.addEdge.apply(this, arguments);
	};

	// エッジが追加された後にノード情報を送信
	graph.addListener(mxEvent.ADD_CELLS, function(sender, evt) {
		var cells = evt.getProperty('cells');
		cells.forEach(function(cell) {
			if (cell.edge) {
				sendNodeConnections();
			}
		});
	});

	document.addEventListener('keydown', function(event) {
		if (event.ctrlKey && event.key === 'x') {
			var selectedCells = graph.getSelectionCells();
			graph.getModel().beginUpdate();
			try {
				for (var i = 0; i < selectedCells.length; i++) {
					var cell = selectedCells[i];
					if (cell.edge) { graph.removeCells([cell]); }
				}
				sendNodeConnections();
			} finally {
				graph.getModel().endUpdate();
			}
			event.preventDefault();
		}
	});
}

// グラフ描画領域を設定
var container = document.getElementById('graphContainer');
container.style.overflow = 'hidden';
container.style.position = 'relative';
main(container);

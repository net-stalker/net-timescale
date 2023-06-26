import { Envelope } from './transport/net-proto-api/envelope.js';

import { TimeIntervalDTO } from './transport/net-timescale-api/time_interval.js';

import { GraphEdgeDTO } from './transport/net-timescale-api/graph_edge.js';
import { GraphNodeDTO } from './transport/net-timescale-api/graph_node.js';
import { NetworkGraphDTO } from './transport/net-timescale-api/network_graph.js';

const serverURL = 'ws://localhost:9091';

let socket;
// variables for the DOM elements:
let incomingSpan;
let outgoingText;
let connectionSpan;
let connectButton;

function setup() {
  // get all the DOM elements that need listeners:
  incomingSpan = document.getElementById('incoming');
  outgoingText = document.getElementById('outgoing');
  connectionSpan = document.getElementById('connection');
  connectButton = document.getElementById('connectButton');
  // set the listeners:
  outgoingText.addEventListener('change', sendMessage);
  connectButton.addEventListener('click', changeConnection);
  openSocket(serverURL);
}

function openSocket(url) {
  // open the socket:
  socket = new WebSocket(url);
  socket.addEventListener('open', openConnection);
  socket.addEventListener('close', closeConnection);
  socket.addEventListener('message', readIncomingMessage);
}

function changeConnection(event) {
  // open the connection if it's closed, or close it if open:
  if (socket.readyState === WebSocket.CLOSED) {
    openSocket(serverURL);
  } else {
    socket.close();
  }
}

function openConnection() {
  // display the change of state:
  connectionSpan.innerHTML = "true";
  connectButton.value = "Disconnect";
}

function closeConnection() {
  // display the change of state:
  connectionSpan.innerHTML = "false";
  connectButton.value = "Connect";
}

function readIncomingMessage(event) {
  // display the incoming message:
  incomingSpan.innerHTML = JSON.parse(event.data);
}

function sendMessage() {
  //if the socket's open, send a message:
  if (socket.readyState === WebSocket.OPEN) {
    socket.send(outgoingText.value);
  }
}


let textEncoder = new TextEncoder();
let textDecoder = new TextDecoder();


console.log(Envelope.decode((new Envelope("ENVELOPE_TYPE", textEncoder.encode("ENVELOPE_DATA"))).encode()));
console.log((new Envelope("ENVELOPE_TYPE", textEncoder.encode("ENVELOPE_DATA"))).encode());
console.log(textDecoder.decode((new Envelope("ENVELOPE_TYPE", textEncoder.encode("ENVELOPE_DATA"))).encode()));
//Encoded from Rust side
console.log(Envelope.decode([123, 116, 121, 112, 101, 58, 32, 34, 69, 78, 86, 69, 76, 79, 80, 69, 95, 84, 89, 80, 69, 34, 44, 32, 100, 97, 116, 97, 58, 32, 123, 123, 82, 85, 53, 87, 82, 85, 120, 80, 85, 69, 86, 102, 82, 69, 70, 85, 81, 81, 61, 61, 125, 125, 125]));

console.log(TimeIntervalDTO.decode((new TimeIntervalDTO(0, 100)).encode()))

console.log(GraphNodeDTO.decode((new GraphNodeDTO("GRAPH_NODE")).encode()));
console.log(GraphEdgeDTO.decode((new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")).encode()));

console.log(NetworkGraphDTO.decode((new NetworkGraphDTO([new GraphNodeDTO("GRAPH_FIRST_NODE"), new GraphNodeDTO("GRAPH_SECOND_NODE")], [new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")])).encode()));
console.log((new NetworkGraphDTO([new GraphNodeDTO("GRAPH_FIRST_NODE"), new GraphNodeDTO("GRAPH_SECOND_NODE")], [new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")])).encode());
console.log(textDecoder.decode((new NetworkGraphDTO([new GraphNodeDTO("GRAPH_FIRST_NODE"), new GraphNodeDTO("GRAPH_SECOND_NODE")], [new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")])).encode()));
//Encoded from Rust side
console.log(NetworkGraphDTO.decode([123, 103, 114, 97, 112, 104, 95, 110, 111, 100, 101, 115, 58, 32, 91, 123, 97, 100, 100, 114, 101, 115, 115, 58, 32, 34, 71, 82, 65, 80, 72, 95, 70, 73, 82, 83, 84, 95, 78, 79, 68, 69, 34, 125, 44, 32, 123, 97, 100, 100, 114, 101, 115, 115, 58, 32, 34, 71, 82, 65, 80, 72, 95, 83, 69, 67, 79, 78, 68, 95, 78, 79, 68, 69, 34, 125, 93, 44, 32, 103, 114, 97, 112, 104, 95, 101, 100, 103, 101, 115, 58, 32, 91, 123, 115, 114, 99, 95, 97, 100, 100, 114, 58, 32, 34, 71, 82, 65, 80, 72, 95, 70, 73, 82, 83, 84, 95, 78, 79, 68, 69, 34, 44, 32, 100, 115, 116, 95, 97, 100, 100, 114, 58, 32, 34, 71, 82, 65, 80, 72, 95, 83, 69, 67, 79, 78, 68, 95, 78, 79, 68, 69, 34, 125, 93, 125]));

// add a listener for the page to load:
window.addEventListener('load', setup);
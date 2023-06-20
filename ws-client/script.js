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

function blobToArrayBuffer(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onloadend = () => {
      if (reader.readyState === FileReader.DONE) {
        resolve(reader.result);
      } else {
        reject(new Error('Unable to read Blob as ArrayBuffer.'));
      }
    };

    reader.onerror = () => {
      reject(new Error('Error occurred while reading Blob.'));
    };

    reader.readAsArrayBuffer(blob);
  });
}

function readIncomingMessage(event) {
  // display the incoming message:
  const blob = event.data;
  console.log(blob);
   blobToArrayBuffer(blob)
     .then((arrayBuffer) => {
       let buffer = new Uint8Array(arrayBuffer);
       console.log(buffer);
       let envelope = Envelope.decode(buffer);
       console.log(envelope);
       let network_graph = NetworkGraphDTO.decode(envelope.data);
       incomingSpan.innerHTML = network_graph;
       console.log(network_graph);
     })
     .catch((error) => {
        incomingSpan.innerHTML = error;
     });
}

function sendMessage() {
  //if the socket's open, send a message:
  if (socket.readyState === WebSocket.OPEN) {
    socket.send(outgoingText.value);
  }
}

let textEncoder = new TextEncoder();

console.log(Envelope.decode((new Envelope("ENVELOPE_TYPE", textEncoder.encode("ENVELOPE_DATA"))).encode()));

console.log(TimeIntervalDTO.decode((new TimeIntervalDTO(0, 100)).encode()))

console.log(GraphNodeDTO.decode((new GraphNodeDTO("GRAPH_NODE")).encode()));
console.log(GraphEdgeDTO.decode((new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")).encode()));
console.log(NetworkGraphDTO.decode((new NetworkGraphDTO([new GraphNodeDTO("GRAPH_FIRST_NODE"), new GraphNodeDTO("GRAPH_SECOND_NODE")], [new GraphEdgeDTO("GRAPH_FIRST_NODE", "GRAPH_SECOND_NODE")])).encode()))

// add a listener for the page to load:
window.addEventListener('load', setup);
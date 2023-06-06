@0xfb6bcb9ccef7c37b;

using import "graph_node.capnp".GraphNode;
using import "graph_edge.capnp".GraphEdge;

struct NetworkGraph {
    nodes @0 :List(GraphNode);
    edges @1 :List(GraphEdge);
}
schema_header::{}

type::{
    name: graph_node,
    type: struct,
    fields: {
        address: string,
    },
}

type::{
    name: graph_edge,
    type: struct,
    fields: {
        src_addr: string,
        dst_addr: string
    },
}

type::{
    name: network_graph,
    type: struct,
    fields: {
        graph_nodes: {
            type: list,
            element: {
                type: graph_node
            },
        },
        graph_edges: {
            type: list,
            element: {
                type: graph_edge
            },
        },
    },
}

schema_footer::{}
schema_header::{}

type::{
    name: network_graph,
    type: struct,
    fields: {
        graph_nodes: {
            type: list,
            element: {
                type: struct,
                fields: {
                    address: string,
                },
            },
        },
        graph_edges: {
            type: list,
            element: {
                type: struct,
                fields: {
                    src_addr: string,
                    dst_addr: string
                },
            },
        },
    },
}

schema_footer::{}
import '../../vendor/ion-bundle.js'
import { GraphEdgeDTO } from './graph_edge.js';
import { GraphNodeDTO } from './graph_node.js';

const NetworkGraphDTO = function (graph_nodes, graph_edges) {
    this.graph_nodes = graph_nodes;
    this.graph_edges = graph_edges;
    
    this.encode = function () {
        let writer = ion.makeTextWriter();
        
        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("graph_nodes");
        writer.stepIn(ion.IonTypes.LIST);
        this.graph_nodes.forEach(graph_node => {
            writer.stepIn(ion.IonTypes.STRUCT);
            
            writer.writeFieldName("address");
            writer.writeString(graph_node.address);
            
            writer.stepOut();
        });
        writer.stepOut();

        writer.writeFieldName("graph_edges");
        writer.stepIn(ion.IonTypes.LIST);
        this.graph_edges.forEach(graph_edge => {
            writer.stepIn(ion.IonTypes.STRUCT);
            
            writer.writeFieldName("src_addr");
            writer.writeString(graph_edge.src_addr);
            
            writer.writeFieldName("dst_addr");
            writer.writeString(graph_edge.dst_addr);
            
            writer.stepOut();
        });
        writer.stepOut();

        writer.stepOut();
        writer.close();
        
        return writer.getBytes();
    }
    
    return this;
}

NetworkGraphDTO.decode = function (data) {
    let reader = ion.makeReader(data);
    
    reader.next();
    reader.stepIn();

    reader.next();
    let graph_nodes = [];
    reader.stepIn();
    while (reader.next()) {
        reader.stepIn();
        reader.next();
        let address = reader.stringValue();
        graph_nodes.push(new GraphNodeDTO(address));
        reader.stepOut();
    }
    reader.stepOut();

    reader.next();
    let graph_edges = [];
    reader.stepIn();
    while (reader.next()) {
        reader.stepIn();
        reader.next();
        let src_addr = reader.stringValue();
        reader.next();
        let dst_addr = reader.stringValue();
        graph_edges.push(new GraphEdgeDTO(src_addr, dst_addr));
        reader.stepOut();
    }
    reader.stepOut();

    return new NetworkGraphDTO(graph_nodes, graph_edges);
}

export {NetworkGraphDTO}
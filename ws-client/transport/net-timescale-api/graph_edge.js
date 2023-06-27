import '../../vendor/ion-bundle.js'

const GraphEdgeDTO = function(src_addr, dst_addr) {
    this.src_addr = src_addr;
    this.dst_addr = dst_addr;
    
    this.encode = function () {
        let writer = ion.makeTextWriter();
        
        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("src_addr");
        writer.writeString(this.src_addr);
        
        writer.writeFieldName("dst_addr");
        writer.writeString(this.dst_addr);
        
        writer.stepOut();
        writer.close();
        
        return writer.getBytes();
    }
    
    return this;
}
    
GraphEdgeDTO.decode = function (data) {
    let reader = ion.makeReader(data);

    reader.next();
    reader.stepIn();

    reader.next();
    let src_addr = reader.stringValue();

    reader.next();
    let dst_addr = reader.stringValue();

    return new GraphEdgeDTO(src_addr, dst_addr);
}

export {GraphEdgeDTO}
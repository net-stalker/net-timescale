import '../../vendor/ion-bundle.js'

const GraphNodeDTO = function (address) {
    this.address = address;
    
    this.encode = function() {
        let writer = ion.makeTextWriter();
        
        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("address");
        writer.writeString(this.address);
        
        writer.stepOut();
        writer.close();
        
        return writer.getBytes();
    }
    
    return this;
}

GraphNodeDTO.decode = function (data) {
    let reader = ion.makeReader(data);

    reader.next();
    reader.stepIn();
    
    reader.next();
    let address = reader.stringValue();

    return new GraphNodeDTO(address);
}

export {GraphNodeDTO}
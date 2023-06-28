import '../../vendor/ion-bundle.js'

const IsRealtimeDTO = function (is_realtime) {
    this.is_realtime = is_realtime;
    
    this.encode = function() {
        let writer = ion.makeTextWriter();
        
        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("is_realtime");
        writer.writeBoolean(this.is_realtime);
        
        writer.stepOut();
        writer.close();
        
        return writer.getBytes();
    }
    
    return this;
}

IsRealtimeDTO.decode = function (data) {
    let reader = ion.makeReader(data);

    reader.next();
    reader.stepIn();
    
    reader.next();
    let is_realtime = reader.booleanValue();

    return new IsRealtimeDTO(is_realtime);
}

export {IsRealtimeDTO}
import '../../vendor/ion-bundle.js'

const TimeIntervalDTO = function(start_date_time, end_date_time, is_realtime) {
    this.start_date_time = start_date_time;
    this.end_date_time = end_date_time;
    this.is_realtime = is_realtime;

    
    this.encode = function() {
        let writer = ion.makeTextWriter();

        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("start_date_time");
        writer.writeInt(this.start_date_time);

        writer.writeFieldName("end_date_time");
        writer.writeInt(this.end_date_time);
        
        writer.writeFieldName("is_realtime");
        writer.writeBoolean(this.is_realtime);

        writer.stepOut();
        writer.close();
        
        return writer.getBytes();
    }

    return this;
}

TimeIntervalDTO.decode = function (data) {
    let reader = ion.makeReader(data);

    reader.next();
    reader.stepIn();

    reader.next();
    let start_date_time = reader.bigIntValue();

    reader.next();
    let end_date_time = reader.bigIntValue();

    reader.next();
    let is_realtime = reader.booleanValue();

    return new TimeIntervalDTO(start_date_time, end_date_time, is_realtime);
}

export {TimeIntervalDTO}
import '../../vendor/ion-bundle.js'

const TimeIntervalDTO = function(start_date_time, end_date_time) {
    this.start_date_time = start_date_time;
    this.end_date_time = end_date_time;

    
    this.encode = function() {
        let writer = ion.makeTextWriter();

        writer.stepIn(ion.IonTypes.STRUCT);
        
        writer.writeFieldName("start_date_time");
        writer.writeInt(this.start_date_time);

        writer.writeFieldName("end_date_time");
        writer.writeInt(this.end_date_time);
        
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

    return new TimeIntervalDTO(start_date_time, end_date_time);
}

export {TimeIntervalDTO}
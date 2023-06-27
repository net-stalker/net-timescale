// import 'https://amazon-ion.github.io/ion-js/browser/scripts/ion-bundle.js';
import '../../vendor/ion-bundle.js'

const Envelope = function(type, data) {
    this.type = type;
    this.data = data;

    this.encode = function () {
        let writer = ion.makeTextWriter();

        writer.stepIn(ion.IonTypes.STRUCT);
    
        writer.writeFieldName("type");
        writer.writeString(this.type);
    
        writer.writeFieldName("data");
        writer.writeBlob(this.data);
    
        writer.stepOut();
        writer.close();
    
        return writer.getBytes();
    }

    return this;
}

Envelope.decode = function (data) {
    let reader = ion.makeReader(data);

    reader.next();
    reader.stepIn();

    reader.next();
    let envelope_type = reader.stringValue();

    reader.next();
    let envelope_data = reader.uInt8ArrayValue();

    return new Envelope(envelope_type, envelope_data);
}

export {Envelope}
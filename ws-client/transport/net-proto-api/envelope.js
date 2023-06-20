// import 'https://amazon-ion.github.io/ion-js/browser/scripts/ion-bundle.js';
import '../ion-bundle.js'

class Envelope {
    type;
    data;

    constructor(type, data) {
        this.type = type;
        this.data = data;
    }

    encode() {
        let writer = ion.makeBinaryWriter();

        writer.stepIn(ion.IonTypes.STRUCT);

        writer.writeFieldName("type");
        writer.writeString(this.type);

        writer.writeFieldName("data");
        writer.writeBlob(this.data);

        writer.stepOut();
        writer.close();

        return writer.getBytes();
    }

    static decode(data) {
        let reader = ion.makeReader(data);

        reader.next();
        reader.stepIn();

        reader.next();
        let envelope_type = reader.stringValue();

        reader.next();
        let envelope_data = reader.uInt8ArrayValue();

        return new Envelope(envelope_type, envelope_data);
    }
}

export {Envelope}
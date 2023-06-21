import '../../vendor/ion-bundle.js'

class GraphNodeDTO {
    address;

    constructor(address) {
        this.address = address;
    }

    encode() {
        let writer = ion.makeTextWriter();

        writer.stepIn(ion.IonTypes.STRUCT);

        writer.writeFieldName("address");
        writer.writeString(this.address);

        writer.stepOut();
        writer.close();

        return writer.getBytes();
    }

    static decode(data) {
        let reader = ion.makeReader(data);

        reader.next();
        reader.stepIn();

        reader.next();
        let address = reader.stringValue();

        return new GraphNodeDTO(address);
    }
}

export {GraphNodeDTO}
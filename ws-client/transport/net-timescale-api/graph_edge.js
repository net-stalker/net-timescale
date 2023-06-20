import '../ion-bundle.js'

class GraphEdgeDTO {
    src_addr;
    dst_addr;

    constructor(src_addr, dst_addr) {
        this.src_addr = src_addr;
        this.dst_addr = dst_addr;
    }

    encode() {
        let writer = ion.makeBinaryWriter();

        writer.stepIn(ion.IonTypes.STRUCT);

        writer.writeFieldName("src_addr");
        writer.writeString(this.src_addr);

        writer.writeFieldName("dst_addr");
        writer.writeString(this.dst_addr);

        writer.stepOut();
        writer.close();

        return writer.getBytes();
    }

    static decode(data) {
        let reader = ion.makeReader(data);

        reader.next();
        reader.stepIn();

        reader.next();
        let src_addr = reader.stringValue();

        reader.next();
        let dst_addr = reader.stringValue();

        return new GraphEdgeDTO(src_addr, dst_addr);
    }
}

export {GraphEdgeDTO}
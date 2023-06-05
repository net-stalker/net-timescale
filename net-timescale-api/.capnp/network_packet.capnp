@0x8407268e65d4ceb4;

struct NetworkPacket {
    frameTime @0:Int64; #The time when the data was captured
    
    srcAddr @1 :Text;   #Source ip address which is taken from a pcap file
    dstAddr @2 :Text;   #Destination ip address which is taken from a pcap file
    data @3 :Data;      #The rest of the data from pcap converted to json
}
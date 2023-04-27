@0x8407268e65d4ceb4;

struct DataToSend {
    requirement @0 :Data;
    #In other words it is an API provided by all the microservice 

    
    frameTime @1 :Int64;
    #The time when the data was captured

    srcAddr @2 :Text;
    #Source ip address which is taken from a pcap file
    dstAddr @3 :Text;
    #Destination ip address which is taken from a pcap file

    json @4 :Data;
    #The rest of the data from pcap converted to json
}
import net from 'net';
import { toLE, formatIPv6, toBE, calculateChecksum } from './helpers';

const prepareVersionMessage = (ip: string): string => {
    let unixTimeSeconds = Math.floor(Date.now() / 1000);

    let protocolVersion = toLE(70015, 4);
    let services = "0000000000000000";  
    let time = toLE(unixTimeSeconds, 8);
    let remoteServices = "0000000000000000";
    let remoteIP = formatIPv6(ip);
    let remotePort = toBE(8333, 2);
    let localServices = "0000000000000000";
    let localIP = formatIPv6('127.0.0.1');
    let localPort = toBE(8333, 2);
    let nonce = "0000000000000000";  
    let userAgentBytes = "0f"
    let userAgent = "2f5361746f7368693a302e392e332f"; 
    let lastBlock = toLE(0, 4);
    let relay = "01";

    let payload = (
        protocolVersion +
        services +
        time +
        remoteServices +
        remoteIP +
        remotePort +
        localServices +
        localIP +
        localPort +
        nonce +
        userAgentBytes +
        userAgent +
        lastBlock +
        relay           
    ).toLowerCase();

    let magic = "f9beb4d9"; 
    let command = "76657273696f6e0000000000"; 
    let length = toLE(payload.length / 2, 4);  
    let checksum = calculateChecksum(payload);

    let versionMessage = magic + command + length + checksum + payload;
    return versionMessage;
};

export const establishTCP = (ip: string): void => {
    const versionMessage = prepareVersionMessage(ip);
    let client = new net.Socket();
    let handshakeComplete = false;
    
    console.log(`Performing handshake for ${ip}`);

    const timeout = setTimeout(() => {
        if (!handshakeComplete) {
            console.log("Handshake timed out");
            client.destroy();
        }
    }, 10000);

    client.connect({ port: 8333, host: ip }, () => {
        const messageBuffer = Buffer.from(versionMessage, 'hex');
        client.write(messageBuffer);
        console.log("→ Sending: VERSION");
    });

    let receivedVersion = false;
    let receivedVerack = false;

    client.on("data", (data) => {
        const hex = data.toString("hex");
        const messages = hex.split("f9beb4d9").filter(msg => msg.length > 0);
        
        messages.forEach(message => {
            if (message.includes("76657273696f6e")) {  // version
                receivedVersion = true;
                console.log("← Received: VERSION from node");
                console.log("→ Sending: VERACK");
                const verackMessage = "f9beb4d976657261636b000000000000000000005df6e0e2";
                client.write(Buffer.from(verackMessage, 'hex'));
            }
            else if (message.includes("76657261636b")) {  // verack
                receivedVerack = true;
                console.log("← Received: VERACK from node");
                console.log("✓ Handshake completed!");
                handshakeComplete = true;
                clearTimeout(timeout);
                client.end();
            }
        });
    });

    client.on("error", () => {
        if (!handshakeComplete) {
            console.log("Failed to connect to node");
        }
        clearTimeout(timeout);
    });

    client.on("close", () => {
        // Only log if we haven't completed the handshake and haven't shown an error
        if (!handshakeComplete && !receivedVersion) {
            console.log("Connection closed before handshake completion");
        }
    });
}
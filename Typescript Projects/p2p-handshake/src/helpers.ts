import { promises as dns } from "dns";
import { toBufferBE, toBufferLE } from "bigint-buffer";
import crypto from "crypto";

export const performLookup = async (number: number): Promise<void> => {
  try {
    const results = await dns.lookup("seed.bitcoin.sipa.be", { all: true });
    const fixedNumberOfResults = results.slice(0, number);
    fixedNumberOfResults.forEach((result, index) => {
      console.log(`Result ${index + 1}: Address: ${result.address}`);
    });
  } catch (err) {
    console.error("Error during lookup:", err);
  }
};

export const toLE = (number: number, byteNumber: number): string => {
  const bigIntValue = BigInt(number);
  const buffer = toBufferLE(bigIntValue, byteNumber);
  return buffer.toString("hex").toUpperCase();
};

export const toBE = (number: number, byteNumber: number): string => {
  const bigIntValue = BigInt(number);
  const buffer = toBufferBE(bigIntValue, byteNumber);
  return buffer.toString("hex").toUpperCase();
};

export const formatIPv6 = (ipv4: string): string => {
  const ipv4Parts = ipv4.split(".").map((part) => parseInt(part));
  const prefix = "00000000000000000000FFFF";
  const ipv4Hex = ipv4Parts
    .map((part) => part.toString(16).padStart(2, "0"))
    .join("");
  return prefix + ipv4Hex.toUpperCase();
};

export const calculateChecksum = (payload: string): string => {
  const buffer = Buffer.from(payload, "hex");
  const firstHash = crypto.createHash("sha256").update(buffer).digest();
  const secondHash = crypto.createHash("sha256").update(firstHash).digest();
  return secondHash.subarray(0, 4).toString("hex");
};

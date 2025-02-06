import { Command } from "commander";
import { performLookup } from "./helpers";
import { establishTCP } from "./handshake";

const program = new Command();

program
  .name("p2p-handshake")
  .description("A CLI p2p handshake app")
  .version("1.0.0");

const ips = new Command("ips")
  .description("Gets a selectable number of bitcoin nodes ips")
  .alias("i")
  .argument("<number>", "Number")
  .action(async (number) => {
    performLookup(number);
});

const handshake = new Command("handshake")
  .description("Performs the p2p handshake")
  .alias("f")
  .argument("<ip>", "Ip of the bitcoin node for which to establish the handshake")
  .action((ip) => {
    establishTCP(ip);
  });

program.addCommand(ips);
program.addCommand(handshake);

export const p2pProgram = program;
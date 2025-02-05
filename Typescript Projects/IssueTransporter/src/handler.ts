import { Command } from "commander";
import { transferIssue, getIssueNumbers } from "./index";

const program = new Command();

program
  .name("issue-transfer")
  .description("A CLI tool to transfer GitHub issues between repositories")
  .version("1.0.0");

// Store state
let owner = '';
let sourceRepo = '';

// Create owner command
const ownerCommand = program
  .command('owner')
  .description('Set the owner of the repositories')
  .argument('<owner-name>', 'GitHub username or organization name');

// Create nested repo-from command
ownerCommand
  .command('repo-from')
  .description('Set the source repository')
  .argument('<repo-name>', 'Source repository name')
  .action((repoName, options, command) => {
    if (!owner) {
      console.error('Please set owner first');
      return;
    }
    // Implementation will go here
  });

// Create nested repo-to command
ownerCommand
  .command('repo-to')
  .description('Set the target repository and transfer issues')
  .argument('<repo-name>', 'Target repository name')
  .option('-s, --select <numbers>', 'Specific issue numbers to transfer (comma-separated)')
  .action((repoName, options, command) => {
    if (!owner || !sourceRepo) {
      console.error('Please set owner and source repository first');
      return;
    }
    // Implementation will go here
  });

// Set owner action
ownerCommand.action((ownerName) => {
  owner = ownerName;
  console.log(`Owner set to: ${ownerName}`);
});

program.parse();
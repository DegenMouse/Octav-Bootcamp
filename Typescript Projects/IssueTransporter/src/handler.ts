import { Command } from "commander";
import { getIssueNumbers } from "./helper";
import { transferIssue } from "./helper";

const program = new Command();

// Store state
let owner = "";
let sourceRepo = "";
let targetRepo = "";

program
  .name("issue-transfer")
  .description("A CLI tool to transfer GitHub issues between repositories")
  .version("1.0.0");

program
  .command('transfer')
  .description('Transfer issues between repositories')
  .argument('<owner>', 'GitHub username or organization')
  .argument('<source-repo>', 'Source repository name')
  .argument('<target-repo>', 'Target repository name')
  .option('-a, --all', 'Transfer all issues (both open and closed)')
  .action(async (ownerArg, sourceRepoArg, targetRepoArg, options) => {
    owner = ownerArg;
    sourceRepo = `${owner}/${sourceRepoArg}`;
    targetRepo = `${owner}/${targetRepoArg}`;
    
    try {
      const allIssues = await getIssueNumbers(sourceRepo);
      if (allIssues.length === 0) {
        console.log('No issues found to transfer');
        return;
      }

      // Filter issues based on options
      let issuesToTransfer = allIssues;
      if (!options.all) {
        issuesToTransfer = allIssues.filter(issue => issue.state === 'OPEN');
        console.log('Transferring open issues (use --all to transfer both open and closed issues)');
      } else {
        console.log('Transferring all issues (both open and closed)...');
      }

      console.log(`Found ${issuesToTransfer.length} issues to transfer`);
      
      const {succesfull, unsuccesfull} = await transferIssue(sourceRepo, issuesToTransfer, targetRepo);
      succesfull.length > 0 && console.log('Successfully transferred issues:', succesfull);
      unsuccesfull.length > 0 && console.log('Unsuccessfully transferred issues:', unsuccesfull);
    } catch (error: any) {
      console.error('Error:', error.message);
      process.exit(1);
    }
  });

export async function parseAndHandle() {
  await program.parseAsync(process.argv);
}
